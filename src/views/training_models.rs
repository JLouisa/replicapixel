use crate::controllers::training_models::routes;
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::_entities::training_models::{ActiveModel, Entity, Model as TrainingModel};
use crate::models::training_models::TrainingModelList;
use crate::{
    domain::{dashboard_sidebar::DashboardSidebar, image::Image, packs::Packs},
    models::_entities::users,
};
use loco_rs::prelude::*;
use serde::Serialize;

use super::auth::UserView;
use super::images::ImageViewModel;

pub fn training_models_update(
    v: impl ViewRenderer,
    model: &TrainingModelView,
    training_route_check: &str,
) -> Result<Response> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models_loading_partial.html",
        data!({"model": model, "training_route_check": training_route_check}),
    )
}

pub fn training_models(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    let base = routes::Models::BASE;
    let check = routes::Models::CHECK_W_ID_W_STATUS;
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models.html",
        data!({"training_models_base": base, "training_models_check": check}),
    )
}

pub fn training_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    images: &Vec<ImageViewModel>,
    models: &Vec<&str>,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    let first_letter = user.name.chars().next().unwrap();
    format::render().view(
        &v,
        "dashboard/content/training.html",
        data!({"sidebar": sidebar, "user": user,"first_letter": first_letter,"credits": "1000", "images": images, "models": models}),
    )
}

pub fn training_partial_dashboard(
    v: impl ViewRenderer,
    user: UserView,
    images: &Vec<ImageViewModel>,
) -> Result<impl IntoResponse> {
    let sidebar = DashboardSidebar::init();
    let first_letter = user.name.chars().next().unwrap();
    format::render().view(
        &v,
        "dashboard/partials/training_partial.html",
        data!({"user": user,"first_letter": first_letter,"credits": "1000", "images": images}),
    )
}

// ============== View Models for the View Templates ==============
#[derive(Serialize)]
pub struct TrainingModelView {
    pub id: i32,
    pub pid: Uuid,
    pub name: String,
    pub training_status: Status,
    pub thumbnail: Option<String>,
    pub trigger_word: String,
}
impl From<&TrainingModel> for TrainingModelView {
    fn from(model: &TrainingModel) -> Self {
        Self {
            id: model.id,
            pid: model.pid,
            name: model.name.clone(),
            training_status: model.training_status,
            trigger_word: model.trigger_word.clone(),
            thumbnail: model.thumbnail.clone(),
        }
    }
}
impl From<TrainingModel> for TrainingModelView {
    fn from(model: TrainingModel) -> Self {
        Self {
            id: model.id,
            pid: model.pid,
            name: model.name,
            training_status: model.training_status,
            trigger_word: model.trigger_word,
            thumbnail: model.thumbnail,
        }
    }
}
impl From<TrainingModelList> for Vec<TrainingModelView> {
    fn from(list: TrainingModelList) -> Vec<TrainingModelView> {
        list.into_inner()
            .iter()
            .map(TrainingModelView::from)
            .collect()
    }
}
impl From<&TrainingModelList> for Vec<TrainingModelView> {
    fn from(list: &TrainingModelList) -> Vec<TrainingModelView> {
        list.clone()
            .into_inner()
            .iter()
            .map(TrainingModelView::from)
            .collect()
    }
}
