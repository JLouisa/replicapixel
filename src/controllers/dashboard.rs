#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::controllers::auth::HxRedirect;
use crate::domain::features::FeatureViewList;
use crate::domain::website::{Website, WebsiteOptions};
use crate::middleware::cookie::ExtractConsentState;
use crate::middleware::i18nv2::LangEngine;
use crate::models::_entities::sea_orm_active_enums::Language;
use crate::models::feature_request::FeatureRequestModelList;
use crate::models::feature_vote::FeatureVoteModelList;
use crate::models::images::ImagesModelList;
use crate::models::join::user_credits_models::{
    load_user_and_credits, load_user_and_settings, load_user_and_training,
    load_user_credit_training, load_user_credits_settings,
};
use crate::models::packs::PackTranslatedList;
use crate::models::training_models::TrainingModelList;
use crate::models::transactions::TransactionModelList;
use crate::models::users::{RegisterParams, UserPid};
use crate::models::{
    FeatureRequestModel, FeatureVoteModel, ImageModel, OAuth2SessionModel, PackModel, PlanModel,
    TrainingModelModel, TransactionModel, UserModel,
};
use crate::service::aws::s3::AwsS3;
use crate::service::redis::redis::{load_cached_web, RedisCacheDriver};
use crate::views;
use crate::views::dashboard::TransactionViewList;
use crate::views::images::ImageViewList;
use axum::response::Redirect;
use axum::Extension;
use axum::{debug_handler, extract::State, response::IntoResponse};
use loco_rs::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::controllers::auth::routes as AuthRoutes;

