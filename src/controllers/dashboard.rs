#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::controllers::training_models as TrainingRoutes;
use crate::domain::features::FeatureViewList;
use crate::domain::packs::Packs;
use crate::domain::website::Website;
use crate::middleware::cookie::{CookieConsentLayer, ExtractConsentState};
use crate::models::feature_request::FeatureRequestModelList;
use crate::models::feature_vote::FeatureVoteModelList;
use crate::models::images::ImagesModelList;
use crate::models::join::user_credits_models::{
    load_user_and_credits, load_user_and_training, load_user_credit_training,
};
use crate::models::packs::PackModelList;
use crate::models::training_models::TrainingModelList;
use crate::models::users::{RegisterParams, User, UserPid};
use crate::models::{
    FeatureRequestModel, FeatureVoteModel, ImageModel, PackModel, TrainingModelModel,
    UserCreditModel, UserModel, UserSettingsModel,
};
use crate::service::aws::s3::AwsS3;
use crate::service::fal_ai::fal_client::FalAiClient;
use crate::service::redis::redis::RedisCacheDriver;
use crate::views;
use crate::views::images::{ImageView, ImageViewList};
use axum::Extension;
use axum::{debug_handler, extract::State, response::IntoResponse};
use loco_rs::prelude::*;
use reqwest::StatusCode;

