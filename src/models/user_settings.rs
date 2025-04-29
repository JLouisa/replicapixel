use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, Condition};

use super::_entities::user_settings;
pub use super::_entities::user_settings::{ActiveModel, Entity, Model};
use loco_rs::model::ModelError;
pub type UserSettings = Entity;

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
    pub async fn find_by_user_id(db: &impl ConnectionTrait, user_id: i32) -> ModelResult<Model> {
        let condition = Condition::all().add(user_settings::Column::UserId.eq(user_id));
        let user_settings = Entity::find().filter(condition).one(db).await?;
        user_settings.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
