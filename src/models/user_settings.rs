use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, Condition};

pub use super::_entities::user_settings::{ActiveModel, Entity, Model};
use super::_entities::{sea_orm_active_enums::ThemePreference, user_settings};
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
    // pub fn update(&self, item: &mut ActiveModel) {
    //     item.id = Set(self.id);
    //     item.user_id = Set(self.user_id.clone());
    //     item.enable_notification_email = Set(self.enable_notification_email.clone());
    //     item.enable_marketing_email = Set(self.enable_marketing_email.clone());
    //     item.language = Set(self.language.clone());
    //     item.theme = Set(self.theme.clone());
    // }
    pub async fn load_item(&self, db: &impl ConnectionTrait) -> Result<Self> {
        let item = Entity::find_by_id(self.id).one(db).await?;
        item.ok_or_else(|| Error::NotFound).map(|item| item.into())
    }
    pub async fn load_item_by_user_id(db: &impl ConnectionTrait, user: &Model) -> Result<Self> {
        let item = Model::find_by_user_id(db, user.id).await?;
        Ok(item)
    }
    pub async fn find_by_user_id(db: &impl ConnectionTrait, user_id: i32) -> ModelResult<Model> {
        let condition = Condition::all().add(user_settings::Column::UserId.eq(user_id));
        let user_settings = Entity::find().filter(condition).one(db).await?;
        user_settings.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn toggle_marketing_settings(
        self,
        db: &impl ConnectionTrait,
        new_bool: bool,
    ) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.enable_marketing_email = ActiveValue::set(new_bool);
        let settings = new.update(db).await?;
        Ok(settings)
    }
    pub async fn toggle_email_settings(
        self,
        db: &impl ConnectionTrait,
        new_bool: bool,
    ) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.enable_notification_email = ActiveValue::set(new_bool);
        let settings = new.update(db).await?;
        Ok(settings)
    }
    pub async fn toggle_dark_mode_settings(
        self,
        db: &impl ConnectionTrait,
        new_bool: ThemePreference,
    ) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.theme = ActiveValue::set(new_bool);
        let settings = new.update(db).await?;
        Ok(settings)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
