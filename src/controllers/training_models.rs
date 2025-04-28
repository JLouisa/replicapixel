#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::domain::response::{handle_general_response, handle_general_response_text};
use crate::domain::website::Website;
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::_entities::training_models::{ActiveModel, Entity, Model};
use crate::models::join::user_credits_models::load_user_and_training;
use crate::models::training_models::{TrainingForm, TrainingModelParams};
use crate::models::users::UserPid;
use crate::models::{TrainingModelActiveModel, TrainingModelModel, UserModel};
use crate::service::aws::s3::{AwsS3, PresignedUrlRequest, PresignedUrlSafe, S3Key};
use crate::service::fal_ai::fal_client::{FalAiClient, FluxLoraTrainingSchema};
use crate::views;
use crate::views::training_models::TrainingModelView;
use axum::{debug_handler, http::StatusCode, response::IntoResponse, Extension, Json};
use loco_rs::prelude::*;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct TrainingModelRoutes {
        pub base: String,
        pub check_id_status: String,
        pub upload: String,
    }
    impl TrainingModelRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(TrainingModel::BASE),
                check_id_status: format!("{}{}", TrainingModel::BASE, TrainingModel::CHECK),
                upload: format!(
                    "{}{}",
                    TrainingModel::BASE,
                    TrainingModel::TRAINING_MODELS_UPLOAD
                ),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct TrainingModel;
    impl TrainingModel {
        pub const BASE: &'static str = "/api/models";
        pub const CHECK_ID_STATUS: &'static str = "/check/{id}/{status}";
        pub const CHECK: &'static str = "/check";
        pub const TRAINING_COMPLETED_ID: &'static str = "/upload/completed/{id}";
        pub const TRAINING_MODELS_UPLOAD: &'static str = "/upload";
        pub const TRAINING_MODELS_ID: &'static str = "/{id}";
        pub const TRAINING_MODELS: &'static str = "/";
        pub const TRAINING_MODELS_BASE: &'static str = "";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::TrainingModel::BASE)
        .add(routes::TrainingModel::TRAINING_MODELS, get(list))
        .add(routes::TrainingModel::TRAINING_MODELS, post(add))
        .add(routes::TrainingModel::CHECK_ID_STATUS, get(check_model))
        .add(routes::TrainingModel::TRAINING_MODELS_ID, get(get_one))
        .add(routes::TrainingModel::TRAINING_MODELS_ID, delete(remove))
        .add(routes::TrainingModel::TRAINING_MODELS_ID, put(update))
        .add(routes::TrainingModel::TRAINING_MODELS_ID, patch(update))
        .add(
            routes::TrainingModel::TRAINING_MODELS_UPLOAD,
            post(upload_training),
        )
        .add(
            routes::TrainingModel::TRAINING_COMPLETED_ID,
            patch(upload_training_completed),
        )
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

// async fn load_item_all(ctx: &AppContext, id: i32) -> Result<Model> {
//     let list = TrainingModelModel::find_all_by_user_id(&ctx.db, id).await?;
//     let item = Entity::find_by_id(id).one(&ctx.db).await?;
//     item.ok_or_else(|| Error::NotFound)
// }

#[debug_handler]
pub async fn upload_training(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Json(form): Json<TrainingForm>,
) -> Result<impl IntoResponse> {
    let user = UserModel::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    //1. Generate Pre-Signed URL
    let pre_url_request: PresignedUrlRequest = form.clone().into();
    let (pre_url, s3_key) = s3_client
        .presigned_save_url(&user.pid, &pre_url_request, None)
        .await
        .map_err(|_| loco_rs::Error::Message(String::from("Generating Pre-sign URL error: 101")))?;

    //2. Create and save Training Model in Database
    let training_params: TrainingModelParams = form.from_form(&user, &s3_key);
    TrainingModelActiveModel::save(&ctx.db, &training_params).await?;

    //3. Create Pre-Signed URL
    let pre_sign_response = PresignedUrlSafe::from_request(pre_url_request, pre_url);
    Ok(handle_general_response(
        StatusCode::OK,
        Some(pre_sign_response),
        Some("PreSign Successfully".to_string()),
    )
    .into_response())
}

#[debug_handler]
pub async fn upload_training_completed(
    auth: auth::JWT,
    Path(training_model_id): Path<Uuid>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = UserModel::find_by_pid(&ctx.db, &auth.claims.pid)
        .await
        .map_err(|e| loco_rs::Error::Message(format!("Error Getting user: {} ", e)))?;
    let train = TrainingModelModel::find_by_pid(&ctx.db, &training_model_id)
        .await
        .map_err(|e| loco_rs::Error::Message(format!("Error Getting Training Model: {} ", e)))?;
    let s3_key: S3Key = S3Key::new(&train.s3_key);

    let exists = s3_client
        .check_object_exists(&s3_key)
        .await
        .map_err(|_| loco_rs::Error::Message(String::from("Error checking storage: 101")))?;

    if !exists {
        return Ok(handle_general_response_text(
            StatusCode::NOT_FOUND,
            Some(format!("Path: {}", s3_key.as_ref())),
            Some("File not found".into()),
        )
        .into_response());
    }
    let train = ActiveModel::from(train).upload_completed(&ctx.db).await?;

    let exp_time = Some(60 * 60 * 3); // 3 hours
    let pre_url = match s3_client.get_object_pre(&s3_key, exp_time).await {
        Ok(url) => url,
        Err(e) => return Ok((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };
    let mut train_schema = FluxLoraTrainingSchema::from_training(train.clone(), pre_url);

    // Send training model to Fal AI Queue
    let queue = fal_ai_client
        .send_training_queue_test(&train_schema)
        .await?;

    // Save Fal AI response in Database
    let _train_model = ActiveModel::from(train)
        .update_with_fal_ai_response(&ctx.db, &queue)
        .await?;

    // Send response back to user
    Ok(handle_general_response(
        StatusCode::OK,
        Some(queue),
        Some("Successfully saved".into()),
    )
    .into_response())

    // // Send response back to user
    // Ok(
    //     handle_general_response_text(StatusCode::OK, None, Some("Successfully saved".into()))
    //         .into_response(),
    // )
}

#[debug_handler]
pub async fn check_model(
    Path((id, status)): Path<(i32, Status)>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let model = load_item(&ctx, id).await?;
    // Check if status matches
    if status == model.training_status {
        return Ok(StatusCode::NO_CONTENT.into_response());
    }
    let model_view: TrainingModelView = model.into();
    views::training_models::training_models_update(v, &website, &model_view)
}

//? ====== REST API ======
#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<TrainingModelParams>,
) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<TrainingModelParams>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}
