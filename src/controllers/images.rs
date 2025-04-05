#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::{db, prelude::*};

use crate::domain::domain_services::image_generation::ImageGenerationService;
use crate::models::_entities::sea_orm_active_enums::{
    BasedOn, Ethnicity, EyeColor, ImageFormat, ImageSize, Sex, Status,
};
use crate::models::_entities::training_models;
use crate::models::images::{AltText, ImageNew, ImageNewList, UserPrompt};
use crate::models::user_credits::UserCreditsClient;
use crate::models::{ImageModel, TrainingModelModel, UserCreditModel, UserModel};
use crate::views::images::{CreditsViewModel, ImageViewModel};
use crate::{
    domain::image::Image,
    models::_entities::images::ActiveModel,
    models::_entities::images::{Entity, Model},
    service::fal_ai::fal_client::{FalAiClient, FluxLoraImageGenerate},
    views,
};
use axum::{debug_handler, Extension};
use axum::{http::HeaderMap, http::StatusCode, response::IntoResponse, Json};
use derive_more::{AsRef, Constructor, Display};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use training_models::Model as TrainingModel;
use uuid::Uuid;

#[derive(Clone, Validate, Debug, Deserialize)]
pub struct ImageGenRequestParams {
    pub training_model_id: i32,
    pub prompt: UserPrompt,
    pub image_size: ImageSize,
    #[validate(range(min = 1, max = 50, message = "Creative must be between 1 and 50"))]
    pub num_inference_steps: u8,
    #[validate(range(
        min = 1,
        max = 20,
        message = "Number of images must be between 1 and 20"
    ))]
    pub num_images: u8,
}
impl ImageGenRequestParams {
    pub fn process(self, model: &TrainingModelModel) -> ImageNewList {
        let sys_prompt = self.prompt.formatted_prompt(model);
        let alt = AltText::from(&self.prompt);
        (0..self.num_images)
            .map(|_| ImageNew {
                pid: Uuid::new_v4(),
                user_id: model.user_id,
                training_model_id: self.training_model_id,
                pack_id: None,
                sys_prompt: sys_prompt.to_owned(),
                user_prompt: self.prompt.to_owned(),
                alt: alt.to_owned(),
                num_inference_steps: self.num_inference_steps as i32,
                content_type: ImageFormat::Jpeg,
                status: Status::Pending,
                image_size: self.image_size,
                fal_ai_request_id: None,
                width: None,
                height: None,
                image_url: None,
                image_url_s3: None,
                is_favorite: false,
                deleted_at: None,
            })
            .collect::<Vec<ImageNew>>()
            .into()
    }
}

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Images;
    impl Images {
        pub const BASE: &'static str = "/api/images";
        pub const IMAGE: &'static str = "/";
        pub const IMAGE_GENERATE: &'static str = "/generate";
        pub const IMAGE_GENERATE_TEST: &'static str = "/generate/test";
        pub const IMAGE_CHECK_TEST: &'static str = "/check/test/{id}";
        pub const IMAGE_CHECK_ID: &'static str = "/check/{id}";
        pub const IMAGE_CHECK: &'static str = "/check";
        pub const IMAGE_ID: &'static str = "/{id}";
        pub const IMAGE_BASE: &'static str = "";

        pub fn check_route() -> String {
            use crate::controllers::images;

            let check_route = format!(
                "{}{}/test",
                images::routes::Images::BASE,
                images::routes::Images::IMAGE_CHECK
            );
            check_route
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Images::BASE)
        .add(routes::Images::IMAGE, get(list))
        // .add(routes::Images::IMAGE, post(add))
        .add(routes::Images::IMAGE_GENERATE_TEST, post(generate_test))
        // .add(routes::Images::IMAGE_GENERATE, post(generate_img))
        .add(routes::Images::IMAGE_CHECK_TEST, get(check_test))
        .add(routes::Images::IMAGE_CHECK_ID, get(check_img))
        .add(routes::Images::IMAGE_ID, get(get_one))
        .add(routes::Images::IMAGE_ID, delete(remove))
    // .add(routes::Images::IMAGE_ID, put(update))
    // .add(routes::Images::IMAGE_ID, patch(update))
}

