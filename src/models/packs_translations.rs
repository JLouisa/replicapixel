pub use super::_entities::packs_translations::{ActiveModel, Entity, Model};
use crate::{
    models::{
        _entities::{packs_translations, sea_orm_active_enums::Language},
        packs::deserialize_comma_separated_string_array,
    },
    views::admin::PackAdmin,
};
use derive_more::{AsRef, Constructor};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{entity::prelude::*, ActiveValue, Condition, IntoActiveModel};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminPackTranslatedPayload {
    pub pack_id: i32,
    pub language: Language,
    pub title: String,
    pub short_description: String,
    pub full_description: String,
    #[serde(default, deserialize_with = "deserialize_comma_separated_string_array")]
    pub features: Vec<String>,
}
impl AdminPackTranslatedPayload {
    pub async fn save(&self, db: &DatabaseConnection) -> ModelResult<Model> {
        let packs_translations = ActiveModel::save(db, self).await?;
        Ok(packs_translations)
    }
    pub async fn update(&self, db: &DatabaseConnection) -> ModelResult<Model> {
        let mut item = Model::find_by_pack_id_lang(db, &self.pack_id, &self.language)
            .await?
            .into_active_model();
        item.title = ActiveValue::Set(self.title.clone());
        item.short_description = ActiveValue::Set(self.short_description.clone());
        item.full_description = ActiveValue::Set(self.full_description.clone());
        item.features = ActiveValue::Set(Some(self.features.clone()));
        let value = item.update(db).await?;
        Ok(value)
    }
    pub async fn upsert(&self, db: &DatabaseConnection) -> ModelResult<Model> {
        let item = match Model::find_by_pack_id_lang(db, &self.pack_id, &self.language).await {
            Ok(_) => self.update(db).await,
            Err(ModelError::EntityNotFound) => self.save(db).await,
            Err(err) => Err(err),
        };
        item
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TranslateGroupView {
    pub spanish: Option<PackTranslatedView>,
    pub german: Option<PackTranslatedView>,
    pub italian: Option<PackTranslatedView>,
    pub dutch: Option<PackTranslatedView>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackTranslatedView {
    pub pack_id: i32,
    pub language: Language,
    pub title: String,
    pub short_description: String,
    pub full_description: String,
    pub features: String,
}

impl From<Model> for PackTranslatedView {
    fn from(item: Model) -> Self {
        Self {
            pack_id: item.id,
            language: item.language,
            title: item.title,
            short_description: item.short_description,
            full_description: item.full_description,
            features: PackAdmin::convert_vec_to_str(&item.features),
        }
    }
}
impl From<AdminPackTranslatedPayload> for PackTranslatedView {
    fn from(item: AdminPackTranslatedPayload) -> Self {
        Self {
            pack_id: item.pack_id,
            language: item.language,
            title: item.title,
            short_description: item.short_description,
            full_description: item.full_description,
            features: PackAdmin::convert_vec_to_str(&Some(item.features)),
        }
    }
}

#[derive(Debug, Constructor, Clone, AsRef)]
pub struct PackTranslationModelList(pub Vec<Model>);
impl PackTranslationModelList {
    pub fn group(self) -> TranslateGroupView {
        let list = self.as_ref();
        let group = TranslateGroupView {
            spanish: list
                .iter()
                .find(|p| p.language == Language::Spanish)
                .cloned()
                .map(Into::into),
            german: list
                .iter()
                .find(|p| p.language == Language::German)
                .cloned()
                .map(Into::into),
            italian: list
                .iter()
                .find(|p| p.language == Language::Italian)
                .cloned()
                .map(Into::into),
            dutch: list
                .iter()
                .find(|p| p.language == Language::Dutch)
                .cloned()
                .map(Into::into),
        };
        group
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_pack_id(db: &DatabaseConnection, pack_id: &i32) -> ModelResult<Self> {
        let condition =
            Condition::all().add(packs_translations::Column::PackId.eq(pack_id.to_owned()));
        let pack = Entity::find().filter(condition).one(db).await?;
        pack.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_all_pack_id(
        db: &DatabaseConnection,
        id: &i32,
    ) -> ModelResult<PackTranslationModelList> {
        let condition = Condition::all().add(packs_translations::Column::PackId.eq(id.to_owned()));
        let pack = Entity::find().filter(condition).all(db).await?;
        Ok(PackTranslationModelList::new(pack))
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
impl ActiveModel {
    pub async fn save(
        db: &DatabaseConnection,
        pack: &AdminPackTranslatedPayload,
    ) -> ModelResult<Model> {
        let item = ActiveModel {
            pack_id: ActiveValue::Set(pack.pack_id),
            language: ActiveValue::Set(pack.language),
            title: ActiveValue::set(pack.title.clone()),
            short_description: ActiveValue::set(pack.short_description.clone()),
            full_description: ActiveValue::set(pack.full_description.clone()),
            features: ActiveValue::set(Some(pack.features.clone())),
            ..Default::default()
        };

        let item = item.insert(db).await?;

        Ok(item)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
