#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::controllers::training_models as TrainingRoutes;
use crate::domain::packs::Packs;
use crate::domain::website::Website;
use crate::middleware::cookie::{CookieConsentLayer, ExtractConsentState};
use crate::models::images::ImagesModelList;
use crate::models::join::user_credits_models::{
    load_user_and_credits, load_user_and_training, load_user_credit_training,
};
use crate::models::training_models::TrainingModelList;
use crate::models::users::UserPid;
use crate::models::{ImageModel, PackModel, TrainingModelModel, UserCreditModel, UserModel};
use crate::service::aws::s3::AwsS3;
use crate::service::redis::redis::Cache;
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
    pub struct Sidebar {
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
        pub help: String,
        pub help_partial: String,
        pub logout: String,
    }

    impl Dashboard {
        pub fn sidebar() -> Sidebar {
            Sidebar {
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
                help: format!("{}{}", Dashboard::BASE, Dashboard::HELP),
                help_partial: format!("{}{}", Dashboard::BASE, Dashboard::HELP_PARTIAL),
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
        pub const HELP: &'static str = "/help";
        pub const HELP_PARTIAL: &'static str = "/partial/help";
        pub const ACCOUNT: &'static str = "/account";
        pub const ACCOUNT_PARTIAL: &'static str = "/partial/account";
        pub const BILLING: &'static str = "/billing";
        pub const BILLING_PARTIAL: &'static str = "/partial/billing";

        pub const DASHBOARD_TEST_SET: &'static str = "/test/set";
        pub const DASHBOARD_TEST_GET: &'static str = "/test/get";
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
        .add(routes::Dashboard::HELP_PARTIAL, get(help_partial_dashboard))
        .add(routes::Dashboard::HELP, get(help_dashboard))
        .add(
            routes::Dashboard::ACCOUNT_PARTIAL,
            get(account_partial_dashboard),
        )
        .add(routes::Dashboard::ACCOUNT, get(account_dashboard))
        .add(
            routes::Dashboard::BILLING_PARTIAL,
            get(billing_partial_dashboard),
        )
        .add(routes::Dashboard::BILLING, get(billing_dashboard))
        .add(
            routes::Dashboard::DASHBOARD_TEST_SET,
            get(dashboard_test_set),
        )
        .add(
            routes::Dashboard::DASHBOARD_TEST_GET,
            get(dashboard_test_get),
        )
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
async fn load_first_images(db: &DatabaseConnection, id: i32, fav: bool) -> Result<ImagesModelList> {
    let list = ImageModel::find_x_images_by_user_id(db, id, fav, 30).await?;
    Ok(ImagesModelList::new(list))
}
async fn load_images_del(db: &DatabaseConnection, id: i32) -> Result<ImagesModelList> {
    let list = ImageModel::find_all_del_by_user_id(db, id).await?;
    Ok(ImagesModelList::new(list))
}
async fn load_packs(db: &DatabaseConnection) -> Result<Vec<PackModel>> {
    let list = PackModel::find_all_packs(db).await?;
    Ok(list)
}

#[debug_handler]
pub async fn dashboard_test_set(Extension(cache): Extension<Cache>) -> Result<impl IntoResponse> {
    match cache.set("testing:1", "Testing Number 2", Some(120)).await {
        Ok(_) => {
            return Ok((StatusCode::OK).into_response());
        }
        Err(e) => {
            println!("Error: {}", e);
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response());
        }
    };
}

#[debug_handler]
pub async fn dashboard_test_get(Extension(cache): Extension<Cache>) -> Result<impl IntoResponse> {
    let _ = match cache.get("testing:1").await {
        Ok(e) => return format::json(e),
        Err(e) => {
            println!("Error: {}", e);
            return Ok((StatusCode::NOT_FOUND).into_response());
        }
    };
}

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
    views::dashboard::billing_dashboard(v, user.into(), &user_credits.into(), &website, &cc_cookie)
}

#[debug_handler]
pub async fn billing_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    views::dashboard::billing_partial_dashboard(v, user.into())
}

#[debug_handler]
pub async fn account_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    views::dashboard::account_dashboard(v, user.into(), &user_credits.into(), &website, &cc_cookie)
}

#[debug_handler]
pub async fn account_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    views::dashboard::account_partial_dashboard(v, user.into())
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
        user.into(),
        &user_credits.into(),
        &website,
        &cc_cookie,
    )
}

#[debug_handler]
pub async fn notification_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    views::dashboard::notification_partial_dashboard(v, user.into())
}

#[debug_handler]
pub async fn help_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    views::dashboard::help_dashboard(v, user.into(), &user_credits.into(), &website, &cc_cookie)
}

#[debug_handler]
pub async fn help_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    views::dashboard::help_partial_dashboard(v, user.into())
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
    views::dashboard::settings_dashboard(v, user.into(), &user_credits.into(), &website, &cc_cookie)
}

