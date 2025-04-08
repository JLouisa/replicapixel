use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;

use super::_entities::handled_fal_events;
pub use super::_entities::handled_fal_events::{ActiveModel, Entity, Model};
pub type HandledFalEvents = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(db: &impl ConnectionTrait, request_id: &str) -> ModelResult<Self> {
        let mut hse = ActiveModel {
            request_id: ActiveValue::set(request_id.to_owned()),
            ..Default::default()
        };
        let hse = hse.insert(db).await?;
        Ok(hse.into())
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_request_id(
        request_id: &str,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Self> {
        let hse = Entity::find()
            .filter(
                model::query::condition()
                    .eq(handled_fal_events::Column::RequestId, request_id.to_owned())
                    .build(),
            )
            .one(db)
            .await?;
        hse.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
