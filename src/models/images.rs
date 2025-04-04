pub use super::ImageModel;
use super::TrainingModelModel;

use super::_entities::images;
pub use super::_entities::images::{ActiveModel, Entity, Model};
use super::_entities::sea_orm_active_enums::{BasedOn, ImageFormat, ImageSize, Status};
use derive_more::{AsRef, Constructor};
use sea_orm::entity::prelude::*;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};
pub type Images = Entity;
use loco_rs::{auth::jwt, hash, prelude::*};

#[derive(Clone, Debug, Serialize)]
pub struct ImageClient {
    pub pid: Uuid,
    pub user_id: i32,
    pub training_model_id: i32,
    pub user_prompt: UserPrompt,
    pub image_size: ImageSize,
    pub image_url: Option<String>,
    pub image_url_s3: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
impl From<ImageModel> for ImageClient {
    fn from(model: ImageModel) -> Self {
        Self {
            pid: model.pid,
            user_id: model.user_id,
            training_model_id: model.training_model_id,
            user_prompt: UserPrompt(model.user_prompt),
            image_size: model.image_size,
            image_url: model.image_url,
            image_url_s3: model.image_url_s3,
            deleted_at: model.deleted_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ImageNew {
    pub pid: Uuid,
    pub user_id: i32,
    pub training_model_id: i32,
    pub pack_id: Option<i32>,
    pub user_prompt: UserPrompt,
    pub sys_prompt: SysPrompt,
    pub num_inference_steps: i32,
    pub content_type: ImageFormat,
    pub status: Status,
    pub image_size: ImageSize,
    pub fal_ai_request_id: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub image_url: Option<String>,
    pub image_url_s3: Option<String>,
    pub is_favorite: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
impl ImageNew {
    pub fn update(&self, item: &mut ActiveModel) {
        item.user_prompt = Set(self.user_prompt.as_ref().to_owned());
        item.sys_prompt = Set(self.sys_prompt.as_ref().to_owned());
        item.pid = Set(self.pid.clone());
        item.user_id = Set(self.user_id.clone());
        item.training_model_id = Set(self.training_model_id.clone());
        item.pack_id = Set((self.pack_id.clone()));
        item.num_inference_steps = Set(self.num_inference_steps.clone());
        item.content_type = Set(self.content_type.clone());
        item.status = Set(self.status);
        item.image_size = Set(self.image_size);
        item.fal_ai_request_id = Set(self.fal_ai_request_id.clone());
        item.width = Set(self.width.clone());
        item.height = Set(self.height.clone());
        item.image_url = Set(self.image_url.clone());
        item.image_url_s3 = Set(self.image_url_s3.clone());
        item.is_favorite = Set(self.is_favorite);
        item.deleted_at = Set(self.deleted_at.clone());
    }
}

#[derive(Clone, Debug, Constructor, AsRef)]
pub struct ImageNewList(Vec<ImageNew>);
impl ImageNewList {
    pub fn into_inner(self) -> Vec<ImageNew> {
        self.0
    }
    pub async fn save_all(&self, txn: &DatabaseTransaction) -> Result<(), loco_rs::Error> {
        // Bulk insert
        let models: Vec<ActiveModel> = self
            .clone()
            .into_inner()
            .iter()
            // .iter_mut()
            .map(|img| {
                let mut item = ActiveModel {
                    ..Default::default()
                };
                img.update(&mut item);
                item
            })
            .collect();
        let results = Entity::insert_many(models).exec(txn).await?;
        Ok(())
    }
}
impl From<Vec<ImageNew>> for ImageNewList {
    fn from(list: Vec<ImageNew>) -> Self {
        Self::new(list)
    }
}

#[derive(Clone, Debug, Serialize, Constructor, AsRef)]
pub struct ImageClientList(Vec<ImageClient>);
impl ImageClientList {
    pub fn into_inner(self) -> Vec<ImageClient> {
        self.0
    }
}
impl From<Vec<ImageClient>> for ImageClientList {
    fn from(list: Vec<ImageClient>) -> Self {
        Self::new(list)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, AsRef, PartialEq)]
pub struct UserPrompt(String);
impl UserPrompt {
    pub fn new<K: Into<String>>(key: K) -> Self {
        Self(key.into())
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl UserPrompt {
    pub fn formatted_prompt(&self, training_model: &TrainingModelModel) -> SysPrompt {
        let sys_prompt = format!(
            "{} {}. The subject is a {} {} {} with {} eyes, aged {}. {}.",
            training_model.trigger_word,
            self.0,
            training_model.ethnicity,
            training_model.sex,
            if training_model.bald {
                "who is bald"
            } else {
                ""
            },
            training_model.eye_color,
            training_model.age,
            training_model.based_on.to_string()
        );
        SysPrompt(sys_prompt)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, AsRef, PartialEq)]
pub struct SysPrompt(String);
impl SysPrompt {
    pub fn new<K: Into<String>>(key: K) -> Self {
        Self(key.into())
    }
    pub fn into_inner(self) -> String {
        self.0
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

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(db: &DatabaseConnection, img: &ImageModel) -> ModelResult<Self> {
        let mut item = ActiveModel {
            pid: ActiveValue::set(img.pid.clone()),
            user_id: ActiveValue::set(img.user_id.clone()),
            training_model_id: ActiveValue::set(img.training_model_id.clone()),
            pack_id: ActiveValue::set((img.pack_id.clone())),
            user_prompt: ActiveValue::set(img.user_prompt.clone()),
            sys_prompt: ActiveValue::set(img.sys_prompt.clone()),
            num_inference_steps: ActiveValue::set(img.num_inference_steps.clone()),
            content_type: ActiveValue::set(img.content_type),
            status: ActiveValue::set(img.status),
            image_size: ActiveValue::set(img.image_size),
            fal_ai_request_id: ActiveValue::set(img.fal_ai_request_id.clone()),
            width: ActiveValue::set(img.width.clone()),
            height: ActiveValue::set(img.height.clone()),
            image_url: ActiveValue::set(img.image_url.clone()),
            image_url_s3: ActiveValue::set(img.image_url_s3.clone()),
            is_favorite: ActiveValue::set(img.is_favorite),
            ..Default::default()
        };

        let item = item.insert(db).await?;

        Ok(item.into())
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(images::Column::Pid, pid.to_string())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
