use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;

use super::FeatureVoteActiveModel;
pub use super::_entities::feature_request;
pub use super::_entities::feature_request::{ActiveModel, Entity, Model};
use derive_more::{AsRef, Constructor};
use serde::Deserialize;
pub type FeatureRequest = Entity;
use sea_orm::QueryOrder;

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

#[derive(Debug, Clone, Deserialize, Validate, Constructor)]
pub struct FeatureParams {
    pub user_id: i32,
    #[validate(length(max = 30, message = "Name must be at least 30 characters long."))]
    pub title: String,
    #[validate(length(max = 140, message = "Name must be at least 140 characters long."))]
    pub description: String,
}
impl FeatureParams {
    pub fn update(&self, item: &mut ActiveModel) {
        item.user_id = Set(self.user_id.clone());
        item.title = Set(self.title.clone());
        item.description = Set(self.description.clone());
    }
}

#[derive(Debug, Clone, Constructor, AsRef)]
pub struct FeatureRequestModelList(pub Vec<Model>);

// implement your read-oriented logic here
impl Model {
    pub async fn load_top_10(db: &DatabaseConnection) -> ModelResult<FeatureRequestModelList> {
        let list = Entity::find()
            .order_by_desc(feature_request::Column::Votes)
            .limit(10)
            .all(db)
            .await?;
        Ok(FeatureRequestModelList::new(list))
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn toggle_vote_count(
        db: &DatabaseConnection,
        user_id: i32,
        feature_request_id: i32,
    ) -> ModelResult<(Model, bool)> {
        let txn = db.begin().await?;

        let feature_request = Entity::find_by_id(feature_request_id).one(&txn).await?;
        let feature_request = feature_request.ok_or_else(|| ModelError::EntityNotFound)?;
        let result = FeatureVoteActiveModel::toggle_vote(&txn, user_id, feature_request.id).await?;
        let votes = match result {
            true => feature_request.votes + 1,
            false => feature_request.votes.saturating_sub(1),
        };
        let mut new = feature_request.into_active_model();
        new.votes = Set(votes);
        let feature = new.update(&txn).await?;

        txn.commit().await?;
        Ok((feature, result))
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
