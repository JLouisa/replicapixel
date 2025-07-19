#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::{
    domain::{
        domain_services::image_generation::ImageGenerationService,
        prompt_renderer::Theme,
        website::{Website, WebsiteOptions},
    },
    middleware::{cookie::ExtractConsentState, i18nv2::LangEngine},
    models::{
        images::ImagesModelList,
        join::{
            packs::load_pack_and_translation,
            user_pack::{load_user_and_one_pack, load_user_one_training_model_one_pack},
        },
        users::UserPid,
        ImageModel,
        _entities::sea_orm_active_enums::{ImageSize, Language},
        packs::{PackDomain, PackModelList},
        training_models::TrainingModelList,
        PackModel, TrainingModelModel, UserModel,
    },
    service::{
        aws::s3::AwsS3,
        fal_ai::fal_client::FalAiClient,
        // meta::meta::{EventData, UserData},
        redis::redis::{load_cached_web, RedisCacheDriver},
    },
    views::{self, auth::UserView, images::ImageViewList, packs::PackView},
    // workers::meta_worker::{MetaConversionApiWorker, MetaConversionApiWorkerArgs},
};

use super::home::load_user;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PackRoutes {
        pub base: String,
        pub gen_pack: String,
        pub show_pack: String,
        pub show_pack_partial: String,
        pub api_packs_all: String,
        pub api_video_infinite: String,
    }
    impl PackRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Pack::API_BASE),
                gen_pack: String::from(Pack::API_GEN_PACK),
                show_pack: String::from(Pack::SHOW_PACK),
                show_pack_partial: String::from(Pack::SHOW_PACK_PARTIAL),
                api_packs_all: String::from(Pack::API_PACKS_ALL),
                api_video_infinite: format!("{}{}", Pack::API_BASE, Pack::API_INFINITE),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Pack;
    impl Pack {
        pub const SHOW_PACK_PID: &'static str = "/packs/{pid}";
        pub const SHOW_PACK: &'static str = "/packs";
        pub const SHOW_PACK_PARTIAL_PID: &'static str = "/partial/packs/{pid}";
        pub const SHOW_PACK_PARTIAL: &'static str = "/partial/packs";
        pub const API_BASE: &'static str = "/api/pack";
        pub const API_GEN_PACK_PID: &'static str = "/api/pack/gen/{pid}";
        pub const API_GEN_PACK: &'static str = "/api/pack/gen";
        pub const API_PACKS_ALL: &'static str = "/api/packs/all";
        pub const API_PACK_ADD: &'static str = "/api/pack/add";
        pub const API_INFINITE_ID: &'static str = "/infinite/{id}";
        pub const API_INFINITE: &'static str = "/infinite";
    }
}

