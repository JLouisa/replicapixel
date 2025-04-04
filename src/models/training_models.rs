pub use super::_entities::training_models::{ActiveModel, Entity, Model};
use super::{
    TrainingModelActiveModel, TrainingModelModel, UserModel,
    _entities::{
        sea_orm_active_enums::{BasedOn, Ethnicity, EyeColor, ImageFormat, Sex, Status},
        training_models,
    },
};
use derive_more::{AsRef, Constructor};
use sea_orm::{entity::prelude::*, QueryOrder};
use serde::{Deserialize, Serialize};
pub type TrainingModels = Entity;

use crate::service::aws::s3::{AwsS3, PresignedUrlRequest, PresignedUrlSafe, S3Folders, S3Key};
use crate::service::fal_ai::fal_client::{FalAiClient, FluxLoraTrainingSchema};
use crate::{domain::image::Image, views};
use crate::{
    domain::response::{handle_general_response, handle_general_response_text},
    service::fal_ai::fal_client::FluxQueueResponse,
};
use axum::{debug_handler, Extension};
use axum::{http::HeaderMap, http::StatusCode, response::IntoResponse, Json};
use loco_rs::prelude::*;
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingModelClient {
    pub id: i32,
    pub pid: Uuid,
    pub user_id: i32,
    pub name: String,
    pub sex: Sex,
    pub age: i32,
    pub eye_color: EyeColor,
    pub bald: bool,
    pub creative: i32,
    pub based_on: BasedOn,
    pub ethnicity: Ethnicity,
    pub trigger_word: String,
    pub training_status: Status,
}

