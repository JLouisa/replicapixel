// use crate::controllers::training_models::routes;
// use crate::models::_entities::training_models::{ActiveModel, Entity, Model as TrainingModel};
// use crate::models::training_models::TrainingDomain;
// use crate::{
//     domain::{dashboard_sidebar::DashboardSidebar, image::Image, packs::Packs},
//     models::_entities::users,
// };
// use loco_rs::prelude::*;

// pub fn training_models_update(
//     v: impl ViewRenderer,
//     model: &TrainingDomain,
//     training_route_check: &str,
// ) -> Result<Response> {
//     format::render().view(
//         &v,
//         "dashboard/content/training_models/training_models_loading_partial.html",
//         data!({"model": model, "training_route_check": training_route_check}),
//     )
// }

// pub fn training_models(v: impl ViewRenderer) -> Result<impl IntoResponse> {
//     let base = routes::Models::BASE;
//     let check = routes::Models::CHECK_W_ID_W_STATUS;
//     format::render().view(
//         &v,
//         "dashboard/content/training_models/training_models.html",
//         data!({"training_models_base": base, "training_models_check": check}),
//     )
// }

// pub fn training_dashboard(
//     v: impl ViewRenderer,
//     user: &users::Model,
//     images: &Vec<Image>,
//     models: &Vec<&str>,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     let first_letter = user.name.chars().next().unwrap();
//     format::render().view(
//         &v,
//         "dashboard/content/training.html",
//         data!({"sidebar": sidebar, "user": user,"first_letter": first_letter,"credits": "1000", "images": images, "models": models}),
//     )
// }

// pub fn training_partial_dashboard(
//     v: impl ViewRenderer,
//     user: &users::Model,
//     images: &Vec<Image>,
// ) -> Result<impl IntoResponse> {
//     let sidebar = DashboardSidebar::init();
//     let first_letter = user.name.chars().next().unwrap();
//     format::render().view(
//         &v,
//         "dashboard/partials/training_partial.html",
//         data!({"user": user,"first_letter": first_letter,"credits": "1000", "images": images}),
//     )
// }