#[debug_handler]
pub async fn settings_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    views::dashboard::settings_partial_dashboard(v, user.into())
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
    let training_route_check = format!(
        "{}{}",
        TrainingRoutes::routes::Models::BASE,
        TrainingRoutes::routes::Models::CHECK_W_ID_W_STATUS
    );
    views::dashboard::training_dashboard(
        v,
        user.into(),
        &user_credits.into(),
        training_models.into(),
        training_route_check,
        &website,
        &cc_cookie,
    )
}

#[debug_handler]
pub async fn training_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, training_models) = load_user_and_training(&ctx.db, &user_pid).await?;
    let training_route_check = format!(
        "{}{}",
        TrainingRoutes::routes::Models::BASE,
        TrainingRoutes::routes::Models::CHECK_W_ID_W_STATUS
    );
    views::dashboard::training_partial_dashboard(
        v,
        user.into(),
        training_models.into(),
        training_route_check,
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
    let _packs = load_packs(&ctx.db).await?;
    let packs = Packs::get_packs();
    views::dashboard::packs_dashboard(
        v,
        &user.into(),
        &user_credits.into(),
        &packs,
        &website,
        &cc_cookie,
    )
}

#[debug_handler]
pub async fn packs_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;

    let packs = Packs::get_packs();
    views::dashboard::packs_partial_dashboard(v, &user.into(), &packs)
}

#[debug_handler]
pub async fn album_deleted_dashboard(
    auth: auth::JWT,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    State(ctx): State<AppContext>,
    Extension(cache): Extension<Cache>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let images = load_images_del(&ctx.db, user.id).await?;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, false).await?.into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let is_deleted = true;
    let is_favorite = false;
    views::dashboard::photo_dashboard(
        v,
        &user.into(),
        &user_credits.into(),
        &images,
        training_models.into(),
        is_deleted,
        is_favorite,
        &website,
        &cc_cookie,
    )
}

#[debug_handler]
pub async fn album_deleted_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(cache): Extension<Cache>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let images = load_images_del(&ctx.db, user.id).await?;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, false).await?.into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let is_deleted = true;
    let is_favorite = false;
    views::dashboard::photo_partial_dashboard(
        v,
        &user.into(),
        &images,
        training_models.into(),
        &website,
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
    Extension(cache): Extension<Cache>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let images = load_first_images(&ctx.db, user.id, true).await?;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, false).await?.into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let is_deleted = false;
    let is_favorite = true;

    views::dashboard::photo_dashboard(
        v,
        &user.into(),
        &user_credits.into(),
        &images,
        training_models.into(),
        is_deleted,
        is_favorite,
        &website,
        &cc_cookie,
    )
}

#[debug_handler]
pub async fn album_favorite_partial_dashboard(
    auth: auth::JWT,

    State(ctx): State<AppContext>,
    Extension(cache): Extension<Cache>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();
    let images = load_first_images(&ctx.db, user.id, true).await?;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, false).await?.into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let is_deleted = false;
    let is_favorite = true;

    views::dashboard::photo_partial_dashboard(
        v,
        &user.into(),
        &images,
        training_models.into(),
        &website,
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
    Extension(cache): Extension<Cache>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, false).await?.into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let is_deleted = false;
    let is_favorite = false;

    views::dashboard::photo_dashboard(
        v,
        &user.into(),
        &user_credits.into(),
        &images,
        training_models.into(),
        is_deleted,
        is_favorite,
        &website,
        &cc_cookie,
    )
}

#[debug_handler]
pub async fn photo_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(cache): Extension<Cache>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;
    let mut images: ImageViewList = load_first_images(&ctx.db, user.id, false).await?.into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let is_deleted = false;
    let is_favorite = false;

    views::dashboard::photo_partial_dashboard(
        v,
        &user.into(),
        &images,
        training_models.into(),
        &website,
        &user_credits.into(),
        is_deleted,
        is_favorite,
    )
}

//Todo Remove for photo dashboard controller
// #[debug_handler]
// async fn render_dashboard(
//     auth: auth::JWT,
//     State(ctx): State<AppContext>,
//     Extension(cache): Extension<Cache>,
//     Extension(s3_client): Extension<AwsS3>,
//     Extension(website): Extension<Website>,
//     ViewEngine(v): ViewEngine<TeraView>,
// ) -> Result<impl IntoResponse> {
//     let user_pid = UserPid::new(&auth.claims.pid);
//     let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
//     let training_models = load_item_all_completed(&ctx, user.id).await?;
//     let images = load_first_images(&ctx.db, user.id, false).await?;
//     let mut images: ImageViewList = load_first_images(&ctx.db, user.id, false).await?.into();
//     let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
//     let is_deleted = false;
//     let is_favorite = false;

//     views::dashboard::photo_dashboard(
//         v,
//         &user.into(),
//         &images,
//         training_models.into(),
//         &website,
//         &user_credits.into(),
//         is_deleted,
//         is_favorite,
//     )
// }
//Todo Remove for photo dashboard controller
