use super::auth::{UserCreditsView, UserView};
use super::images::{ImageView, ImageViewList};
use super::settings::UserSettingsView;
use super::training_models::TrainingModelView;
use crate::domain::dashboard_sidebar::DashboardSidebar;
use crate::domain::features::FeatureViewList;
use crate::domain::website::Website;
use crate::middleware::cookie::CookieConsent;
use crate::models::_entities::sea_orm_active_enums::{Language, ThemePreference};
use crate::models::packs::PackModelList;
use crate::models::{PackModel, UserSettingsModel};
use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use serde::Serialize;

pub fn billing_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/billing/billing.html",
        data!(
            {
                "website": website, "user": user,
                "credits": credits, "cc_cookie": cc_cookie
            }
        ),
    )
}
pub fn billing_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/billing/billing_partial.html",
        data!({ "website": website, "user": user }),
    )
}

pub fn notification_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/notification/notification.html",
        data!({ "website": website, "user": user, "credits": credits }),
    )
}
pub fn notification_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/notification/notification_partial.html",
        data!({ "website": website, "user": user }),
    )
}

pub fn features_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    features_view: &FeatureViewList,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/features.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "features_view": features_view, "cc_cookie": cc_cookie
            }
        ),
    )
}
pub fn features_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    features_view: &FeatureViewList,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/features/features_partial.html",
        data!(
            {
                "website": website, "user": user,
                "features_view": features_view,
            }
        ),
    )
}

pub fn settings_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    user_settings: &UserSettingsView,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/settings/settings.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "cc_cookie": cc_cookie, "user_settings": user_settings
            }
        ),
    )
}
pub fn settings_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    user_settings: &UserSettingsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/settings/settings_partial.html",
        data!({"website": website, "user": user, "user_settings": user_settings}),
    )
}

pub fn training_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    models: Vec<TrainingModelView>,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    // let models: Vec<TrainingModelView> = Vec::new();
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models.html",
        data!(
            {
                "website": website, "user": user, "credits": credits,
                "models": models, "cc_cookie": cc_cookie
            }
        ),
    )
}
pub fn training_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: UserView,
    credits: &UserCreditsView,
    models: Vec<TrainingModelView>,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models_partial.html",
        data!({ "website": website, "user": user, "credits": credits, "models": models }),
    )
}

pub fn packs_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    packs: PackViewList,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs.html",
        data!(
            {
                "website": website, "credits": credits, "packs": packs,
                "cc_cookie": cc_cookie
            }
        ),
    )
}
pub fn packs_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    packs: PackViewList,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs_partial.html",
        data!({ "website": website, "packs": packs,  "partial": true }),
    )
}

pub fn photo_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
    images: &ImageViewList,
    training_models: Vec<TrainingModelView>,
    is_deleted: bool,
    is_favorite: bool,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo.html",
        data!(
              {
                  "website": website, "user": user, "images": images,
                  "training_models": training_models, "credits": credits,
                  "is_deleted": is_deleted, "is_favorite": is_favorite,
                  "is_initial_load": true, "cc_cookie": cc_cookie
              }
        ),
    )
}
pub fn photo_partial_dashboard(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    images: &ImageViewList,
    training_models: Vec<TrainingModelView>,
    credits: &UserCreditsView,
    is_deleted: bool,
    is_favorite: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo_partial.html",
        data!(
              {
                "website": website, "images": images,
                "training_models": training_models,
                "credits": credits, "is_deleted": is_deleted,
                "is_favorite": is_favorite
            }
        ),
    )
}

#[derive(Debug, Serialize, Clone, Constructor)]
pub struct PackView {
    pub id: i32,
    pub pid: Uuid,
    pub title: String,
    pub description: String,
    pub credits: i32,
    pub amount: i32,
    pub image_url: String,
}
impl From<PackModel> for PackView {
    fn from(p: PackModel) -> Self {
        Self {
            id: p.id,
            pid: p.pid,
            title: p.title,
            description: p.description,
            credits: p.credits,
            amount: p.amount,
            image_url: p.image_url,
        }
    }
}

#[derive(Debug, Serialize, Clone, AsRef, Constructor)]
pub struct PackViewList(pub Vec<PackView>);

impl From<PackModelList> for PackViewList {
    fn from(p: PackModelList) -> Self {
        Self(p.0.into_iter().map(|x| x.into()).collect())
    }
}
