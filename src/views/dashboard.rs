use super::auth::{UserCreditsView, UserView};
use super::images::{ImageView, ImageViewList};
use super::training_models::TrainingModelView;
use crate::controllers::dashboard::routes::Sidebar;
use crate::domain::dashboard_sidebar::DashboardSidebar;
use crate::domain::features::FeatureViewList;
use crate::domain::website::Website;
use crate::middleware::cookie::CookieConsent;
use crate::models::packs::PackModelList;
use crate::models::PackModel;
use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use serde::Serialize;

pub fn billing_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    credits: &UserCreditsView,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/billing/billing.html",
        data!(
            {
                "sidebar":  website.dashboard_sidebar, "user": user,
                "website": website, "sidebar_routes": website.sidebar_routes,
                "credits": credits, "cc_cookie": cc_cookie
            }
        ),
    )
}

pub fn billing_partial_dashboard(
    v: impl ViewRenderer,
    user: UserView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/billing/billing_partial.html",
        data!({"sidebar": sidebar, "user": user}),
    )
}

pub fn account_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    credits: &UserCreditsView,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/account/account.html",
        data!(
            {
                "user": user, "credits": credits,
                "website": website, "sidebar":  website.dashboard_sidebar,
                "sidebar_routes": website.sidebar_routes,
                "cc_cookie": cc_cookie
            }
        ),
    )
}

pub fn account_partial_dashboard(
    v: impl ViewRenderer,
    user: UserView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/account/account_partial.html",
        data!({"sidebar": sidebar, "user": user}),
    )
}

pub fn notification_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    credits: &UserCreditsView,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/notification/notification.html",
        data!(
            {
                "user": user, "credits": credits,
                "website": website,  "sidebar":  website.dashboard_sidebar,
                "sidebar_routes": website.sidebar_routes,
                "cc_cookie": cc_cookie
            }
        ),
    )
}

pub fn notification_partial_dashboard(
    v: impl ViewRenderer,
    user: UserView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/notification/notification_partial.html",
        data!({"sidebar": sidebar, "user": user}),
    )
}

pub fn features_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    credits: &UserCreditsView,
    features_view: &FeatureViewList,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/features.html",
        data!(
            {
                "user": user, "credits": credits, "features_view": features_view,
                "website": website, "sidebar":  website.dashboard_sidebar,
                "sidebar_routes": website.sidebar_routes,
                "cc_cookie": cc_cookie
            }
        ),
    )
}

pub fn features_partial_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    features_view: &FeatureViewList,
    website: &Website,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/features/features_partial.html",
        data!(
            {
                "sidebar": website.dashboard_sidebar, "user": user,
                "features_view": features_view,
            }
        ),
    )
}

pub fn settings_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    credits: &UserCreditsView,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/settings/settings.html",
        data!(
            {
                "user": user, "credits": credits,
                "website": website, "sidebar":  website.dashboard_sidebar,
                "sidebar_routes": website.sidebar_routes,
                "cc_cookie": cc_cookie
            }
        ),
    )
}

pub fn settings_partial_dashboard(
    v: impl ViewRenderer,
    user: UserView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/settings/settings_partial.html",
        data!({"sidebar": sidebar, "user": user}),
    )
}

pub fn training_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    credits: &UserCreditsView,
    models: Vec<TrainingModelView>,
    training_route_check: String,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    // let models: Vec<TrainingModelView> = Vec::new();
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models.html",
        data!(
            {
                "user": user, "credits": credits,
                "models": models, "training_route_check": training_route_check,
                "website": website, "sidebar":  website.dashboard_sidebar,
                "sidebar_routes": website.sidebar_routes,
                "cc_cookie": cc_cookie
            }
        ),
    )
}

pub fn training_partial_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    models: Vec<TrainingModelView>,
    training_route_check: String,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models_partial.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
                "models": models,
                "training_route_check": training_route_check
            }
        ),
    )
}

pub fn packs_dashboard(
    v: impl ViewRenderer,
    user: &UserView,
    credits: &UserCreditsView,
    packs: PackViewList,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs.html",
        data!(
                {
                    "user": user, "credits": credits, "packs": packs,
                    "website": website, "sidebar":  website.dashboard_sidebar,
                    "sidebar_routes": website.sidebar_routes,
                    "cc_cookie": cc_cookie
             }
        ),
    )
}

pub fn packs_partial_dashboard(
    v: impl ViewRenderer,
    user: &UserView,
    packs: PackViewList,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/packs/packs_partial.html",
        data!({"user": user, "packs": packs,  "partial": true}),
    )
}

pub fn photo_dashboard(
    v: impl ViewRenderer,
    user: &UserView,
    credits: &UserCreditsView,
    images: &ImageViewList,
    training_models: Vec<TrainingModelView>,
    is_deleted: bool,
    is_favorite: bool,
    website: &Website,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo.html",
        data!({"sidebar": &website.dashboard_sidebar,
            "sidebar_routes": &website.sidebar_routes,
            "user": user, "images": images, "training_models": training_models,
            "website": website, "credits": credits,
            "check_route":  website.main_routes.check,
            "delete_route":  website.main_routes.image,
            "restore_route": website.main_routes.image_restore,
            "favorite_route": website.main_routes.image_favorite,
            "is_deleted": is_deleted,
            "is_favorite": is_favorite,
            "is_initial_load": true,
            "cc_cookie": cc_cookie
        }),
    )
}

pub fn photo_partial_dashboard(
    v: impl ViewRenderer,
    user: &UserView,
    images: &ImageViewList,
    training_models: Vec<TrainingModelView>,
    website: &Website,
    credits: &UserCreditsView,
    is_deleted: bool,
    is_favorite: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/photo/photo_partial.html",
        data!({"user": user, "images": images,
            "training_models": training_models,
            "website": website, "credits": credits,
            "check_route": website.main_routes.check,
            "delete_route":  website.main_routes.image,
            "restore_route": website.main_routes.image_restore,
            "favorite_route": website.main_routes.image_favorite,
            "is_deleted": is_deleted,
            "is_favorite": is_favorite
        }),
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