impl From<TrainingModelModel> for TrainingModelClient {
    fn from(model: TrainingModelModel) -> Self {
        Self {
            id: model.id,
            pid: model.pid,
            user_id: model.user_id,
            name: model.name,
            sex: model.sex,
            age: model.age,
            eye_color: model.eye_color,
            bald: model.bald,
            creative: 28,
            based_on: model.based_on,
            ethnicity: model.ethnicity,
            training_status: model.training_status,
            trigger_word: model.trigger_word,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TrainingForm {
    #[serde(default = "Uuid::new_v4")]
    pub pid: Uuid,
    pub name: String,
    pub sex: Sex,
    pub age: i16,
    pub eye_color: EyeColor,
    pub bald: bool,
    pub creative: i32,
    pub create_mask: bool,
    pub is_style: bool,
    pub ethnicity: Ethnicity,
    pub based_on: BasedOn,
    pub file_type: ImageFormat,
    #[serde(default)]
    pub training_status: Status,
    #[serde(default = "cuid2::slug")]
    pub slug: String,
    training_images: Option<serde_json::Value>,
    pub consent: bool,
}

impl TrainingForm {
    pub fn from_from(&self, user: &UserModel, s3_key: &S3Key) -> TrainingModelParams {
        let tw = format!("{}-{}", &self.name, &self.slug);
        TrainingModelParams {
            pid: self.pid,
            user_id: user.id,
            name: self.name.clone(),
            sex: self.sex,
            age: self.age,
            eye_color: self.eye_color,
            bald: self.bald,
            steps: self.creative,
            create_mask: self.create_mask,
            is_style: self.is_style,
            s3_key: s3_key.as_ref().to_owned(),
            based_on: self.based_on,
            ethnicity: self.ethnicity,
            is_verified: false,
            training_status: self.training_status,
            trigger_word: tw,
            fal_ai_request_id: None,
            tensor_path: None,
            thumbnail: None,
            training_images: self.training_images.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingModelParams {
    pub pid: Uuid,
    pub user_id: i32,
    pub name: String,
    pub sex: Sex,
    pub age: i16,
    pub eye_color: EyeColor,
    pub bald: bool,
    pub steps: i32,
    pub create_mask: bool,
    pub is_style: bool,
    pub based_on: BasedOn,
    pub ethnicity: Ethnicity,
    pub trigger_word: String,
    pub s3_key: String,
    pub training_status: Status,
    pub is_verified: bool,
    pub fal_ai_request_id: Option<String>,
    pub tensor_path: Option<String>,
    pub thumbnail: Option<String>,
    pub training_images: Option<serde_json::Value>,
}
impl TrainingModelParams {
    pub fn update(&self, item: &mut TrainingModelActiveModel) {
        item.pid = Set(self.pid.clone());
        item.name = Set(self.name.clone());
        item.sex = Set(self.sex.clone());
        item.age = Set(self.age.clone() as i32);
        item.eye_color = Set(self.eye_color.clone());
        item.bald = Set(self.bald.clone());
        item.steps = Set(self.steps.clone());
        item.create_mask = Set(self.create_mask.clone());
        item.is_style = Set(self.is_style.clone());
        item.ethnicity = Set(self.ethnicity.clone());
        item.based_on = Set(self.based_on.clone());
        item.is_verified = Set(self.is_verified.clone());
        item.fal_ai_request_id = Set(self.fal_ai_request_id.clone());
        item.training_status = Set(self.training_status.clone());
        item.s3_key = Set(self.s3_key.clone());
        item.trigger_word = Set(self.trigger_word.clone());
        item.tensor_path = Set(self.tensor_path.clone());
        item.thumbnail = Set(self.thumbnail.clone());
        item.training_images = Set(self.training_images.clone());
    }
}

#[derive(Clone, Debug, Constructor, AsRef)]
pub struct TrainingModelList(Vec<Model>);
impl TrainingModelList {
    pub fn into_inner(self) -> Vec<Model> {
        self.0
    }
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let training = training_models::Entity::find()
            .filter(
                model::query::condition()
                    .eq(training_models::Column::Id, id.clone())
                    .build(),
            )
            .one(db)
            .await?;
        training.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_all_by_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> ModelResult<Vec<Model>> {
        let training = training_models::Entity::find()
            .filter(training_models::Column::UserId.eq(user_id))
            .order_by_desc(training_models::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(training)
    }

    pub async fn find_all_completed_by_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> ModelResult<Vec<Model>> {
        let training = training_models::Entity::find()
            .filter(training_models::Column::UserId.eq(user_id))
            .filter(training_models::Column::TrainingStatus.eq(Status::Completed)) //? Bugged. Map lowercase in Prisma to fix
            .order_by_desc(training_models::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(training)
    }

    pub async fn find_by_pid(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let training = training_models::Entity::find()
            .filter(
                model::query::condition()
                    .eq(training_models::Column::Pid, pid.clone())
                    .build(),
            )
            .one(db)
            .await?;
        training.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_by_request_id(
        db: &DatabaseConnection,
        request_id: &Uuid,
    ) -> ModelResult<Self> {
        let training = training_models::Entity::find()
            .filter(
                model::query::condition()
                    .eq(
                        training_models::Column::FalAiRequestId,
                        request_id.to_string(),
                    )
                    .build(),
            )
            .one(db)
            .await?;
        training.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(db: &DatabaseConnection, params: &TrainingModelParams) -> ModelResult<Self> {
        let mut item = training_models::ActiveModel {
            pid: ActiveValue::set(params.pid.clone()),
            name: ActiveValue::set(params.name.clone()),
            age: ActiveValue::set(params.age.clone() as i32),
            user_id: ActiveValue::set(params.user_id.clone()),
            sex: ActiveValue::set(params.sex.clone()),
            based_on: ActiveValue::set(params.based_on.clone()),
            ethnicity: ActiveValue::set(params.ethnicity.clone()),
            eye_color: ActiveValue::set(params.eye_color.clone()),
            bald: ActiveValue::set(params.bald.clone()),
            trigger_word: ActiveValue::set(params.trigger_word.clone()),
            tensor_path: ActiveValue::set(params.tensor_path.clone()),
            thumbnail: ActiveValue::set(params.thumbnail.clone()),
            training_status: ActiveValue::set(params.training_status.clone()),
            training_images: ActiveValue::set(params.training_images.clone()),
            fal_ai_request_id: ActiveValue::set(params.fal_ai_request_id.clone()),
            s3_key: ActiveValue::set(params.s3_key.clone()),
            steps: ActiveValue::set(params.steps.clone()),
            create_mask: ActiveValue::set(params.create_mask.clone()),
            is_style: ActiveValue::set(params.is_style.clone()),
            is_verified: ActiveValue::set(params.is_verified.clone()),
            ..Default::default()
        };

        let item = item.insert(db).await?;

        Ok(item.into())
    }

    pub async fn upload_completed(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.is_verified = ActiveValue::Set(true);
        Ok(self.update(db).await?)
    }

    pub async fn update_with_fal_ai_response(
        mut self,
        db: &DatabaseConnection,
        fal: &FluxQueueResponse,
    ) -> ModelResult<Model> {
        self.fal_ai_request_id = ActiveValue::Set(Some(fal.request_id.clone()));
        self.training_status = ActiveValue::Set(Status::Training);
        Ok(self.update(db).await?)
    }

    pub async fn update_fal_ai_webhook_training(
        mut self,
        db: &DatabaseConnection,
        tensor_path: Option<String>,
        status: Status,
    ) -> ModelResult<Model> {
        self.tensor_path = ActiveValue::Set(tensor_path);
        self.training_status = ActiveValue::Set(status);
        Ok(self.update(db).await?)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