pub fn routes() -> Routes {
    let mut routes = Routes::new()
        .add(routes::Pack::SHOW_PACK_PID, get(show_pack))
        .add(routes::Pack::API_PACKS_ALL, get(get_all_packs))
        .add(routes::Pack::SHOW_PACK_PARTIAL_PID, get(show_pack_partial))
        .add(routes::Pack::API_GEN_PACK, post(generate_packs_images))
        .add(routes::Pack::API_INFINITE_ID, get(pack_infinite_handler));

    if cfg!(debug_assertions) {
        pub const TEST_NEW_PROMPT: &'static str = "/api/pack/test/new/prompt/{theme}";
        routes = routes.add(TEST_NEW_PROMPT, get(test_new_prompt));
    }
    routes
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackParams {
    pack_pid: Uuid,
    model_pid: Option<Uuid>,
    #[serde(default)]
    image_size: ImageSize,
}

async fn load_first_images(
    db: &DatabaseConnection,
    id: i32,
    fav: bool,
    del: bool,
) -> Result<ImagesModelList> {
    let list = ImageModel::find_x_images_by_user_id(db, id, fav, del, 30).await?;
    Ok(ImagesModelList::new(list))
}
async fn load_models_all(db: &DatabaseConnection, id: i32) -> Result<TrainingModelList> {
    let list = TrainingModelModel::find_all_completed_by_user_id(db, id).await?;
    Ok(TrainingModelList::new(list))
}
async fn load_pack_by_title_url(db: &DatabaseConnection, title_url: &str) -> Result<PackModel> {
    let pack = PackModel::find_by_title_url(db, title_url).await?;
    Ok(pack)
}
async fn load_packs_all(db: &DatabaseConnection) -> Result<PackModelList> {
    let pack = PackModel::find_all_packs(db).await?;
    Ok(PackModelList::new(pack))
}
async fn increase_used_with_one_pack(db: &DatabaseConnection, pid: &Uuid) -> Result<()> {
    let _ = PackModel::plus_used_one_pack(db, pid).await?;
    Ok(())
}
async fn load_everything(
    db: &DatabaseConnection,
    user_pid: &UserPid,
    form: &PackParams,
    pack_pid: &Uuid,
) -> Result<(UserModel, Option<TrainingModelModel>, PackModel), Error> {
    let Some(model_pid) = form.model_pid else {
        let (user, pack) = load_user_and_one_pack(db, user_pid, pack_pid).await?;
        return Ok((user, None, pack));
    };
    let (user, model, pack) =
        load_user_one_training_model_one_pack(db, user_pid, &model_pid, pack_pid).await?;
    Ok((user, Some(model), pack))
}
async fn load_packs_inf(db: &DatabaseConnection, anchor_image_id: &Uuid) -> Result<PackModelList> {
    let list = PackModel::get_next_12_packs_after(db, anchor_image_id, 12).await?;
    Ok(PackModelList::new(list))
}

pub async fn test_new_prompt(
    Path(title_url): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    use crate::domain::prompt_renderer::{formatted_prompt, Themes, MODEL_UUID};
    use axum::http::StatusCode;
    use loco_rs::controller::ErrorDetail;
    use loco_rs::prelude::Error as LocoError;

    let uuid = Uuid::parse_str(MODEL_UUID).unwrap();
    let training = TrainingModelModel::find_by_pid_opt(&ctx.db, &uuid).await?;
    let themes = Themes::from_title_url(&title_url);
    let pack_model = load_pack_by_title_url(&ctx.db, &title_url).await?;

    let prompt = match formatted_prompt(&pack_model, themes, training) {
        Ok(prompt) => prompt,
        Err(err) => {
            return Err(LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error in formatted_prompt", &err.to_string()),
            ));
        }
    };

    Ok((StatusCode::OK, Json(prompt)).into_response())
}

async fn pack_infinite_handler(
    _auth: auth::JWT,
    Path(anchor_image_pid): Path<Uuid>,
    // Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    // Extension(s3_client): Extension<AwsS3>,
    State(ctx): State<AppContext>,
    ViewEngine(view_engine): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let packs = load_packs_inf(&ctx.db, &anchor_image_pid).await?.into();
    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .packs(&packs)
        .is_infinite()
        .build();
    views::packs::pack_infinite_loading(&view_engine, &website_options)
}

#[debug_handler]
pub async fn get_all_packs(
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let pack = load_packs_all(&ctx.db).await?.into();
    let website_options = WebsiteOptions::new()
        .website(&website)
        .packs(&pack)
        .is_pack_partial()
        .build();
    views::packs::get_all_packs(v, &website_options)
}

