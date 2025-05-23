#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query, Extension};
use axum::{http::StatusCode, response::IntoResponse, Json};
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::domain_services::image_generation::ImageGenerationService;
use crate::domain::website::Website;
use crate::models::_entities::sea_orm_active_enums::{ImageFormat, ImageSize, Status};
use crate::models::images::{
    AltText, ImageNew, ImageNewList, ImagesModelList, SysPrompt, UserPrompt,
};
use crate::models::join::user_credits_models::load_user_and_one_training_model;
use crate::models::join::user_image::load_user_and_image;
use crate::models::packs::PacksDomain;
use crate::models::users::UserPid;
use crate::models::{ImageActiveModel, ImageModel, TrainingModelModel, UserCreditModel, UserModel};
use crate::service::aws::s3::{AwsS3, S3Folders};
use crate::service::fal_ai::fal_client::Lora;
use crate::service::redis::redis::RedisCacheDriver;
use crate::views::images::{CreditsViewModel, ImageView, ImageViewList};
use crate::{models::_entities::images::Entity, service::fal_ai::fal_client::FalAiClient, views};

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ImageRoutes {
        pub base: String,
        pub check: String,
        pub image_restore: String,
        pub image_favorite: String,
        pub api_image_infinite: String,
        pub api_image_s3_complete_upload: String,
    }
    impl ImageRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Images::BASE),
                check: format!("{}{}", Images::BASE, Images::IMAGE_CHECK),
                image_restore: format!("{}{}", Images::BASE, Images::IMAGE_RESTORE),
                image_favorite: format!("{}{}", Images::BASE, Images::IMAGE_FAVORITE),
                api_image_infinite: format!("{}{}", Images::BASE, Images::IMAGE_INFINITE),
                api_image_s3_complete_upload: format!(
                    "{}{}",
                    Images::BASE,
                    Images::IMAGE_S3_UPLOAD_COMPLETE
                ),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Images;
    impl Images {
        pub const BASE: &'static str = "/api/images";
        pub const IMAGE: &'static str = "/";
        pub const IMAGE_S3_UPLOAD_COMPLETE_ID: &'static str = "/upload/complete/{id}";
        pub const IMAGE_S3_UPLOAD_COMPLETE: &'static str = "/upload/complete";
        pub const IMAGE_INFINITE_ID: &'static str = "/infinite/{id}";
        pub const IMAGE_INFINITE: &'static str = "/infinite";
        pub const IMAGE_GENERATE_TEST: &'static str = "/generate/test";
        pub const IMAGE_GENERATE: &'static str = "/generate";
        pub const IMAGE_CHECK_TEST_ID: &'static str = "/check/test/{id}";
        pub const IMAGE_CHECK_TEST: &'static str = "/check/test";
        pub const IMAGE_CHECK_ID: &'static str = "/check/{id}";
        pub const IMAGE_CHECK: &'static str = "/check";
        pub const IMAGE_ID: &'static str = "/{id}";
        pub const IMAGE_RESTORE_ID: &'static str = "/restore/{id}";
        pub const IMAGE_RESTORE: &'static str = "/restore";
        pub const IMAGE_FAVORITE_ID: &'static str = "/favorite/{id}";
        pub const IMAGE_FAVORITE: &'static str = "/favorite";
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
        .add(routes::Images::IMAGE_GENERATE_TEST, post(generate))
        .add(routes::Images::IMAGE_CHECK_ID, get(check_img))
        .add(routes::Images::IMAGE_ID, get(get_one))
        .add(routes::Images::IMAGE_ID, delete(remove))
        .add(routes::Images::IMAGE_RESTORE_ID, delete(restore))
        .add(routes::Images::IMAGE_FAVORITE_ID, patch(favorite_toggle))
        .add(
            routes::Images::IMAGE_INFINITE_ID,
            get(image_infinite_handler),
        )
        .add(
            routes::Images::IMAGE_S3_UPLOAD_COMPLETE_ID,
            patch(img_s3_upload_completed),
        )
}

