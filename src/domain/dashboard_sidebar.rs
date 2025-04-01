use super::active_enums::{Emotion, Ethnicity, EyeColor, Sex, TypeModel};
use axum::extract::rejection::FailedToDeserializeQueryString;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardSidebar {
    create_model: CreateModel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateModel {
    type_model: Vec<TypeModel>,
    ethnicity: Vec<Ethnicity>,
    eye_color: Vec<EyeColor>,
    emotion: Vec<Emotion>,
    sex: Vec<Sex>,
}

impl DashboardSidebar {
    pub fn init() -> Self {
        Self {
            create_model: CreateModel {
                type_model: TypeModel::iter().collect(),
                sex: Sex::iter().collect(),
                ethnicity: Ethnicity::iter().collect(),
                eye_color: EyeColor::iter().collect(),
                emotion: Emotion::iter().collect(),
            },
        }
    }
}