async fn load_user(db: &DatabaseConnection, pid: &str) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, pid).await?;
    Ok(item)
}
async fn load_item(ctx: &AppContext, id: i32) -> Result<ImageModel> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}
async fn load_item_pid(ctx: &AppContext, id: Uuid) -> Result<ImageModel> {
    let item = ImageModel::find_by_pid(&ctx.db, &id).await?;
    Ok(item)
}
async fn load_credits(db: &DatabaseConnection, id: i32) -> Result<UserCreditModel> {
    let credits = UserCreditModel::find_by_user_id(db, id).await?;
    Ok(credits)
}

#[debug_handler]
pub async fn check_test(
    auth: auth::JWT,
    Path(pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    use rand::Rng;
    let mut image = load_item_pid(&ctx, pid).await?;
    let user: crate::models::users::Model = load_user(&ctx.db, &auth.claims.pid).await?;

    if image.user_id != user.id {
        return Err(Error::Unauthorized("Unauthorized".to_string()));
    }

    let user_credits = load_credits(&ctx.db, user.id).await?;
    let user_credits_view: CreditsViewModel = user_credits.into();

    let change = rand::rng().random_range(0..=10);
    let num = rand::rng().random_range(1..=11);
    if change == 0 {
        let user_credits = image.image_url = Some(format!(
            "https://flowbite.s3.amazonaws.com/docs/gallery/square/image-{}.jpg",
            num
        ));
        image.status = Status::Completed;
        let check_route = routes::Images::check_route();
        return views::images::img_completed(
            &v,
            &vec![image.into()],
            check_route.as_str(),
            &user_credits_view,
        );
    }
    Ok((StatusCode::NO_CONTENT).into_response())
}

#[debug_handler]
pub async fn check_img(
    auth: auth::JWT,
    Path(pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    use rand::Rng;
    let image = load_item_pid(&ctx, pid).await?;
    let user = load_user(&ctx.db, &auth.claims.pid).await?;

    if image.user_id != user.id {
        return Err(Error::Unauthorized("Unauthorized".to_string()));
    }

    if image.status == Status::Completed {
        let user_credits = load_credits(&ctx.db, user.id).await?;
        let user_credits_view: CreditsViewModel = user_credits.into();
        let check_route = routes::Images::check_route();
        return views::images::img_completed(
            &v,
            &vec![image.into()],
            check_route.as_str(),
            &user_credits_view,
        );
    }
    Ok((StatusCode::NO_CONTENT).into_response())
}

#[debug_handler]
pub async fn generate_test(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(request): Json<ImageGenRequestParams>,
) -> Result<Response> {
    // 1. Validate request payload format
    request.validate()?;

    // 2. Call the Domain Service to perform the core logic
    let (updated_credits, saved_images) =
        ImageGenerationService::generate(&ctx, &fal_ai_client, &auth.claims.pid, request).await?;

    // 3. Prepare Data for the View using safe View Models
    let credits_view_model = CreditsViewModel::from(&updated_credits);
    let image_view_models: Vec<ImageViewModel> = saved_images.into();

    // 4. Prepare other view-specific data
    let check_route = format!(
        "{}{}/test",
        routes::Images::BASE,
        routes::Images::IMAGE_CHECK
    );

    // 5. Render the view using the View Models
    views::images::img_completed(&v, &image_view_models, &check_route, &credits_view_model)
}

#[debug_handler]
pub async fn get_one(
    auth: auth::JWT,
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let user = UserModel::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let image = load_item_pid(&ctx, id).await?;
    if image.user_id != user.id {
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    views::images::show(&v, &image)
}
#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}
#[debug_handler]
pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    // --- Load Entities ---
    let user = UserModel::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let img = load_item(&ctx, user.id).await?;
    if user.id == img.user_id {
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    img.delete(&ctx.db).await?;
    format::empty()
}
