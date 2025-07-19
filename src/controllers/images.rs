#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query, Extension};
use axum::{http::StatusCode, response::IntoResponse, Json};
use derive_more::Constructor;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::domain_services::image_generation::ImageGenerationService;
use crate::domain::url::Url;
use crate::domain::website::{Website, WebsiteOptions};
use crate::middleware::i18nv2::LangEngine;
use crate::models::_entities::sea_orm_active_enums::{ImageSize, Status};
use crate::models::images::{
    AltText, ImageNew, ImageNewList, ImagesModelList, SysPrompt, UserPrompt,
};
use crate::models::join::user_credits_models::load_user_and_one_training_model;
use crate::models::join::user_image::load_user_and_image;
use crate::models::packs::PackDomain;
use crate::models::users::UserPid;
use crate::models::{ImageActiveModel, ImageModel, TrainingModelModel, UserCreditModel, UserModel};
use crate::service::aws::s3::{AwsS3, S3Key};
use crate::service::fal_ai::fal_client::{Lora, WebhookPayload};
use crate::service::redis::redis::RedisCacheDriver;
use crate::views::images::{ImageView, ImageViewList};
use crate::{models::_entities::images::Entity, service::fal_ai::fal_client::FalAiClient, views};

pub const IMAGE_COST: i32 = 1;

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
        pub const API_IMAGE_DOWNLOAD_LINK: &'static str = "/download/{img_pid}";

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
        .add(routes::Images::IMAGE_GENERATE, post(generate_image))
        .add(routes::Images::IMAGE_CHECK_ID, get(check_image))
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
        .add(
            routes::Images::API_IMAGE_DOWNLOAD_LINK,
            get(img_s3_download_link),
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
    #[serde(default)]
    pub model: WebhookPayload,
}
impl ImageGenRequestParams {
    fn normalize(text: &str) -> String {
        text.replace(['\'', '"'], "’")
            .chars()
            .filter(|c| !c.is_control())
            .collect()
    }
    pub fn sanitize(&mut self) {
        let prompt = Self::normalize(self.prompt.as_ref());
        self.prompt = UserPrompt::new(prompt);
    }
}

pub trait ImageGenerationTrait {
    fn formatted_prompt(&self, model: &TrainingModelModel) -> UserPrompt;
    fn steps(&self) -> i32;
    fn num_images(&self) -> i32;
    fn image_size(&self) -> ImageSize;
    fn cost(&self) -> i32;
    fn quality_model(&self) -> WebhookPayload;
    fn process(self, model: &Option<TrainingModelModel>, user_pid: &UserModel) -> ImageNewList;
}
impl ImageGenerationTrait for PackDomain {
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
    fn quality_model(&self) -> WebhookPayload {
        WebhookPayload::default()
    }

    fn process(self, model: &Option<TrainingModelModel>, user: &UserModel) -> ImageNewList {
        let model_id = match model {
            Some(m) => Some(m.id),
            None => None,
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
        let user_prompt = match model {
            Some(m) => self.formatted_prompt(&m),
            None => UserPrompt::new(self.pack_prompts.clone()),
        };
        let sys_prompt = SysPrompt::new(user_prompt.as_ref());
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
                    user_prompt: user_prompt.to_owned(),
                    sys_prompt: sys_prompt.to_owned(),
                    alt: alt.to_owned(),
                    loras: loras.clone(),
                    image_size: self.image_size,
                    image_cost: IMAGE_COST,
                    num_inference_steps: self.num_images() as i32,
                    model: self.quality_model(),
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
        self.num_images as i32 * IMAGE_COST
    }
    fn quality_model(&self) -> WebhookPayload {
        self.model.clone()
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
                    image_cost: IMAGE_COST,
                    num_inference_steps: self.num_inference_steps as i32,
                    image_s3_key: s3_key,
                    image_size: self.image_size,
                    loras: loras.clone(),
                    model: self.quality_model(),
                    ..Default::default()
                }
            })
            .collect::<Vec<ImageNew>>()
            .into()
    }
}

#[derive(Deserialize, Debug)]
pub struct InfiniteLoadingParams {
    pub deleted: Option<bool>,
    pub favorite: Option<bool>,
}

#[derive(Serialize, Debug, Constructor, Clone)]
pub struct ImageDownloadLink {
    pre_url: Url,
}

async fn load_user_opt_training(
    ctx: &AppContext,
    user_pid: &UserPid,
    params: &ImageGenRequestParams,
) -> Result<(UserModel, Option<TrainingModelModel>)> {
    let models = match params.training_model_pid {
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
    Ok(models)
}
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
    params: InfiniteLoadingParams,
) -> Result<ImagesModelList> {
    let list =
        ImageModel::get_next_20_images_after(db, user.id, anchor_image_id, 20, params).await?;
    Ok(ImagesModelList::new(list))
}