#[derive(Clone, Validate, Debug, Deserialize)]
pub struct ImageGenRequestParams {
    pub training_model_pid: Option<Uuid>,
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

pub trait ImageGenerationTrait {
    fn process(self, model: &Option<TrainingModelModel>, user_pid: &UserModel) -> ImageNewList;
    fn formatted_prompt(&self, model: &TrainingModelModel) -> UserPrompt;
    fn steps(&self) -> i32;
    fn num_images(&self) -> i32;
    fn image_size(&self) -> ImageSize;
    fn cost(&self) -> i32;
}
impl ImageGenerationTrait for PacksDomain {
    fn formatted_prompt(&self, _model: &TrainingModelModel) -> UserPrompt {
        let prompt = self.pack_prompts.clone();
        UserPrompt::new(prompt)
    }
    fn steps(&self) -> i32 {
        self.num_inference_steps as i32
    }
    fn num_images(&self) -> i32 {
        self.num_images as i32
    }
    fn image_size(&self) -> ImageSize {
        self.image_size
    }
    fn cost(&self) -> i32 {
        self.credits as i32
    }
    fn process(self, model: &Option<TrainingModelModel>, user: &UserModel) -> ImageNewList {
        let model_id = match model {
            Some(m) => Some(m.id),
            None => None,
        };
        let user_prompt = match model {
            Some(m) => self.formatted_prompt(&m),
            None => UserPrompt::new(self.pack_prompts.clone()),
        };
        let loras = match model {
            Some(m) => match m.tensor_path.clone() {
                Some(p) => vec![Lora {
                    path: p,
                    scale: 1.0,
                }],
                None => vec![],
            },
            None => vec![],
        };

        let alt: AltText = user_prompt.clone().into();
        (0..self.num_images())
            .map(|_| {
                let uuid = Uuid::new_v4();
                let s3_key = AwsS3::init_img_s3_key(&user.pid, &uuid);
                ImageNew {
                    pid: uuid,
                    image_s3_key: s3_key,
                    user_id: user.id,
                    training_model_id: model_id,
                    pack_id: Some(self.id),
                    sys_prompt: SysPrompt::new(user_prompt.as_ref()),
                    user_prompt: user_prompt.to_owned(),
                    alt: alt.to_owned(),
                    loras: loras.clone(),
                    image_size: self.image_size,
                    num_inference_steps: self.num_images() as i32,
                    ..Default::default()
                }
            })
            .collect::<Vec<ImageNew>>()
            .into()
    }
}
impl ImageGenerationTrait for ImageGenRequestParams {
    fn formatted_prompt(&self, _model: &TrainingModelModel) -> UserPrompt {
        self.prompt.clone()
    }
    fn steps(&self) -> i32 {
        self.num_inference_steps as i32
    }
    fn num_images(&self) -> i32 {
        self.num_images as i32
    }
    fn image_size(&self) -> ImageSize {
        self.image_size
    }
    fn cost(&self) -> i32 {
        self.num_images as i32
    }
    fn process(self, model: &Option<TrainingModelModel>, user: &UserModel) -> ImageNewList {
        let model_id = match model {
            Some(m) => Some(m.id),
            None => None,
        };
        let sys_prompt = match model {
            Some(m) => self.prompt.formatted_prompt(m),
            None => SysPrompt::new(self.prompt.as_ref()),
        };
        let loras = match model {
            Some(m) => match m.tensor_path.clone() {
                Some(p) => vec![Lora {
                    path: p,
                    scale: 1.0,
                }],
                None => vec![],
            },
            None => vec![],
        };
        let alt = AltText::truncate_with_ellipsis(sys_prompt.as_ref());
        (0..self.num_images)
            .map(|_| {
                let uuid = Uuid::new_v4();
                let s3_key = AwsS3::init_img_s3_key(&user.pid, &uuid);
                ImageNew {
                    pid: uuid,
                    user_id: user.id,
                    training_model_id: model_id,
                    sys_prompt: sys_prompt.to_owned(),
                    user_prompt: self.prompt.to_owned(),
                    alt: alt.to_owned(),
                    num_inference_steps: self.num_inference_steps as i32,
                    image_s3_key: s3_key,
                    image_size: self.image_size,
                    loras: loras.clone(),
                    ..Default::default()
                }
            })
            .collect::<Vec<ImageNew>>()
            .into()
    }
}

#[derive(Deserialize, Debug)]
pub struct ImageLoadingParams {
    pub deleted: Option<bool>,
    pub favorite: Option<bool>,
}

// async fn load_user(db: &DatabaseConnection, pid: &str) -> Result<UserModel> {
//     let item = UserModel::find_by_pid(db, pid).await?;
//     Ok(item)
// }
// async fn load_item(ctx: &AppContext, id: i32) -> Result<ImageModel> {
//     let item = Entity::find_by_id(id).one(&ctx.db).await?;
//     item.ok_or_else(|| Error::NotFound)
// }
async fn load_item_pid(ctx: &AppContext, id: Uuid) -> Result<ImageModel> {
    let item = ImageModel::find_by_pid(&ctx.db, &id).await?;
    Ok(item)
}
async fn load_credits(db: &DatabaseConnection, id: i32) -> Result<UserCreditModel> {
    let credits = UserCreditModel::find_by_user_id(db, id).await?;
    Ok(credits)
}
async fn load_user(db: &DatabaseConnection, pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, &pid.as_ref().to_string()).await?;
    Ok(item)
}
async fn load_images_inf(
    db: &DatabaseConnection,
    user: &UserModel,
    anchor_image_id: &Uuid,
    params: ImageLoadingParams,
) -> Result<ImagesModelList> {
    let list =
        ImageModel::get_next_20_images_after(db, user.id, anchor_image_id, 20, params).await?;
    Ok(ImagesModelList::new(list))
}

