pub use super::_entities::user_credits::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type UserCredits = Entity;
use super::{_entities::user_credits, users::User};
use loco_rs::{auth::jwt, hash, prelude::*};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub struct UserCreditsInit {
    pub pid: Uuid,
    pub model_amount: i32,
    pub credit_amount: i32,
}
impl Default for UserCreditsInit {
    fn default() -> Self {
        Self {
            pid: Uuid::new_v4(),
            model_amount: 0,
            credit_amount: 0,
        }
    }
}
impl UserCreditsInit {
    pub fn update(&self, item: &mut ActiveModel) {
        item.credit_amount = Set(self.credit_amount.clone());
        item.model_amount = Set(self.model_amount.clone());
    }
}

pub struct UserCreditsNew {
    pub pid: Uuid,
    pub user_id: i32,
    pub model_amount: i32,
    pub credit_amount: i32,
}
impl UserCreditsNew {
    pub fn new(user_id: i32) -> Self {
        let init = UserCreditsInit::default();
        Self {
            pid: init.pid,
            user_id,
            model_amount: init.model_amount,
            credit_amount: init.credit_amount,
        }
    }
    pub fn update(&self, item: &mut ActiveModel) {
        item.model_amount = Set(self.model_amount.clone());
        item.credit_amount = Set(self.credit_amount.clone());
    }
    pub async fn add(&self, user_id: i32, ctx: &AppContext) -> Result<UserCreditsClient> {
        let init = UserCreditsNew::new(user_id);
        let mut item = ActiveModel {
            ..Default::default()
        };

        item.pid = Set(init.pid);
        item.user_id = Set(user_id);
        item.model_amount = Set(init.model_amount);
        item.credit_amount = Set(init.credit_amount);

        let item = item.insert(&ctx.db).await?;
        Ok(item.into())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UserCreditsClient {
    pub id: i32,
    pub pid: Uuid,
    pub user_id: i32,
    pub model_amount: i32,
    pub credit_amount: i32,
}
impl UserCreditsClient {
    pub fn update(&self, item: &mut ActiveModel) {
        item.id = Set(self.id);
        item.pid = Set(self.pid.clone());
        item.user_id = Set(self.user_id.clone());
        item.credit_amount = Set(self.credit_amount.clone());
        item.model_amount = Set(self.model_amount.clone());
    }
    pub async fn load_item(&self, ctx: &AppContext) -> Result<Self> {
        let item = Entity::find_by_id(self.id).one(&ctx.db).await?;
        item.ok_or_else(|| Error::NotFound).map(|item| item.into())
    }
    pub async fn load_item_by_user_id(ctx: &AppContext, user: &User) -> Result<Model> {
        let item = Model::find_by_user_id(&ctx.db, user.id).await?;
        Ok(item)
    }
    pub async fn save(&self, ctx: &AppContext) -> Result<Self> {
        let mut item = ActiveModel {
            ..Default::default()
        };
        self.update(&mut item);
        let item = item.update(&ctx.db).await?;
        Ok(item.into())
    }
}
impl From<Model> for UserCreditsClient {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            pid: item.pid,
            user_id: item.user_id,
            credit_amount: item.credit_amount,
            model_amount: item.model_amount,
        }
    }
}

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
    pub async fn find_by_user_id(db: &impl ConnectionTrait, user_id: i32) -> ModelResult<Model> {
        let user_credits = user_credits::Entity::find()
            .filter(
                model::query::condition()
                    .eq(user_credits::Column::UserId, user_id)
                    .build(),
            )
            .one(db)
            .await?;
        user_credits.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
