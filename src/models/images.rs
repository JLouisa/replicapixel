use crate::controllers::images::ImageLoadingParams;
use crate::domain::url::Url;
use crate::service::aws::s3::S3Key;

use super::users::{User, UserPid};
pub use super::ImageModel;
use super::TrainingModelModel;

use super::_entities::images;
pub use super::_entities::images::{ActiveModel, Entity, Model};
use super::_entities::sea_orm_active_enums::{ImageFormat, ImageSize, Status};
use derive_more::{AsRef, Constructor};
use sea_orm::entity::prelude::*;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
pub type Images = Entity;
use loco_rs::prelude::*;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ImageNew {
    pub pid: Uuid,
    pub user_id: i32,
    pub training_model_id: i32,
    pub pack_id: Option<i32>,
    pub user_prompt: UserPrompt,
    pub sys_prompt: SysPrompt,
    pub alt: AltText,
    pub num_inference_steps: i32,
    pub content_type: ImageFormat,
    pub status: Status,
    pub image_size: ImageSize,
    pub fal_ai_request_id: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub image_s3_key: S3Key,
    pub image_url_fal: Option<String>,
    pub is_favorite: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
impl ImageNew {
    pub fn update(&self, item: &mut ActiveModel) {
        item.pid = Set(self.pid.clone());
        item.user_id = Set(self.user_id.clone());
        item.training_model_id = Set(self.training_model_id.clone());
        item.user_prompt = Set(self.user_prompt.as_ref().to_owned());
        item.sys_prompt = Set(self.sys_prompt.as_ref().to_owned());
        item.alt = Set(self.alt.as_ref().to_owned());
        item.pack_id = Set(self.pack_id.clone());
        item.num_inference_steps = Set(self.num_inference_steps.clone());
        item.content_type = Set(self.content_type.clone());
        item.status = Set(self.status);
        item.image_size = Set(self.image_size);
        item.fal_ai_request_id = Set(self.fal_ai_request_id.clone());
        item.width = Set(self.width.clone());
        item.height = Set(self.height.clone());
        item.image_url_fal = Set(self.image_url_fal.clone());
        item.image_s3_key = Set(self.image_s3_key.as_ref().to_string());
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
    pub async fn save_all(&self, txn: &DatabaseTransaction) -> Result<(), DbErr> {
        let models: Vec<ActiveModel> = self
            .clone()
            .into_inner()
            .iter()
            .map(|img| {
                let mut item = ActiveModel {
                    ..Default::default()
                };
                img.update(&mut item);
                item
            })
            .collect();
        Entity::insert_many(models).exec(txn).await?;
        Ok(())
    }
}
impl From<Vec<ImageNew>> for ImageNewList {
    fn from(list: Vec<ImageNew>) -> Self {
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

#[derive(Clone, Debug, Serialize, Deserialize, AsRef, PartialEq)]
pub struct AltText(String);
impl AltText {
    pub fn new<K: Into<String>>(key: K) -> Self {
        Self(key.into())
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    fn truncate_with_ellipsis(text: &str) -> Self {
        let max_len: usize = 15;
        let text = if text.chars().count() > max_len {
            // String is longer than max_len: take first max_len chars and add "..."
            let truncated: String = text.chars().take(max_len).collect();
            format!("{}...", truncated)
        } else {
            // String is max_len or shorter: return the whole string
            text.to_string()
        };
        Self::new(text)
    }
}
impl From<UserPrompt> for AltText {
    fn from(value: UserPrompt) -> Self {
        Self::truncate_with_ellipsis(value.as_ref())
    }
}
impl From<&UserPrompt> for AltText {
    fn from(value: &UserPrompt) -> Self {
        Self::truncate_with_ellipsis(value.as_ref())
    }
}

#[derive(Clone, Debug, Constructor, AsRef)]
pub struct ImagesModelList(Vec<Model>);
impl ImagesModelList {
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

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(db: &DatabaseConnection, img: &ImageModel) -> ModelResult<Self> {
        let item = ActiveModel {
            pid: ActiveValue::set(img.pid.clone()),
            user_id: ActiveValue::set(img.user_id.clone()),
            training_model_id: ActiveValue::set(img.training_model_id.clone()),
            pack_id: ActiveValue::set(img.pack_id.clone()),
            user_prompt: ActiveValue::set(img.user_prompt.clone()),
            sys_prompt: ActiveValue::set(img.sys_prompt.clone()),
            alt: ActiveValue::set(img.alt.clone()),
            num_inference_steps: ActiveValue::set(img.num_inference_steps.clone()),
            content_type: ActiveValue::set(img.content_type),
            status: ActiveValue::set(img.status),
            image_size: ActiveValue::set(img.image_size),
            fal_ai_request_id: ActiveValue::set(img.fal_ai_request_id.clone()),
            width: ActiveValue::set(img.width.clone()),
            height: ActiveValue::set(img.height.clone()),
            image_url_fal: ActiveValue::set(img.image_url_fal.clone()),
            image_s3_key: ActiveValue::set(img.image_s3_key.clone()),
            is_favorite: ActiveValue::set(img.is_favorite),
            ..Default::default()
        };

        let item = item.insert(db).await?;

        Ok(item.into())
    }
    pub async fn upload_s3_completed(
        mut self,
        key: &S3Key,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        self.status = ActiveValue::Set(Status::Completed);
        Ok(self.update(db).await?)
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn get_next_20_images_after(
        db: &DatabaseConnection,
        user_id: i32,
        anchor_image_pid: &Uuid,
        num: u64,
        params: ImageLoadingParams,
    ) -> ModelResult<Vec<Self>> {
        // Fetch the anchor image first (optional, if needed)
        let anchor_image = Entity::find()
            .filter(images::Column::UserId.eq(user_id))
            .filter(images::Column::Pid.eq(anchor_image_pid.clone()))
            .one(db)
            .await?;

        if let Some(anchor) = anchor_image {
            let mut query = Entity::find()
                .filter(images::Column::UserId.eq(user_id))
                .filter(images::Column::Id.lt(anchor.id))
                .order_by_desc(images::Column::Id);

            if params.favorite.is_some() && params.deleted.is_none() {
                query = query
                    .filter(images::Column::IsFavorite.eq(true))
                    .filter(images::Column::DeletedAt.is_null())
                    .order_by_desc(images::Column::Id)
            } else if params.deleted.is_some() && params.favorite.is_none() {
                query = query
                    .filter(images::Column::DeletedAt.is_not_null())
                    .order_by_desc(images::Column::DeletedAt)
            } else {
                query = query
                    .filter(images::Column::DeletedAt.is_null())
                    .order_by_desc(images::Column::Id)
            }
            let next_images = query.limit(num).all(db).await?;
            Ok(next_images)
        }
        //===================================Old Stable Query===========================================
        // if let Some(anchor) = anchor_image {
        //     let mut next_images = Entity::find()
        //         .filter(images::Column::UserId.eq(user_id))
        //         .filter(images::Column::Id.lt(anchor.id))
        //         .filter(images::Column::DeletedAt.is_null())
        //         .limit(num)
        //         .order_by_desc(images::Column::Id)
        //         .all(db)
        //         .await?;
        //     Ok(next_images)
        // }
        else {
            Err(ModelError::EntityNotFound)
        }
    }
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(images::Column::Pid, pid.clone())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_x_images_by_user_id(
        db: &DatabaseConnection,
        id: i32,
        is_favorite: bool,
        is_deleted: bool,
        num: u64,
    ) -> ModelResult<Vec<Self>> {
        let mut condition = Condition::all().add(images::Column::UserId.eq(id));

        if is_deleted {
            condition = condition.add(images::Column::DeletedAt.is_not_null());
        } else {
            condition = condition.add(images::Column::DeletedAt.is_null());
            if is_favorite {
                condition = condition.add(images::Column::IsFavorite.eq(true));
            }
        }

        let order_column = if is_deleted {
            images::Column::DeletedAt
        } else {
            images::Column::UpdatedAt
        };

        let results = Entity::find()
            .filter(condition)
            .limit(num)
            .order_by_desc(order_column)
            .all(db)
            .await?;

        Ok(results)
    }

    pub async fn find_all_by_user_id(
        db: &DatabaseConnection,
        id: i32,
        fav: bool,
    ) -> ModelResult<Vec<Self>> {
        let mut condition = Condition::all()
            .add(images::Column::UserId.eq(id))
            .add(images::Column::DeletedAt.is_null());
        if fav {
            condition = condition.add(images::Column::IsFavorite.eq(true));
        }
        let results = Entity::find()
            .filter(condition)
            .order_by_desc(images::Column::UpdatedAt)
            .all(db)
            .await?;
        Ok(results)
    }
    pub async fn find_all_del_by_user_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> ModelResult<Vec<Self>> {
        let condition = Condition::all()
            .add(images::Column::UserId.eq(id))
            .add(images::Column::DeletedAt.is_not_null());
        let list = Entity::find()
            .filter(condition)
            .order_by_desc(images::Column::DeletedAt)
            .all(db)
            .await?;
        Ok(list)
    }
    pub async fn update_fal_image_url(
        self,
        url: &Url,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.image_url_fal = ActiveValue::set(Some(url.as_ref().to_owned()));
        new.status = ActiveValue::set(Status::Processing);
        let image = new.update(db).await?;
        Ok(image)
    }
    pub async fn favorite_image_toggle(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let time = chrono::Utc::now().into();
        let new_bool = !self.is_favorite;
        let mut new = ActiveModel::from(self);
        new.is_favorite = ActiveValue::set(new_bool);
        new.updated_at = ActiveValue::set(time);
        let image = new.update(db).await?;
        Ok(image)
    }
    pub async fn delete_image(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let time = chrono::Utc::now().into();
        let mut new = ActiveModel::from(self);
        new.deleted_at = ActiveValue::set(Some(time));
        new.updated_at = ActiveValue::set(time);
        let image = new.update(db).await?;
        Ok(image)
    }
    pub async fn restore_image(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.updated_at = ActiveValue::set(chrono::Utc::now().into());
        new.deleted_at = ActiveValue::set(None);
        let image = new.update(db).await?;
        Ok(image)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
