pub use super::_entities::packs::{ActiveModel, Entity, Model};
use super::{
    PackModel,
    _entities::{packs, sea_orm_active_enums::ImageSize},
};
use derive_more::{AsRef, Constructor};
use sea_orm::{entity::prelude::*, Condition, QueryOrder, QuerySelect};
pub type Packs = Entity;
use loco_rs::prelude::*;

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
pub struct PackModelList(pub Vec<PackModel>);

#[derive(Debug, Clone)]
pub struct PacksDomain {
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
impl PacksDomain {
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

// implement your read-oriented logic here
impl Model {
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
    pub async fn find_first_12_packs(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let packs = Entity::find()
            .order_by_asc(packs::Column::Id)
            .limit(12)
            .all(db)
            .await?;
        Ok(packs)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
