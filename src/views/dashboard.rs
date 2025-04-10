use super::auth::{UserCreditsView, UserView};
use super::images::{self, ImageViewModel};
use super::training_models::TrainingModelView;
use crate::controllers::dashboard::routes::Sidebar;
use crate::controllers::images::routes::Images;
use crate::domain::website::Website;
use crate::models::_entities::training_models::Model as TrainingModel;
use crate::{
    domain::{dashboard_sidebar::DashboardSidebar, image::Image, packs::Packs},
    models::_entities::users,
};
use loco_rs::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

pub fn billing_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    sidebar_routes: Sidebar,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/billing/billing.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
                "sidebar_routes": sidebar_routes,
                "credits": credits,
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
    sidebar_routes: Sidebar,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/account/account.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
                "sidebar_routes": sidebar_routes,
                "credits": credits,
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
    sidebar_routes: Sidebar,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/notification/notification.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
                "sidebar_routes": sidebar_routes,
                "credits": credits,
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

pub fn help_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    sidebar_routes: Sidebar,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/help/help.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
                "sidebar_routes": sidebar_routes,
                "credits": credits,
            }
        ),
    )
}

pub fn help_partial_dashboard(v: impl ViewRenderer, user: UserView) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/help/help_partial.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
            }
        ),
    )
}

pub fn settings_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    sidebar_routes: Sidebar,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/settings/settings.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
                "sidebar_routes": sidebar_routes,
                "credits": credits,
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
    models: Vec<TrainingModelView>,
    training_route_check: String,
    sidebar_routes: Sidebar,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    // let models: Vec<TrainingModelView> = Vec::new();
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models.html",
        data!(
            {
                "sidebar": sidebar, "user": user,
                "sidebar_routes": sidebar_routes,
                "credits": credits,
                "models": models,
                "training_route_check": training_route_check

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
    packs: &Vec<Packs>,
    sidebar_routes: Sidebar,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/packs/packs.html",
        data!({"sidebar": sidebar, "user": user,
        "sidebar_routes": sidebar_routes,
        "credits": credits, "packs": packs}),
    )
}

pub fn packs_partial_dashboard(
    v: impl ViewRenderer,
    user: &UserView,
    packs: &Vec<Packs>,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/packs/packs_partial.html",
        data!({"user": user, "packs": packs,  "partial": true}),
    )
}

pub fn photo_dashboard(
    v: impl ViewRenderer,
    user: &UserView,
    images: &Vec<ImageViewModel>,
    sidebar_routes: Sidebar,
    training_models: Vec<TrainingModelView>,
    website: &Website,
    credits: &UserCreditsView,
    is_deleted: Option<bool>,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
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
             "is_initial_load": true,
             "is_deleted": true
        }),
    )
}

pub fn photo_partial_dashboard(
    v: impl ViewRenderer,
    user: &UserView,
    images: &Vec<ImageViewModel>,
    training_models: Vec<TrainingModelView>,
    website: &Website,
    credits: &UserCreditsView,
    is_deleted: Option<bool>,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    format::render().view(
        &v,
        "dashboard/content/photo/photo_partial.html",
        data!({"user": user, "images": images,
            "training_models": training_models,
            "website": website, "credits": credits,
            "check_route": website.main_routes.check,
            "delete_route":  website.main_routes.image,
            "restore_route": website.main_routes.image_restore,
            "is_deleted": true
        }),
    )
}