#[debug_handler]
pub async fn generate(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(request): Json<ImageGenRequestParams>,
) -> Result<Response> {
    dbg!(&request);
    // 0. Validate request payload format
    request.validate()?;

    // 1. Load User and Training Model
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, training_model) = match request.training_model_pid {
        Some(pid) => {
            let (user, training_model) =
                load_user_and_one_training_model(&ctx.db, &user_pid, pid).await?;
            (user, Some(training_model))
        }
        None => {
            let user = load_user(&ctx.db, &user_pid).await?;
            (user, None)
        }
    };

    dbg!(&user);
    dbg!(&training_model);

    // 2. Call the Domain Service to perform the core logic
    let (updated_credits, saved_images) =
        ImageGenerationService::generate(&ctx, &fal_ai_client, request, &user, &training_model)
            .await?;
    let is_image_gen = Some(true);
    // 3. Render the view using the View Models
    views::images::img_completed(
        &v,
        &website,
        &saved_images.into(),
        &updated_credits.into(),
        is_image_gen,
    )

    // format::empty()
}

#[debug_handler]
pub async fn img_s3_upload_completed(
    auth: auth::JWT,
    Path(img_pid): Path<Uuid>,
    Extension(s3_client): Extension<AwsS3>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (user, image) = load_user_and_image(&ctx.db, &auth.claims.pid, &img_pid).await?;
    let s3_key = s3_client.create_s3_key(
        &user.pid,
        &S3Folders::Images,
        &image.pid.to_string(),
        &ImageFormat::Jpeg,
    );

    let exists = s3_client
        .check_object_exists(&s3_key)
        .await
        .map_err(|_| loco_rs::Error::Message(String::from("Error checking storage: 101")))?;

    if !exists {
        return Ok((StatusCode::NO_CONTENT).into_response().into_response());
    }

    ImageActiveModel::from(image)
        .upload_s3_completed(&ctx.db)
        .await
        .ok();

    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
pub async fn check_img(
    auth: auth::JWT,
    Path(pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(s3_client): Extension<AwsS3>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let (user, image) = load_user_and_image(&ctx.db, &auth.claims.pid, &pid).await?;

    if image.user_id != user.id {
        return Err(Error::Unauthorized("Unauthorized".to_string()));
    }

    if image.status == Status::Processing {
        let user_credits = load_credits(&ctx.db, user.id).await?;
        let user_credits_view: CreditsViewModel = user_credits.into();
        let image: ImageView = image.into();
        let image: ImageView = image
            .clone()
            .set_pre_url(&s3_client)
            .await
            .unwrap_or_else(|_| image);
        let is_image_gen = Some(true);

        return views::images::img_completed(
            &v,
            &website,
            &ImageViewList::new(vec![image]),
            &user_credits_view,
            is_image_gen,
        );
    }

    Ok((StatusCode::NO_CONTENT).into_response())
}

async fn image_infinite_handler(
    auth: auth::JWT,
    Path(anchor_image_pid): Path<Uuid>,
    Query(params): Query<ImageLoadingParams>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    Extension(s3_client): Extension<AwsS3>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let images: ImageViewList = load_images_inf(&ctx.db, &user, &anchor_image_pid, params)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    views::images::img_infinite_loading(&v, &website, &images.into())
}

#[debug_handler]
pub async fn favorite_toggle(
    auth: auth::JWT,
    Path(img_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let (user, img) = load_user_and_image(&ctx.db, &auth.claims.pid, &img_pid).await?;
    if img.user_id != user.id {
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    let image: ImageView = img.favorite_image_toggle(&ctx.db).await?.into();
    views::images::favorite(&v, &website, &image)
}

#[debug_handler]
pub async fn remove(
    auth: auth::JWT,
    Path(img_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (user, img) = load_user_and_image(&ctx.db, &auth.claims.pid, &img_pid).await?;
    if img.user_id != user.id {
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    img.delete_image(&ctx.db).await?;
    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
pub async fn restore(
    auth: auth::JWT,
    Path(img_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (user, img) = load_user_and_image(&ctx.db, &auth.claims.pid, &img_pid).await?;
    if img.user_id != user.id {
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    img.restore_image(&ctx.db).await?;
    Ok((StatusCode::OK).into_response())
}

//=======================
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
