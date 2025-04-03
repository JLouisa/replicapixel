// use crate::controllers::dashboard::routes::Sidebar;
// use crate::domain::website::Website;
// use crate::models::_entities::training_models::Model as TrainingModel;
// use crate::models::training_models::TrainingDomain;
// use crate::models::user_credits::UserCreditsDomain;
// use crate::models::users::UserDomain;
// use crate::{
//     domain::{dashboard_sidebar::DashboardSidebar, image::Image, packs::Packs},
//     models::_entities::users,
// };
// use loco_rs::prelude::*;
// use strum::{EnumIter, IntoEnumIterator};

// pub fn billing_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     sidebar_routes: Sidebar,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/billing/billing.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "sidebar_routes": sidebar_routes,
//                 "credits": credits,
//             }
//         ),
//     )
// }

// pub fn billing_partial_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/billing/billing_partial.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//             }
//         ),
//     )
// }

// pub fn account_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     sidebar_routes: Sidebar,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/account/account.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "sidebar_routes": sidebar_routes,
//                 "credits": credits,
//             }
//         ),
//     )
// }

// pub fn account_partial_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/account/account_partial.html",
//         data!({"sidebar": sidebar, "user": user}),
//     )
// }

// pub fn notification_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     sidebar_routes: Sidebar,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/notification/notification.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "sidebar_routes": sidebar_routes,
//                 "credits": credits,
//             }
//         ),
//     )
// }

// pub fn notification_partial_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/notification/notification_partial.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//             }
//         ),
//     )
// }

// pub fn help_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     models: Vec<TrainingDomain>,
//     sidebar_routes: Sidebar,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/help/help.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "sidebar_routes": sidebar_routes,
//                 "credits": credits,
//                 "models": models
//             }
//         ),
//     )
// }

// pub fn help_partial_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     models: Vec<TrainingDomain>,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/help/help_partial.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "models": models
//             }
//         ),
//     )
// }

// pub fn settings_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     models: Vec<TrainingDomain>,
//     sidebar_routes: Sidebar,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/settings/settings.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "sidebar_routes": sidebar_routes,
//                 "credits": credits,
//                 "models": models
//             }
//         ),
//     )
// }

// pub fn settings_partial_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     models: Vec<TrainingDomain>,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/settings/settings_partial.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "credits": "1000",
//                 "models": models
//             }
//         ),
//     )
// }

// pub fn training_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     models: Vec<TrainingDomain>,
//     training_route_check: String,
//     sidebar_routes: Sidebar,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/training_models/training_models.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "sidebar_routes": sidebar_routes,
//                 "credits": credits,
//                 "models": models,
//                 "training_route_check": training_route_check

//             }
//         ),
//     )
// }

// pub fn training_partial_dashboard(
//     v: impl ViewRenderer,
//     user: UserDomain,
//     models: Vec<TrainingDomain>,
//     training_route_check: String,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/training_models/training_models_partial.html",
//         data!(
//             {
//                 "sidebar": sidebar, "user": user,
//                 "credits": "1000",
//                 "models": models,
//                 "training_route_check": training_route_check
//             }
//         ),
//     )
// }

// pub fn packs_dashboard(
//     v: impl ViewRenderer,
//     user: &UserDomain,
//     packs: &Vec<Packs>,
//     models: &Vec<&str>,
//     sidebar_routes: Sidebar,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/packs/packs.html",
//         data!({"sidebar": sidebar, "user": user,
//         "sidebar_routes": sidebar_routes,
//         "credits": credits, "packs": packs, "models": models}),
//     )
// }

// pub fn packs_partial_dashboard(
//     v: impl ViewRenderer,
//     user: &UserDomain,
//     packs: &Vec<Packs>,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/packs/packs_partial.html",
//         data!({"user": user,"credits": "1000", "packs": packs,  "partial": true}),
//     )
// }

// pub fn photo_dashboard(
//     v: impl ViewRenderer,
//     user: &UserDomain,
//     images: &Vec<Image>,
//     sidebar_routes: Sidebar,
//     training_models: Vec<TrainingDomain>,
//     website: &Website,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/photo/photo.html",
//         data!({"sidebar": &website.dashboard_sidebar, "user": user,
//             "sidebar_routes": &website.sidebar_routes,
//             "credits": credits, "images": images, "training_models": training_models,
//             "website": website
//         }),
//     )
// }

// pub fn photo_partial_dashboard(
//     v: impl ViewRenderer,
//     user: &UserDomain,
//     images: &Vec<Image>,
//     training_models: Vec<TrainingDomain>,
//     website: &Website,
//     user_credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/photo/photo_partial.html",
//         data!({"user": user,"credits": "1000", "images": images, "training_models": training_models,
//         "website": website, "credits": user_credits
//         }),
//     )
// }

// pub fn dashboard(
//     v: impl ViewRenderer,
//     user: &UserDomain,
//     images: &Vec<Image>,
//     sidebar_routes: Sidebar,
//     training_models: Vec<TrainingDomain>,
//     credits: &UserCreditsDomain,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     format::render().view(
//         &v,
//         "dashboard/content/photo/photo.html",
//         data!({"sidebar": sidebar, "user": user,
//         "sidebar_routes": sidebar_routes,
//         "credits": credits, "images": images, "training_models": training_models}),
//     )
// }
