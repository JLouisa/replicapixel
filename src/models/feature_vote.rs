use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Condition};

pub use super::_entities::feature_vote::{ActiveModel, Entity, Model};
use derive_more::{AsRef, Constructor};
pub type FeatureVote = Entity;
use super::_entities::feature_vote;

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

#[derive(Debug, Clone, Constructor, AsRef)]
pub struct FeatureVoteModelList(pub Vec<Model>);

// implement your read-oriented logic here
impl Model {
    pub async fn load_all_votes(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> ModelResult<FeatureVoteModelList> {
        let condition = Condition::all().add(feature_vote::Column::UserId.eq(user_id));
        let list = Entity::find().filter(condition).all(db).await?;
        Ok(FeatureVoteModelList::new(list))
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn toggle_vote(
        db: &impl ConnectionTrait,
        user_id: i32,
        feature_request_id: i32,
    ) -> Result<bool, DbErr> {
        let condition = Condition::all()
            .add(feature_vote::Column::UserId.eq(user_id))
            .add(feature_vote::Column::FeatureRequestId.eq(feature_request_id));
        let existing = Entity::find().filter(condition).one(db).await?;
        tracing::warn!(existing = ?existing, "Does feature vote exist? Existing?");

        if let Some(model) = existing {
            tracing::warn!(model = ?model, "Does feature vote exist? Existing?");
            model.delete(db).await?;
            Ok(false) // Remove vote
        } else {
            tracing::warn!("Creating New Vote");
            let new = ActiveModel {
                user_id: Set(user_id),
                feature_request_id: Set(feature_request_id),
                ..Default::default()
            };
            dbg!(&new);
            new.insert(db).await?;
            Ok(true) // Add vote
        }
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
