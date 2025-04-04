#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::controllers::training_models as TrainingRoutes;
use crate::domain::dashboard_sidebar::DashboardSidebar;
use crate::domain::website::Website;
use crate::domain::{image::Image, packs::Packs};
use crate::models::_entities::users;
use crate::models::training_models::TrainingModelList;
use crate::models::{
    TrainingModelActiveModel, TrainingModelEntity, TrainingModelModel, UserCreditEntity,
    UserCreditModel, UserEntity, UserModel,
};
use crate::views;
use axum::{
    debug_handler,
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use loco_rs::prelude::*;

pub mod routes {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
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
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Dashboard::BASE)
        .add(routes::Dashboard::DASHBOARD, get(render_dashboard))
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
}

async fn load_user(db: &DatabaseConnection, pid: &str) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, pid).await?;
    Ok(item)
}
async fn load_item(db: &DatabaseConnection, id: i32) -> Result<TrainingModelModel> {
    let item = TrainingModelEntity::find_by_id(id).one(db).await?;
    item.ok_or_else(|| Error::NotFound)
}
async fn load_user_credits(db: &DatabaseConnection, id: i32) -> Result<UserCreditModel> {
    let item = UserCreditEntity::find_by_id(id).one(db).await?;
    item.ok_or_else(|| Error::NotFound)
}
async fn load_item_all(db: &DatabaseConnection, id: i32) -> Result<TrainingModelList> {
    let list = TrainingModelModel::find_all_by_user_id(db, id).await?;
    Ok(TrainingModelList::new(list))
}
async fn load_item_all_completed(ctx: &AppContext, id: i32) -> Result<TrainingModelList> {
    let list = TrainingModelModel::find_all_completed_by_user_id(&ctx.db, id).await?;
    Ok(TrainingModelList::new(list))
}

#[debug_handler]
pub async fn billing_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let sidebar_routes = routes::Dashboard::sidebar();
    views::dashboard::billing_dashboard(v, user.into(), sidebar_routes, &user_credits.into())
}

#[debug_handler]
pub async fn billing_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    views::dashboard::billing_partial_dashboard(v, user.into())
}

#[debug_handler]
pub async fn account_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let sidebar_routes = routes::Dashboard::sidebar();
    views::dashboard::account_dashboard(v, user.into(), sidebar_routes, &user_credits.into())
}

#[debug_handler]
pub async fn account_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    views::dashboard::account_partial_dashboard(v, user.into())
}

#[debug_handler]
pub async fn notification_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let sidebar_routes = routes::Dashboard::sidebar();
    views::dashboard::notification_dashboard(v, user.into(), sidebar_routes, &user_credits.into())
}

#[debug_handler]
pub async fn notification_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    views::dashboard::notification_partial_dashboard(v, user.into())
}

#[debug_handler]
pub async fn help_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let sidebar_routes = routes::Dashboard::sidebar();
    views::dashboard::help_dashboard(v, user.into(), sidebar_routes, &user_credits.into())
}

#[debug_handler]
pub async fn help_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    views::dashboard::help_partial_dashboard(v, user.into())
}

#[debug_handler]
pub async fn settings_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let sidebar_routes = routes::Dashboard::sidebar();
    views::dashboard::settings_dashboard(v, user.into(), sidebar_routes, &user_credits.into())
}

#[debug_handler]
pub async fn settings_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    views::dashboard::settings_partial_dashboard(v, user.into())
}

#[debug_handler]
pub async fn album_deleted_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models = load_item_all(&ctx.db, user.id).await?;

    //Todo Get Images from database
    let images: Vec<Image> = Image::get_mock_images(true);

    let images_fav: Vec<Image> = images.into_iter().filter(|i| i.is_deleted).collect();
    let sidebar_routes = routes::Dashboard::sidebar();
    let website = Website::init();
    views::dashboard::photo_dashboard(
        v,
        &user.into(),
        &images_fav,
        sidebar_routes,
        training_models.into(),
        &website,
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn album_deleted_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();

    //Todo Get Images from database
    let images = Image::get_mock_images(true);

    let images_fav = images.into_iter().filter(|i| i.is_deleted).collect();
    let website = Website::init();
    views::dashboard::photo_partial_dashboard(
        v,
        &user.into(),
        &images_fav,
        training_models.into(),
        &website,
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn album_favorite_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();

    //Todo Get Images from database
    let images = Image::get_mock_images(true);
    let images = images.into_iter().filter(|i| i.is_favorite).collect();

    let sidebar_routes = routes::Dashboard::sidebar();
    let website = Website::init();
    views::dashboard::photo_dashboard(
        v,
        &user.into(),
        &images,
        sidebar_routes,
        training_models.into(),
        &website,
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn album_favorite_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models: TrainingModelList = TrainingModelList::empty();

    //Todo Get Images from database
    let images = Image::get_mock_images(true);
    let images = images.into_iter().filter(|i| i.is_favorite).collect();

    let website = Website::init();
    views::dashboard::photo_partial_dashboard(
        v,
        &user.into(),
        &images,
        training_models.into(),
        &website,
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn photo_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;

    //Todo Get Images from database
    let images = Image::get_mock_images(true);
    let images: Vec<Image> = images.into_iter().filter(|i| i.is_deleted).collect();

    let sidebar_routes = routes::Dashboard::sidebar();
    let website = Website::init();
    views::dashboard::photo_dashboard(
        v,
        &user.into(),
        &images.into(),
        sidebar_routes,
        training_models.into(),
        &website.into(),
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn photo_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models = load_item_all_completed(&ctx, user.id).await?;

    //Todo Get Images from database
    let images = Image::get_mock_images(true);
    let images = images.into_iter().filter(|i| !i.is_deleted).collect();

    let website = Website::init();
    views::dashboard::photo_partial_dashboard(
        v,
        &user.into(),
        &images,
        training_models.into(),
        &website,
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn training_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models = load_item_all(&ctx.db, user.id).await?;
    let training_route_check = format!(
        "{}{}",
        TrainingRoutes::routes::Models::BASE,
        TrainingRoutes::routes::Models::CHECK_W_ID_W_STATUS
    );
    let sidebar_routes = routes::Dashboard::sidebar();
    views::dashboard::training_dashboard(
        v,
        user.into(),
        training_models.into(),
        training_route_check,
        sidebar_routes,
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn training_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let training_models = load_item_all(&ctx.db, user.id).await?;

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
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let packs = Packs::get_packs();
    let sidebar_routes = routes::Dashboard::sidebar();
    views::dashboard::packs_dashboard(
        v,
        &user.into(),
        &packs,
        sidebar_routes,
        &user_credits.into(),
    )
}

#[debug_handler]
pub async fn packs_partial_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let packs = Packs::get_packs();
    views::dashboard::packs_partial_dashboard(v, &user.into(), &packs)
}

//Todo Remove for photo dashboard controller
#[debug_handler]
async fn render_dashboard(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user = load_user(&ctx.db, &auth.claims.pid).await?;
    let user_credits = load_user_credits(&ctx.db, user.id).await?;
    let training_models = load_item_all(&ctx.db, user.id).await?;

    //Todo Get Images from database
    let images = Image::get_mock_images(true);
    let images = images.into_iter().filter(|i| !i.is_deleted).collect();

    let sidebar_routes = routes::Dashboard::sidebar();
    let website = Website::init();
    views::dashboard::photo_dashboard(
        v,
        &user.into(),
        &images,
        sidebar_routes,
        training_models.into(),
        &website,
        &user_credits.into(),
    )
}
