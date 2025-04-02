use crate::models::_entities::sea_orm_active_enums::{BasedOn, Emotion, Ethnicity, EyeColor, Sex};
use axum::extract::rejection::FailedToDeserializeQueryString;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardSidebar {
    create_model: CreateModel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateModel {
    type_model: Vec<BasedOn>,
    ethnicity: Vec<Ethnicity>,
    eye_color: Vec<EyeColor>,
    emotion: Vec<Emotion>,
    sex: Vec<Sex>,
}

impl DashboardSidebar {
    pub fn init() -> Self {
        Self {
            create_model: CreateModel {
                type_model: BasedOn::iter().collect(),
                sex: Sex::iter().collect(),
                ethnicity: Ethnicity::iter().collect(),
                eye_color: EyeColor::iter().collect(),
                emotion: Emotion::iter().collect(),
            },
        }
    }
}
