pub use super::_entities::packs_translations::{ActiveModel, Entity, Model};
use crate::models::_entities::{packs_translations, sea_orm_active_enums::Language};
use derive_more::{AsRef, Constructor};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{entity::prelude::*, Condition};
pub type PacksTranslations = Entity;

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

#[derive(Debug, Constructor, Clone, AsRef)]
pub struct PackTranslationModelList(pub Vec<Model>);

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_pack_id(db: &DatabaseConnection, pack_id: &i32) -> ModelResult<Self> {
        let condition =
            Condition::all().add(packs_translations::Column::PackId.eq(pack_id.to_owned()));
        let pack = Entity::find().filter(condition).one(db).await?;
        pack.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_pack_id_lang(
        db: &DatabaseConnection,
        pack_id: &i32,
        lang: &Language,
    ) -> ModelResult<Self> {
        let condition = Condition::all()
            .add(packs_translations::Column::PackId.eq(pack_id.to_owned()))
            .add(packs_translations::Column::Language.eq(lang.to_owned()));
        let pack = Entity::find().filter(condition).one(db).await?;
        pack.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let packs = Entity::find().all(db).await?;
        Ok(packs)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
