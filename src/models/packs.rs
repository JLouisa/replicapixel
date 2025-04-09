pub use super::_entities::packs::{ActiveModel, Entity, Model};
use super::_entities::plans;
use sea_orm::entity::prelude::*;
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

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(plans::Column::Pid, pid.clone())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_id(db: &DatabaseConnection, id: &i32) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(plans::Column::Id, id.clone())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_all_packs(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let packs = Entity::find().all(db).await?;
        Ok(packs)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
