use std::collections::HashMap;

use crate::models::{
    PackTranslationModel, _entities::sea_orm_active_enums::Language,
    join::packs::load_pack_and_all_translated, packs_translations::PackTranslationModelList,
};

pub use super::_entities::packs::{ActiveModel, Entity, Model};
use super::{
    PackModel,
    _entities::{packs, sea_orm_active_enums::ImageSize},
};
use derive_more::{AsRef, Constructor, Debug};
use sea_orm::{entity::prelude::*, Condition, QueryOrder, QuerySelect};
pub type Packs = Entity;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, AsRef, Constructor)]
pub struct PackModelList(Vec<PackModel>);
impl PackModelList {
    pub fn into_inner(self) -> Vec<PackModel> {
        self.0
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PackDomain {
    pub id: i32,
    pub pid: Uuid,
    pub title: String,
    pub pack_prompts: String,
    pub short_description: String,
    pub full_description: String,
    pub credits: i32,
    pub num_images: i32,
    pub num_inference_steps: i32,
    pub main_image: String,
    pub image_size: ImageSize,
}
impl PackDomain {
    pub fn from_model(packs: PackModel, image_size: ImageSize) -> Self {
        Self {
            id: packs.id,
            pid: packs.pid,
            title: packs.title,
            pack_prompts: packs.pack_prompts,
            short_description: packs.short_description,
            full_description: packs.full_description,
            credits: packs.credits,
            num_images: packs.num_images,
            num_inference_steps: packs.num_inference_steps,
            main_image: packs.main_image,
            image_size,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PackTranslated {
    pub id: i32,
    pub pid: Uuid,
    pub pack_prompts: String,
    pub credits: i32,
    pub num_images: i32,
    pub num_inference_steps: i32,
    pub images: Option<Vec<String>>,
    pub main_image: String,
    pub used: i32,
    pub stars: i32,
    pub popular: bool,
    pub title_url: String,
    pub title: String,
    pub full_description: String,
    pub short_description: String,
    pub features: Option<Vec<String>>,
}
impl PackTranslated {
    pub fn from_model(packs: PackModel) -> Self {
        Self {
            id: packs.id,
            pid: packs.pid,
            pack_prompts: packs.pack_prompts,
            credits: packs.credits,
            num_images: packs.num_images,
            num_inference_steps: packs.num_inference_steps,
            images: packs.images,
            main_image: packs.main_image,
            used: packs.used,
            stars: packs.stars,
            popular: packs.popular,
            title_url: packs.title_url,
            title: packs.title,
            full_description: packs.full_description,
            short_description: packs.short_description,
            features: packs.features,
        }
    }
}
impl PackTranslated {
    pub fn translate(pack: Model, translation: PackTranslationModel) -> Self {
        Self {
            id: pack.id,
            pid: pack.pid,
            pack_prompts: pack.pack_prompts,
            credits: pack.credits,
            num_images: pack.num_images,
            num_inference_steps: pack.num_inference_steps,
            images: pack.images,
            main_image: pack.main_image,
            used: pack.used,
            stars: pack.stars,
            popular: pack.popular,
            title_url: pack.title_url,
            title: translation.title,
            full_description: translation.full_description,
            short_description: translation.short_description,
            features: translation.features,
        }
    }
}

#[derive(Debug, Constructor, Serialize, Clone)]
pub struct PackTranslatedList(pub Vec<PackTranslated>);
impl PackTranslatedList {
    pub fn translate(
        packs: PackModelList,
        translations: PackTranslationModelList,
        lang: &Language,
    ) -> Self {
        if lang == &Language::English {
            return packs.into();
        }
        let language = lang.clone();
        let mut translation_map = HashMap::new();
        for translation in translations.0 {
            if translation.language == language {
                translation_map.insert(translation.pack_id, translation);
            }
        }

        let mut list = Vec::with_capacity(packs.as_ref().len());
        for plan in packs.0 {
            if let Some(translation) = translation_map.get(&plan.id) {
                list.push(PackTranslated::translate(plan, (*translation).clone()));
            }
        }

        Self(list)
    }
    pub fn from_related(
        packs_with_translations: Vec<(PackModel, Vec<PackTranslationModel>)>,
        lang: &Language,
    ) -> Self {
        let pack_list = packs_with_translations
            .into_iter()
            .map(|(pack, translations)| {
                if *lang == Language::English {
                    return PackTranslated::from_model(pack);
                }
                let target_translation = translations
                    .iter()
                    .find(|translation| translation.language == *lang);

                match target_translation {
                    Some(translation) => PackTranslated::translate(pack, translation.clone()),
                    None => PackTranslated::from_model(pack),
                }
            })
            .collect();

        Self(pack_list)
    }
}

impl From<PackModelList> for PackTranslatedList {
    fn from(packs: PackModelList) -> Self {
        Self(
            packs
                .0
                .iter()
                .map(|p| PackTranslated::from_model(p.clone()))
                .collect(),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePackPayload {
    #[serde(default = "Uuid::new_v4")]
    pub pid: Uuid,
    pub title: String,
    pub title_url: String,
    pub short_description: String,
    pub full_description: String,
    pub pack_prompts: String,
    pub credits: i32,
    pub num_images: i32,
    #[serde(default = "default_num_inference_steps")]
    pub num_inference_steps: i32,
    #[serde(default = "default_stars")]
    pub stars: i32,
    #[serde(default)]
    pub popular: bool,
    pub main_image: String,
    #[serde(default, deserialize_with = "deserialize_comma_separated_string_array")]
    pub images: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_comma_separated_string_array")]
    pub features: Vec<String>,
}
impl CreatePackPayload {
    pub async fn save(&self, db: &DatabaseConnection) -> ModelResult<PackModel> {
        let pack = ActiveModel::save(db, self).await?;
        Ok(pack)
    }
    pub fn update(&self, item: &mut ActiveModel) {
        item.pid = Set(self.pid.clone());
        item.title = Set(self.title.clone());
        item.title_url = Set(self.title_url.clone());
        item.short_description = Set(self.short_description.clone());
        item.full_description = Set(self.full_description.clone());
        item.pack_prompts = Set(self.pack_prompts.clone());
        item.credits = Set(self.credits.clone());
        item.num_images = Set(self.num_images);
        item.num_inference_steps = Set(self.num_inference_steps.clone());
        item.stars = Set(self.stars.clone());
        item.popular = Set(self.popular.clone());
        item.main_image = Set(self.main_image.clone());
        item.images = Set(Some(self.images.clone()));
        item.features = Set(Some(self.features.clone()));
    }
    /// Sanitizes the `title_url` field in-place.
    /// If `title_url` is empty or becomes empty after sanitization,
    /// it attempts to generate it from the `title` field.
    pub fn sanitize_title_url_in_place(&mut self) {
        let mut sanitized = self
            .title_url
            .trim()
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>();

        // If title_url was empty or only special chars, try to use title
        if sanitized.is_empty() && !self.title.is_empty() {
            sanitized = self
                .title
                .trim()
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                .collect::<String>();
        }
        let new_main_image = self.main_image.trim().to_lowercase();
        self.title_url = sanitized;
        self.main_image = new_main_image;
    }
}
fn default_num_inference_steps() -> i32 {
    50
}
fn default_stars() -> i32 {
    5
}
fn deserialize_comma_separated_string_array<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(s.split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect())
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_all_translated(
        db: &DatabaseConnection,
        lang: &Language,
    ) -> ModelResult<PackTranslatedList> {
        if *lang == Language::English {
            tracing::info!("English packs requested, loading from DB.");
            return Ok(PackModelList::new(Model::find_all_packs(db).await?).into());
        }
        let packs_translated = load_pack_and_all_translated(db, lang).await?;
        Ok(packs_translated)
    }
    pub async fn find_by_title_url(db: &DatabaseConnection, title_url: &str) -> ModelResult<Self> {
        let condition = Condition::all().add(packs::Column::TitleUrl.eq(title_url.to_owned()));
        let pack = Entity::find().filter(condition).one(db).await?;
        pack.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let condition = Condition::all().add(packs::Column::Pid.eq(pid.clone()));
        let pack = Entity::find().filter(condition).one(db).await?;
        pack.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_id(db: &DatabaseConnection, id: &i32) -> ModelResult<Self> {
        let condition = Condition::all().add(packs::Column::Id.eq(id.to_owned()));
        let pack = Entity::find().filter(condition).one(db).await?;
        pack.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_all_packs(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let packs = Entity::find()
            .order_by_asc(packs::Column::Id)
            .all(db)
            .await?;
        Ok(packs)
    }
    pub async fn plus_used_one_pack(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let condition = Condition::all().add(packs::Column::Pid.eq(pid.clone()));
        let pack = Entity::find()
            .filter(condition)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        Entity::update(ActiveModel {
            id: ActiveValue::Unchanged(pack.id),
            used: ActiveValue::Set(pack.used + 1),
            ..pack.into_active_model()
        })
        .exec(db)
        .await
        .map_err(Into::into)
    }
    pub async fn find_first_12_packs(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let packs = Entity::find()
            .order_by_asc(packs::Column::Id)
            .limit(12)
            .all(db)
            .await?;
        Ok(packs)
    }
    pub async fn update_pack_admin(
        self,
        pack: &CreatePackPayload,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        let mut item = ActiveModel::from(self);
        pack.update(&mut item);
        let pack = item.update(db).await?;
        Ok(pack)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(db: &DatabaseConnection, pack: &CreatePackPayload) -> ModelResult<Model> {
        let item = ActiveModel {
            pid: ActiveValue::set(pack.pid.clone()),
            title: ActiveValue::set(pack.title.clone()),
            title_url: ActiveValue::set(pack.title_url.clone()),
            short_description: ActiveValue::set(pack.short_description.clone()),
            full_description: ActiveValue::set(pack.full_description.clone()),
            pack_prompts: ActiveValue::set(pack.pack_prompts.clone()),
            credits: ActiveValue::set(pack.credits.clone()),
            num_images: ActiveValue::set(pack.num_images),
            num_inference_steps: ActiveValue::set(pack.num_inference_steps.clone()),
            stars: ActiveValue::set(pack.stars.clone()),
            popular: ActiveValue::set(pack.popular.clone()),
            main_image: ActiveValue::set(pack.main_image.clone()),
            images: ActiveValue::set(Some(pack.images.clone())),
            features: ActiveValue::set(Some(pack.features.clone())),
            ..Default::default()
        };

        let item = item.insert(db).await?;

        Ok(item)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
