#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    domain::website::Website,
    models::{
        join::user_credits_models::load_user_and_credits,
        users::UserPid,
        UserSettingsModel,
        _entities::{
            sea_orm_active_enums::{Language, ThemePreference},
            user_settings,
        },
    },
};
use axum::{debug_handler, http::StatusCode, response::IntoResponse, Extension, Json};
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
        pub const API_EMAIL: &'static str = "/email";
        pub const API_MARKETING: &'static str = "/marketing";
        pub const API_DARK_MODE: &'static str = "/dark-mode";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Settings::BASE)
        .add(routes::Settings::API_EMAIL, patch(settings_email))
}

async fn load_user_settings(db: &DatabaseConnection, user_id: i32) -> Result<UserSettingsModel> {
    let settings = UserSettingsModel::find_by_user_id(db, user_id).await?;
    Ok(settings)
}

#[debug_handler]
pub async fn settings_email(
    auth: auth::JWT,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    // let is_home = true;
    // let user_pid = UserPid::new(&auth.claims.pid);
    // let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    // let user_settings = load_user_settings(&ctx.db, user.id).await?;
    // views::home::home(v, &website, is_home)
    format::empty()
}
