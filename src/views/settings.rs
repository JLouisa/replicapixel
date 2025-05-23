use crate::{
    domain::website::Website,
    models::{
        UserSettingsModel,
        _entities::sea_orm_active_enums::{Language, ThemePreference},
    },
};
use derive_more::Constructor;
use loco_rs::prelude::*;
use serde::Serialize;

use super::auth::UserView;

pub fn email_notification(
    v: impl ViewRenderer,
    website: &Website,
    user_settings: &UserSettingsView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/partials/email_notification_input.html",
        data!({ "website": website, "user_settings": user_settings }),
    )
}

pub fn marketing_notifications(
    v: impl ViewRenderer,
    website: &Website,
    user_settings: &UserSettingsView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/partials/marketing_notification_input.html",
        data!({ "website": website, "user_settings": user_settings  }),
    )
}

pub fn dark_mode(
    v: impl ViewRenderer,
    website: &Website,
    user_settings: &UserSettingsView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/partials/dark_mode_input.html",
        data!({ "website": website, "user_settings": user_settings }),
    )
}

pub fn password_change(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    msg: Option<String>,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/settings/partials/password_settings_partial.html",
        data!({ "website": website, "user": user, "msg": msg }),
    )
}

#[derive(Debug, Serialize, Clone, Constructor)]
pub struct UserSettingsView {
    pub user_id: i32,
    pub enable_notification_email: bool,
    pub enable_marketing_email: bool,
    pub language: Language,
    pub theme: ThemePreference,
}
impl From<UserSettingsModel> for UserSettingsView {
    fn from(value: UserSettingsModel) -> Self {
        Self {
            user_id: value.user_id,
            enable_notification_email: value.enable_notification_email,
            enable_marketing_email: value.enable_marketing_email,
            language: value.language,
            theme: value.theme,
        }
    }
}