#[debug_handler]
pub async fn img_s3_download_link(
    auth: auth::JWT,
    Path(img_pid): Path<Uuid>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(s3_client): Extension<AwsS3>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let (_user, image) = load_user_and_image(&ctx.db, &auth.claims.pid, &img_pid).await?;
    let mut image_view: ImageView = (&image).into();

    if image_view.image_url_fal.is_some()
        && image_view.image_status == Status::Completed.to_string()
    {
        let updated = image_view.get_pre_url_mut(&s3_client, &cache).await;
        image_view = updated.clone();
    }

    let cached_url = match image_view.s3_pre_url {
        Some(v) => {
            tracing::info!("Pre download link url cached found: {}", &image_view.pid);
            Url::new(v)
        }
        None => {
            tracing::warn!(
                "Download Link Url cached not found! Creating S3 download URL: {}",
                &image_view.pid
            );
            let s3_key = S3Key::new(image.image_s3_key.clone());
            let exists = s3_client.check_object_exists(&s3_key).await?;
            if !exists {
                return Ok((StatusCode::NO_CONTENT).into_response().into_response());
            }
            let pre_url = s3_client.get_object_pre(&s3_key, None).await?;
            let _ = cache.set_s3_pre_url(&image_view).await;
            pre_url
        }
    };
    let url = ImageDownloadLink::new(cached_url);

    return Ok((StatusCode::OK, Json(url)).into_response());
}

#[debug_handler]
pub async fn generate_image(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
    Json(mut params): Json<ImageGenRequestParams>,
) -> Result<Response> {
    // 0. Validate request payload format
    params.validate()?;
    params.sanitize();

    // 1. Load User and Training Model
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, training_model) = load_user_opt_training(&ctx, &user_pid, &params).await?;

    // 2. Call the Domain Service to perform the core logic
    let (updated_credits, saved_images) =
        ImageGenerationService::generate(&ctx, &fal_ai_client, params, &user, &training_model)
            .await?;

    // 3. Render the view using the View Models
    let saved_images: ImageViewList = saved_images.into();
    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .images(&saved_images)
        .user_credits(updated_credits.into())
        .is_image_gen()
        .is_oob_credits()
        .build();
    views::images::image_router(&v, &website_options)
}

#[debug_handler]
pub async fn img_s3_upload_completed(
    auth: auth::JWT,
    Path(img_pid): Path<Uuid>,
    Extension(s3_client): Extension<AwsS3>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (user, image) = load_user_and_image(&ctx.db, &auth.claims.pid, &img_pid).await?;
    let s3_key = AwsS3::init_img_s3_key(&user.pid, &image.pid);

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
pub async fn check_image(
    auth: auth::JWT,
    Path(pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(s3_client): Extension<AwsS3>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    let (user, image) = load_user_and_image(&ctx.db, &auth.claims.pid, &pid).await?;

    if image.user_id != user.id {
        return Err(Error::Unauthorized("Unauthorized".to_string()));
    }

    let image_list = match image.status {
        Status::Processing => {
            let image_view = ImageView::from(&image);
            match image_view.set_pre_url(&s3_client).await {
                Ok(updated) => ImageViewList::one(updated),
                Err(_) => ImageViewList::one(image.into()),
            }
        }
        Status::Failed => ImageViewList::one(image.into()),
        _ => return Ok(StatusCode::NO_CONTENT.into_response()),
    };

    let user_credits = load_credits(&ctx.db, user.id).await?;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .images(&image_list)
        .user_credits(user_credits.into())
        .build();
    views::images::image_router(&v, &website_options)
}

async fn image_infinite_handler(
    auth: auth::JWT,
    Path(anchor_image_pid): Path<Uuid>,
    Query(params): Query<InfiniteLoadingParams>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    Extension(s3_client): Extension<AwsS3>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let images: ImageViewList = load_images_inf(&ctx.db, &user, &anchor_image_pid, params)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .images(&images)
        .is_infinite()
        .build();
    views::images::image_router(&v, &website_options)
}

#[debug_handler]
pub async fn favorite_toggle(
    auth: auth::JWT,
    Path(img_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    let (user, img) = load_user_and_image(&ctx.db, &auth.claims.pid, &img_pid).await?;
    if img.user_id != user.id {
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    let image: ImageView = img.favorite_image_toggle(&ctx.db).await?.into();
    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .image(&image)
        .build();
    views::images::favorite(&v, &website_options)
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

#[debug_handler]
pub async fn get_one(
    auth: auth::JWT,
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    let user = UserModel::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let image = load_item_pid(&ctx, id).await?;
    if image.user_id != user.id {
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    let image_view = image.into();
    let website_options = WebsiteOptions::new().image(&image_view).language(&lang);
    views::images::show(&v, &website_options)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}