#[debug_handler]
pub async fn show_pack(
    auth: Result<auth::JWT>,
    Path(title_url): Path<String>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user: Option<UserView> = match auth {
        Ok(auth) => {
            let user_pid = UserPid::new(&auth.claims.pid);
            let user = match load_user(&ctx.db, &user_pid).await {
                Ok(user) => Some(user.into()),
                Err(_) => None,
            };
            user
        }
        Err(_) => None,
    };

    let images = load_cached_web(&ctx, &lang, &cache).await?;
    let pack: Option<&PackView> = images
        .packs
        .as_ref()
        .iter()
        .find(|p| p.title_url == title_url)
        .or_else(|| None);

    let pack = match pack {
        Some(p) => p.clone(),
        None => match lang {
            Language::English => load_pack_by_title_url(&ctx.db, &title_url).await?.into(),
            _ => load_pack_and_translation(&ctx.db, &title_url, &lang)
                .await?
                .into(),
        },
    };
    let pack_images = pack.create_item_groups();

    // if !cfg!(debug_assertions) && user.is_some() {
    //     let user_model = UserModel::find_by_pid_uuid(&ctx.db, user.clone().unwrap().pid).await?;
    //     let user_data = UserData::new(&user_model);
    //     let meta = EventData::page_view().set_user_data(&user_data);
    //     let worker_arg =
    //         MetaConversionApiWorkerArgs::new(meta, website.website_basic_info.meta_pixel.clone());
    //     if let Err(e) = MetaConversionApiWorker::perform_later(&ctx, worker_arg).await {
    //         tracing::warn!("⚠️ Failed to queue MetaConversionApiWorker: {e}");
    //     }
    // }

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .set_user(user)
        .pack(pack)
        .web_gallery(&pack_images)
        .web_images(&images)
        .is_pack()
        .build();

    views::packs::packs(v, &website_options)
}

#[debug_handler]
pub async fn show_pack_partial(
    auth: Result<auth::JWT>,
    Path(title_url): Path<String>,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user: Option<UserView> = match auth {
        Ok(auth) => {
            let user_pid = UserPid::new(&auth.claims.pid);
            let user = match load_user(&ctx.db, &user_pid).await {
                Ok(user) => Some(user.into()),
                Err(_) => None,
            };
            user
        }
        Err(_) => None,
    };

    let images = load_cached_web(&ctx, &lang, &cache).await?;
    let pack: Option<&PackView> = images
        .packs
        .as_ref()
        .iter()
        .find(|p| p.title_url == title_url)
        .or_else(|| None);

    let pack = match pack {
        Some(p) => p.clone(),
        None => match lang {
            Language::English => load_pack_by_title_url(&ctx.db, &title_url).await?.into(),
            _ => load_pack_and_translation(&ctx.db, &title_url, &lang)
                .await?
                .into(),
        },
    };
    let pack_images = pack.create_item_groups();

    // if !cfg!(debug_assertions) && user.is_some() {
    //     let user_model = UserModel::find_by_pid_uuid(&ctx.db, user.clone().unwrap().pid).await?;
    //     let user_data = UserData::new(&user_model);
    //     let meta = EventData::page_view().set_user_data(&user_data);
    //     let worker_arg =
    //         MetaConversionApiWorkerArgs::new(meta, website.website_basic_info.meta_pixel.clone());
    //     if let Err(e) = MetaConversionApiWorker::perform_later(&ctx, worker_arg).await {
    //         tracing::warn!("⚠️ Failed to queue MetaConversionApiWorker: {e}");
    //     }
    // }

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .set_user(user)
        .pack(pack)
        .web_gallery(&pack_images)
        .web_images(&images)
        .is_pack_partial()
        .build();

    views::packs::packs_partial(v, &website_options)
}

#[debug_handler]
pub async fn generate_packs_images(
    auth: auth::JWT,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(form): Json<PackParams>,
) -> Result<impl IntoResponse> {
    // 0. Load User, Pack and Training Model
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, training_model, pack) =
        load_everything(&ctx.db, &user_pid, &form, &form.pack_pid).await?;

    // 1. Call the Domain Service to perform the core logic
    let pack_domain = PackDomain::from_model(pack, form.image_size);
    let (updated_credits_model, _) =
        ImageGenerationService::generate(&ctx, &fal_ai_client, pack_domain, &user, &training_model)
            .await?;
    increase_used_with_one_pack(&ctx.db, &form.pack_pid).await?;

    // 2. Render the view using the View Models
    let images: ImageViewList = load_first_images(&ctx.db, user.id, false, false)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let training_models = load_models_all(&ctx.db, user.id).await?;

    // 3. Render the view
    let website_options = WebsiteOptions::new()
        .website(&website)
        .user(user.into())
        .user_credits(updated_credits_model.into())
        .training_models(training_models.into())
        .images(&images)
        .is_oob_credits()
        .build();
    views::dashboard::photo_partial_dashboard(v, &website_options)
}
