#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    domain::website::Website,
    models::{
        _entities::sea_orm_active_enums::ThemePreference,
        join::user_credits_models::load_user_and_settings, users::UserPid,
    },
    views,
};
use axum::{debug_handler, response::IntoResponse, Extension};
use loco_rs::prelude::*;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SettingRoutes {
        pub base: String,
        pub update_notifications: String,
        pub update_marketing: String,
        pub dark_mode: String,
    }
    impl SettingRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Settings::BASE),
                update_notifications: format!("{}{}", Settings::BASE, Settings::API_EMAIL),
                update_marketing: format!("{}{}", Settings::BASE, Settings::API_MARKETING),
                dark_mode: format!("{}{}", Settings::BASE, Settings::API_DARK_MODE),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Settings;
    impl Settings {
        pub const BASE: &'static str = "/settings";
        pub const API_EMAIL_BOOL: &'static str = "/email/{bool}";
        pub const API_EMAIL: &'static str = "/email";
        pub const API_MARKETING_BOOL: &'static str = "/marketing/{bool}";
        pub const API_MARKETING: &'static str = "/marketing";
        pub const API_DARK_MODE_BOOL: &'static str = "/dark-mode/{bool}";
        pub const API_DARK_MODE: &'static str = "/dark-mode";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Settings::BASE)
        .add(
            routes::Settings::API_EMAIL_BOOL,
            patch(update_email_notifications),
        )
        .add(
            routes::Settings::API_MARKETING_BOOL,
            patch(update_marketing_notifications),
        )
        .add(
            routes::Settings::API_DARK_MODE_BOOL,
            patch(update_dark_mode),
        )
}

// async fn load_user_settings(db: &DatabaseConnection, user_id: i32) -> Result<UserSettingsModel> {
//     let settings = UserSettingsModel::find_by_user_id(db, user_id).await?;
//     Ok(settings)
// }
// async fn toggle_marketing_settings(
//     db: &DatabaseConnection,
//     user_id: i32,
//     new_bool: bool,
// ) -> Result<UserSettingsModel> {
//     let settings = UserSettingsModel::find_by_user_id(db, user_id).await?;
//     Ok(settings)
// }

#[debug_handler]
pub async fn update_email_notifications(
    auth: auth::JWT,
    Path(bool): Path<bool>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let new_bool = !bool;
    let user_pid = UserPid::new(&auth.claims.pid);
    let (_, user_settings) = load_user_and_settings(&ctx.db, &user_pid).await?;
    let user_settings = user_settings
        .toggle_email_settings(&ctx.db, new_bool)
        .await?;
    views::settings::email_notification(v, &website, &user_settings.into())
}
#[debug_handler]
pub async fn update_marketing_notifications(
    auth: auth::JWT,
    Path(bool): Path<bool>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let new_bool = !bool;
    let user_pid = UserPid::new(&auth.claims.pid);
    let (_, user_settings) = load_user_and_settings(&ctx.db, &user_pid).await?;
    let user_settings = user_settings
        .toggle_marketing_settings(&ctx.db, new_bool)
        .await?;
    views::settings::marketing_notifications(v, &website, &user_settings.into())
}
#[debug_handler]
pub async fn update_dark_mode(
    auth: auth::JWT,
    Path(bool): Path<bool>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let theme = match !bool {
        true => ThemePreference::Dark,
        false => ThemePreference::Light,
    };
    let user_pid = UserPid::new(&auth.claims.pid);
    let (_, user_settings) = load_user_and_settings(&ctx.db, &user_pid).await?;
    let user_settings = user_settings
        .toggle_dark_mode_settings(&ctx.db, theme)
        .await?;
    views::settings::dark_mode(v, &website, &user_settings.into())
}