use crate::{
    models::{o_auth2_sessions, users, users::OAuth2UserProfile},
    views::auth::LoginResponse,
};
use loco_oauth2::controllers::middleware::OAuth2CookieUser;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DashboardRoutes {
        pub base: String,
        pub billing: String,
        pub billing_partial: String,
        pub sidebar: SidebarRoutes,
    }
    impl DashboardRoutes {
        pub fn init() -> Self {
            Self {
                base: format!("{}", Dashboard::BASE),
                billing: format!("{}{}", Dashboard::BASE, Dashboard::BILLING),
                billing_partial: format!("{}{}", Dashboard::BASE, Dashboard::BILLING_PARTIAL),
                sidebar: SidebarRoutes::init(),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SidebarRoutes {
        pub packs: String,
        pub packs_partial: String,
        pub photo: String,
        pub photo_partial: String,
        pub album_favorite: String,
        pub album_favorite_partial: String,
        pub album_deleted: String,
        pub album_deleted_partial: String,
        pub training_models: String,
        pub training_models_partial: String,
        pub settings: String,
        pub settings_partial: String,
        pub notifications: String,
        pub notifications_partial: String,
        pub features: String,
        pub features_partial: String,
        pub logout: String,
    }

    impl SidebarRoutes {
        pub fn init() -> Self {
            Self {
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
                training_models: format!("{}{}", Dashboard::BASE, Dashboard::TRAINING_MODELS),
                training_models_partial: format!(
                    "{}{}",
                    Dashboard::BASE,
                    Dashboard::TRAINING_MODELS_PARTIAL
                ),
                settings: format!("{}{}", Dashboard::BASE, Dashboard::SETTINGS),
                settings_partial: format!("{}{}", Dashboard::BASE, Dashboard::SETTINGS_PARTIAL),
                notifications: format!("{}{}", Dashboard::BASE, Dashboard::NOTIFICATIONS),
                notifications_partial: format!(
                    "{}{}",
                    Dashboard::BASE,
                    Dashboard::NOTIFICATIONS_PARTIAL
                ),
                features: format!("{}{}", Dashboard::BASE, Dashboard::FEATURES),
                features_partial: format!("{}{}", Dashboard::BASE, Dashboard::FEATURES_PARTIAL),
                logout: crate::controllers::auth::routes::Auth::API_LOGOUT.to_string(),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Dashboard;
    impl Dashboard {
        pub const BASE: &'static str = "/dashboard";
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
        pub const SETTINGS: &'static str = "/settings";
        pub const SETTINGS_PARTIAL: &'static str = "/partial/settings";
        pub const NOTIFICATIONS: &'static str = "/notifications";
        pub const NOTIFICATIONS_PARTIAL: &'static str = "/partial/notifications";
        pub const FEATURES: &'static str = "/features";
        pub const FEATURES_PARTIAL: &'static str = "/partial/features";
        pub const ACCOUNT: &'static str = "/account";
        pub const ACCOUNT_PARTIAL: &'static str = "/partial/account";
        pub const BILLING: &'static str = "/billing";
        pub const BILLING_PARTIAL: &'static str = "/partial/billing";

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
            routes::Dashboard::NOTIFICATIONS,
            get(notification_dashboard),
        )
        .add(
            routes::Dashboard::NOTIFICATIONS_PARTIAL,
            get(notification_partial_dashboard),
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
            get(billing_partial_dashboard),
        )
        .add(routes::Dashboard::BILLING, get(billing_dashboard))
        .add(
            routes::Dashboard::DASHBOARD_TEST_CLEAR,
            get(dashboard_test_clear),
        )
        // .add(
        //     routes::Dashboard::DASHBOARD_TEST_GET,
        //     get(dashboard_test_get),
        // )
        .add(routes::Dashboard::DASHBOARD_TEST, post(dashboard_test))
        .layer(CookieConsentLayer::new())
}

async fn load_user(db: &DatabaseConnection, pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, &pid.as_ref().to_string()).await?;
    Ok(item)
}
async fn load_user_credits(db: &DatabaseConnection, user: &UserModel) -> Result<UserCreditModel> {
    let item = UserCreditModel::load_item_by_user_id(db, user).await?;
    Ok(item)
}
// async fn load_item_all(db: &DatabaseConnection, id: i32) -> Result<TrainingModelList> {
//     let list = TrainingModelModel::find_all_by_user_id(db, id).await?;
//     Ok(TrainingModelList::new(list))
// }
async fn load_item_all_completed(ctx: &AppContext, id: i32) -> Result<TrainingModelList> {
    let list = TrainingModelModel::find_all_completed_by_user_id(&ctx.db, id).await?;
    Ok(TrainingModelList::new(list))
}
async fn load_images(db: &DatabaseConnection, id: i32, fav: bool) -> Result<ImagesModelList> {
    let list = ImageModel::find_all_by_user_id(db, id, fav).await?;
    Ok(ImagesModelList::new(list))
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
// async fn load_first_images_del(db: &DatabaseConnection, id: i32) -> Result<ImagesModelList> {
//     let list = ImageModel::find_x_images_by_user_id_del(db, id, 30).await?;
//     Ok(ImagesModelList::new(list))
// }
async fn load_images_del(db: &DatabaseConnection, id: i32) -> Result<ImagesModelList> {
    let list = ImageModel::find_all_del_by_user_id(db, id).await?;
    Ok(ImagesModelList::new(list))
}
async fn load_packs(db: &DatabaseConnection) -> Result<PackModelList> {
    let list = PackModel::find_all_packs(db).await?;
    Ok(PackModelList::new(list))
}

async fn load_features(db: &DatabaseConnection) -> Result<FeatureRequestModelList> {
    let list = FeatureRequestModel::load_top_10(&db).await?;
    Ok(list)
}
async fn load_votes(db: &DatabaseConnection, user_id: i32) -> Result<FeatureVoteModelList> {
    let list = FeatureVoteModel::load_all_votes(&db, user_id).await?;
    Ok(list)
}
async fn load_user_settings(db: &DatabaseConnection, user_id: i32) -> Result<UserSettingsModel> {
    let user_settings = UserSettingsModel::find_by_user_id(db, user_id).await?;
    Ok(user_settings)
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

// #[debug_handler]
// pub async fn dashboard_test_get(
//     Extension(cache): Extension<RedisCacheDriver>,
// ) -> Result<impl IntoResponse> {
//     let _ = match cache.get("testing:1").await {
//         Ok(e) => return format::json(e),
//         Err(e) => {
//             println!("Error: {}", e);
//             return Ok((StatusCode::NOT_FOUND).into_response());
//         }
//     };
// }

#[debug_handler]
pub async fn billing_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    views::dashboard::billing_dashboard(v, &website, user.into(), &user_credits.into(), &cc_cookie)
}
#[debug_handler]
pub async fn billing_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    views::dashboard::billing_partial_dashboard(v, &website, user.into())
}

#[debug_handler]
pub async fn notification_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    views::dashboard::notification_dashboard(
        v,
        &website,
        user.into(),
        &user_credits.into(),
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn notification_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    views::dashboard::notification_partial_dashboard(v, &website, user.into())
}

#[debug_handler]
pub async fn features_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let features = load_features(&ctx.db).await?;
    let votes = load_votes(&ctx.db, user.id).await?;
    let features_view = FeatureViewList::convert(features, votes);
    views::dashboard::features_dashboard(
        v,
        &website,
        user.into(),
        &user_credits.into(),
        &features_view,
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn features_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let user_settings = load_user_settings(&ctx.db, user.id).await?;
    let features = load_features(&ctx.db).await?;
    let votes = load_votes(&ctx.db, user.id).await?;
    let features_view = FeatureViewList::convert(features, votes);
    views::dashboard::features_partial_dashboard(v, &website, user.into(), &features_view)
}

#[debug_handler]
pub async fn settings_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let user_settings = load_user_settings(&ctx.db, user.id).await?;
    views::dashboard::settings_dashboard(
        v,
        &website,
        &user.into(),
        &user_credits.into(),
        &user_settings.into(),
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn settings_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let user_settings = load_user_settings(&ctx.db, user.id).await?;
    views::dashboard::settings_partial_dashboard(v, &website, &user.into(), &user_settings.into())
}

#[debug_handler]
pub async fn training_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;
    views::dashboard::training_dashboard(
        v,
        &website,
        user.into(),
        &user_credits.into(),
        training_models.into(),
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn training_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;
    views::dashboard::training_partial_dashboard(
        v,
        &website,
        user.into(),
        &user_credits.into(),
        training_models.into(),
    )
}

#[debug_handler]
pub async fn packs_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let packs = load_packs(&ctx.db).await?;
    views::dashboard::packs_dashboard(
        v,
        &website,
        &user.into(),
        &user_credits.into(),
        packs.into(),
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn packs_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let packs = load_packs(&ctx.db).await?;
    views::dashboard::packs_partial_dashboard(v, &website, &user.into(), packs.into())
}

#[debug_handler]
pub async fn album_deleted_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let is_deleted = true;
    let is_favorite = false;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    views::dashboard::photo_dashboard(
        v,
        &website,
        &user.into(),
        &user_credits.into(),
        &images,
        training_models.into(),
        is_deleted,
        is_favorite,
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn album_deleted_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let is_deleted = true;
    let is_favorite = false;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    views::dashboard::photo_partial_dashboard(
        v,
        &website,
        &user.into(),
        &images,
        training_models.into(),
        &user_credits.into(),
        is_deleted,
        is_favorite,
    )
}

#[debug_handler]
pub async fn album_favorite_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let is_deleted = false;
    let is_favorite = true;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    views::dashboard::photo_dashboard(
        v,
        &website,
        &user.into(),
        &user_credits.into(),
        &images,
        training_models.into(),
        is_deleted,
        is_favorite,
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn album_favorite_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let is_deleted = false;
    let is_favorite = true;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    views::dashboard::photo_partial_dashboard(
        v,
        &website,
        &user.into(),
        &images,
        training_models.into(),
        &user_credits.into(),
        is_deleted,
        is_favorite,
    )
}

#[debug_handler]
pub async fn photo_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;
    let is_deleted = false;
    let is_favorite = false;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    views::dashboard::photo_dashboard(
        v,
        &website,
        &user.into(),
        &user_credits.into(),
        &images,
        training_models.into(),
        is_deleted,
        is_favorite,
        &cc_cookie,
    )
}
#[debug_handler]
pub async fn photo_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;
    let is_deleted = false;
    let is_favorite = false;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;

    views::dashboard::photo_partial_dashboard(
        v,
        &website,
        &user.into(),
        &images,
        training_models.into(),
        &user_credits.into(),
        is_deleted,
        is_favorite,
    )
}
