use crate::domain::website::Website;
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::_entities::training_models::Model as TrainingModel;
use crate::models::training_models::TrainingModelList;
use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use serde::Serialize;

pub fn training_models_update(
    v: impl ViewRenderer,
    website: &Website,
    model: &TrainingModelView,
) -> Result<Response> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models_loading_partial.html",
        data!({ "website": website, "model": model }),
    )
}

pub fn training_models(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/training_models/training_models.html",
        data!({ "website": website }),
    )
}

// ============== View Models for the View Templates ==============
#[derive(Serialize, Clone, Debug)]
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
#[derive(Clone, Serialize, Debug, Constructor, AsRef)]
pub struct TrainingModelViewList(Vec<TrainingModelView>);
impl TrainingModelViewList {
    pub fn into_inner(self) -> Vec<TrainingModelView> {
        self.0
    }
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}
impl From<TrainingModelList> for TrainingModelViewList {
    fn from(list: TrainingModelList) -> Self {
        Self(
            list.into_inner()
                .iter()
                .map(TrainingModelView::from)
                .collect(),
        )
    }
}

// impl From<TrainingModelList> for Vec<TrainingModelView> {
//     fn from(list: TrainingModelList) -> Vec<TrainingModelView> {
//         list.into_inner()
//             .iter()
//             .map(TrainingModelView::from)
//             .collect()
//     }
// }
// impl From<&TrainingModelList> for Vec<TrainingModelView> {
//     fn from(list: &TrainingModelList) -> Vec<TrainingModelView> {
//         list.clone()
//             .into_inner()
//             .iter()
//             .map(TrainingModelView::from)
//             .collect()
//     }
// }