use std::collections::HashMap;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DashboardRoutes {
        pub base: String,
        pub billing: String,
        pub billing_partial: String,
        pub create_training_models: String,
        pub create_training_models_partial: String,
        pub sidebar: SidebarRoutes,
    }
    impl DashboardRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Dashboard::BASE),
                billing: format!("{}{}", Dashboard::BASE, Dashboard::BILLING),
                billing_partial: format!("{}{}", Dashboard::BASE, Dashboard::BILLING_PARTIAL),

                create_training_models: format!(
                    "{}{}",
                    Dashboard::BASE,
                    Dashboard::CREATE_TRAINING_MODELS
                ),
                create_training_models_partial: format!(
                    "{}{}",
                    Dashboard::BASE,
                    Dashboard::CREATE_TRAINING_MODELS_PARTIAL
                ),
                sidebar: SidebarRoutes::init(),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SidebarRoutes {
        pub training_models: String,
        pub training_models_partial: String,
        pub packs: String,
        pub packs_partial: String,
        pub photo: String,
        pub photo_partial: String,
        pub album_favorite: String,
        pub album_favorite_partial: String,
        pub album_deleted: String,
        pub album_deleted_partial: String,
        pub settings: String,
        pub settings_partial: String,
        pub features: String,
        pub features_partial: String,
        pub logout: String,
    }

    impl SidebarRoutes {
        pub fn init() -> Self {
            Self {
                training_models: format!("{}{}", Dashboard::BASE, Dashboard::TRAINING_MODELS),
                training_models_partial: format!(
                    "{}{}",
                    Dashboard::BASE,
                    Dashboard::TRAINING_MODELS_PARTIAL
                ),
                packs: format!("{}{}", Dashboard::BASE, Dashboard::PACKS),
                packs_partial: format!("{}{}", Dashboard::BASE, Dashboard::PACKS_PARTIAL),
                photo: format!("{}{}", Dashboard::BASE, Dashboard::PHOTO),
                photo_partial: format!("{}{}", Dashboard::BASE, Dashboard::PHOTO_PARTIAL),
                album_favorite: format!("{}{}", Dashboard::BASE, Dashboard::ALBUM_FAVORITE),
                album_favorite_partial: format!(
                    "{}{}",
                    Dashboard::BASE,
                    Dashboard::ALBUM_FAVORITE_PARTIAL
                ),
                album_deleted: format!("{}{}", Dashboard::BASE, Dashboard::ALBUM_DELETED),
                album_deleted_partial: format!(
                    "{}{}",
                    Dashboard::BASE,
                    Dashboard::ALBUM_DELETED_PARTIAL
                ),

                settings: format!("{}{}", Dashboard::BASE, Dashboard::SETTINGS),
                settings_partial: format!("{}{}", Dashboard::BASE, Dashboard::SETTINGS_PARTIAL),
                features: format!("{}{}", Dashboard::BASE, Dashboard::FEATURES),
                features_partial: format!("{}{}", Dashboard::BASE, Dashboard::FEATURES_PARTIAL),
                logout: crate::controllers::auth::routes::Auth::API_LOGOUT.to_string(),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Dashboard;
    impl Dashboard {
        pub const BASE: &'static str = "/studio";
        pub const DASHBOARD: &'static str = "/";
        pub const PACKS: &'static str = "/packs";
        pub const PACKS_PARTIAL: &'static str = "/partial/packs";
        pub const PHOTO: &'static str = "/photo";
        pub const PHOTO_PARTIAL: &'static str = "/partial/photo";
        pub const ALBUM_FAVORITE: &'static str = "/album/favorite";
        pub const ALBUM_FAVORITE_PARTIAL: &'static str = "/partial/album/favorite";
        pub const ALBUM_DELETED: &'static str = "/album/deleted";
        pub const ALBUM_DELETED_PARTIAL: &'static str = "/partial/album/deleted";
        pub const TRAINING_MODELS: &'static str = "/models";
        pub const TRAINING_MODELS_PARTIAL: &'static str = "/partial/models";
        pub const CREATE_TRAINING_MODELS: &'static str = "/models/create";
        pub const CREATE_TRAINING_MODELS_PARTIAL: &'static str = "/partial/models/create";
        pub const SETTINGS: &'static str = "/settings";
        pub const SETTINGS_PARTIAL: &'static str = "/partial/settings";
        pub const FEATURES: &'static str = "/features";
        pub const FEATURES_PARTIAL: &'static str = "/partial/features";
        pub const ACCOUNT: &'static str = "/account";
        pub const ACCOUNT_PARTIAL: &'static str = "/partial/account";
        pub const BILLING: &'static str = "/billing";
        pub const BILLING_PARTIAL: &'static str = "/partial/billing";
        pub const BILLING_NEW: &'static str = "/billing?partial={enum_htmx}";

        pub const DASHBOARD_TEST_SET: &'static str = "/test/set";
        pub const DASHBOARD_TEST_GET: &'static str = "/test/get";
        pub const DASHBOARD_TEST_CLEAR: &'static str = "/test/clear";
        pub const DASHBOARD_TEST: &'static str = "/test";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Dashboard::BASE)
        .add(routes::Dashboard::DASHBOARD, get(photo_dashboard))
        .add(routes::Dashboard::PACKS, get(packs_dashboard))
        .add(
            routes::Dashboard::PACKS_PARTIAL,
            get(packs_partial_dashboard),
        )
        .add(routes::Dashboard::PHOTO, get(photo_dashboard))
        .add(
            routes::Dashboard::PHOTO_PARTIAL,
            get(photo_partial_dashboard),
        )
        .add(routes::Dashboard::TRAINING_MODELS, get(training_dashboard))
        .add(
            routes::Dashboard::TRAINING_MODELS_PARTIAL,
            get(training_partial_dashboard),
        )
        .add(
            routes::Dashboard::ALBUM_FAVORITE,
            get(album_favorite_dashboard),
        )
        .add(
            routes::Dashboard::ALBUM_FAVORITE_PARTIAL,
            get(album_favorite_partial_dashboard),
        )
        .add(
            routes::Dashboard::ALBUM_DELETED,
            get(album_deleted_dashboard),
        )
        .add(
            routes::Dashboard::ALBUM_DELETED_PARTIAL,
            get(album_deleted_partial_dashboard),
        )
        .add(
            routes::Dashboard::SETTINGS_PARTIAL,
            get(settings_partial_dashboard),
        )
        .add(routes::Dashboard::SETTINGS, get(settings_dashboard))
        .add(
            routes::Dashboard::FEATURES_PARTIAL,
            get(features_partial_dashboard),
        )
        .add(routes::Dashboard::FEATURES, get(features_dashboard))
        .add(
            routes::Dashboard::BILLING_PARTIAL,
            get(billing_partial_dashboard_new),
        )
        .add(routes::Dashboard::BILLING, get(billing_dashboard_new))
        .add(routes::Dashboard::DASHBOARD_TEST, post(dashboard_test))
        .add(
            routes::Dashboard::CREATE_TRAINING_MODELS,
            get(new_training_dashboard),
        )
        .add(
            routes::Dashboard::CREATE_TRAINING_MODELS_PARTIAL,
            get(new_training_dashboard_partials),
        )
        .add(
            routes::Dashboard::DASHBOARD_TEST_CLEAR,
            get(dashboard_test_clear),
        )
}

async fn load_user(db: &DatabaseConnection, pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, &pid.as_ref().to_string()).await?;
    Ok(item)
}
pub async fn load_item_all_completed(ctx: &AppContext, id: i32) -> Result<TrainingModelList> {
    let list = TrainingModelModel::find_all_completed_by_user_id(&ctx.db, id).await?;
    Ok(TrainingModelList::new(list))
}
pub async fn load_first_images(
    db: &DatabaseConnection,
    id: i32,
    fav: bool,
    del: bool,
) -> Result<ImagesModelList> {
    let list = ImageModel::find_x_images_by_user_id(db, id, fav, del, 30).await?;
    Ok(ImagesModelList::new(list))
}
async fn load_packs_translated(
    db: &DatabaseConnection,
    lang: &Language,
) -> Result<PackTranslatedList> {
    let list = PackModel::find_all_translated(db, lang).await?;
    Ok(list)
}

async fn load_features(db: &DatabaseConnection) -> Result<FeatureRequestModelList> {
    let list = FeatureRequestModel::load_top_10(&db).await?;
    Ok(list)
}
async fn load_votes(db: &DatabaseConnection, user_id: i32) -> Result<FeatureVoteModelList> {
    let list = FeatureVoteModel::load_all_votes(&db, user_id).await?;
    Ok(list)
}
pub async fn is_oauth(db: &DatabaseConnection, user_id: i32) -> Result<bool> {
    let is_oauth = OAuth2SessionModel::is_find_by_user_id(db, user_id).await?;
    Ok(is_oauth)
}
async fn load_transactions(db: &DatabaseConnection, user_id: i32) -> Result<TransactionModelList> {
    let list = TransactionModel::find_all_user_txn(db, user_id).await?;
    Ok(list)
}
async fn load_plans(db: &DatabaseConnection) -> Result<HashMap<i32, PlanModel>> {
    let list = PlanModel::find_all(db).await?;
    let mut map: HashMap<i32, PlanModel> = HashMap::new();

    for item in list.0 {
        map.insert(item.id, item);
    }

    Ok(map)
}

#[debug_handler]
pub async fn dashboard_test(Json(params): Json<RegisterParams>) -> Result<impl IntoResponse> {
    dbg!(params);
    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
pub async fn dashboard_test_clear(State(ctx): State<AppContext>) -> Result<impl IntoResponse> {
    match ctx.cache.clear().await {
        Ok(_) => {
            return Ok((StatusCode::OK).into_response());
        }
        Err(e) => {
            println!("Error: {}", e);
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response());
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CurrentPage {
    Billing,
    Features,
    Settings,
    Models,
    Packs,
    Deleted,
    Favorite,
    Album,
}

#[debug_handler]
pub async fn billing_dashboard_new(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };

    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let orders = load_transactions(&ctx.db, user.id).await?;
    let plans = load_plans(&ctx.db).await?;
    let orders_view = TransactionViewList::from_model(orders, plans);

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .orders(&orders_view)
        .current_page(CurrentPage::Billing)
        .build();

    Ok(views::dashboard::billing_dashboard_new(v, &website_options).into_response())
}
#[debug_handler]
pub async fn billing_partial_dashboard_new(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };

    let user = load_user(&ctx.db, &user_pid).await?;
    let orders = load_transactions(&ctx.db, user.id).await?;
    let plans = load_plans(&ctx.db).await?;
    let orders_view = TransactionViewList::from_model(orders, plans);

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .orders(&orders_view)
        .current_page(CurrentPage::Billing)
        .build();

    Ok(views::dashboard::billing_partial_dashboard_new(v, &website_options).into_response())
}

#[debug_handler]
pub async fn features_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };

    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let features = load_features(&ctx.db).await?;
    let votes = load_votes(&ctx.db, user.id).await?;
    let features_view = FeatureViewList::convert(features, votes);

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .features(&features_view)
        .current_page(CurrentPage::Features)
        .build();

    Ok(views::dashboard::features_dashboard(v, &website_options).into_response())
}
#[debug_handler]
pub async fn features_partial_dashboard(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let user = load_user(&ctx.db, &user_pid).await?;
    let features = load_features(&ctx.db).await?;
    let votes = load_votes(&ctx.db, user.id).await?;
    let features_view = FeatureViewList::convert(features, votes);

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .features(&features_view)
        .build();

    Ok(views::dashboard::features_partial_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn settings_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };
    let (user, user_credits, user_settings) =
        load_user_credits_settings(&ctx.db, &user_pid).await?;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .user_settings(user_settings.into())
        .current_page(CurrentPage::Settings)
        .build();

    Ok(views::dashboard::settings_dashboard(v, &website_options).into_response())
}
#[debug_handler]
pub async fn settings_partial_dashboard(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (user, user_settings) = load_user_and_settings(&ctx.db, &user_pid).await?;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_settings(user_settings.into())
        .build();

    Ok(views::dashboard::settings_partial_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn training_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .current_page(CurrentPage::Models)
        .build();

    Ok(views::dashboard::training_dashboard(v, &website_options).into_response())
}
#[debug_handler]
pub async fn training_partial_dashboard(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .current_page(CurrentPage::Models)
        .build();

    Ok(views::dashboard::training_partial_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn new_training_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .current_page(CurrentPage::Models)
        .build();

    Ok(views::dashboard::create_training_dashboard(v, &website_options).into_response())
}
#[debug_handler]
pub async fn new_training_dashboard_partials(
    auth: Result<auth::JWT>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let _user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .build();
    Ok(views::dashboard::create_training_dashboard_partial(v, &website_options).into_response())
}

#[debug_handler]
pub async fn packs_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;
    let packs = match load_cached_web(&ctx, &lang, &cache).await {
        Ok(images) => images.packs().clone(),
        Err(_) => load_packs_translated(&ctx.db, &lang).await?.into(),
    };
    let packs = packs.into();
    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .packs(&packs)
        .current_page(CurrentPage::Packs)
        .build();

    Ok(views::dashboard::packs_dashboard(v, &website_options).into_response())
}
#[debug_handler]
pub async fn packs_partial_dashboard(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (_, training_models) = load_user_and_training(&ctx.db, &user_pid).await?;
    let packs = match load_cached_web(&ctx, &lang, &cache).await {
        Ok(images) => images.packs().clone(),
        Err(_) => load_packs_translated(&ctx.db, &lang).await?.into(),
    };
    let packs = packs.into();
    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .training_models(training_models.into())
        .packs(&packs)
        .current_page(CurrentPage::Packs)
        .build();

    Ok(views::dashboard::packs_partial_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn album_deleted_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let images: ImageViewList = load_first_images(&ctx.db, user.id, false, true)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .images(&images)
        .current_page(CurrentPage::Deleted)
        .is_deleted()
        .is_initial_load()
        .build();

    Ok(views::dashboard::photo_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn album_deleted_partial_dashboard(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let is_deleted = true;
    let is_favorite = false;
    let images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .images(&images)
        .is_deleted()
        .build();

    Ok(views::dashboard::photo_partial_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn album_favorite_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let images: ImageViewList = load_first_images(&ctx.db, user.id, true, false)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .images(&images)
        .current_page(CurrentPage::Favorite)
        .is_favorite()
        .is_initial_load()
        .build();

    Ok(views::dashboard::photo_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn album_favorite_partial_dashboard(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let images: ImageViewList = load_first_images(&ctx.db, user.id, true, false)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .images(&images)
        .is_favorite()
        .build();

    Ok(views::dashboard::photo_partial_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn photo_dashboard(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
        }
    };
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;
    let images: ImageViewList = load_first_images(&ctx.db, user.id, false, false)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .images(&images)
        .current_page(CurrentPage::Album)
        .is_initial_load()
        .build();

    Ok(views::dashboard::photo_dashboard(v, &website_options).into_response())
}

#[debug_handler]
pub async fn photo_partial_dashboard(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;
    let is_deleted = false;
    let is_favorite = false;
    let images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .images(&images)
        .build();

    Ok(views::dashboard::photo_partial_dashboard(v, &website_options).into_response())
}
