use super::_entities::handled_stripe_events;
pub use super::_entities::handled_stripe_events::{ActiveModel, Entity, Model};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type HandledStripeEvents = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(session_id: &str, db: &impl ConnectionTrait) -> ModelResult<Self> {
        let mut hse = ActiveModel {
            session_id: ActiveValue::set(session_id.to_owned()),
            ..Default::default()
        };
        let hse = hse.insert(db).await?;
        Ok(hse.into())
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_session_id(
        session_id: &str,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Self> {
        let hse = Entity::find()
            .filter(
                model::query::condition()
                    .eq(
                        handled_stripe_events::Column::SessionId,
                        session_id.to_owned(),
                    )
                    .build(),
            )
            .one(db)
            .await?;
        hse.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
