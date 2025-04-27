pub use super::_entities::user_credits::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type UserCredits = Entity;
use super::{TransactionModel, UserModel, _entities::user_credits, images::ImageNewList};
use loco_rs::prelude::*;
use serde::Serialize;

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
    pub async fn load_item_by_user_id(ctx: &AppContext, user: &UserModel) -> Result<Model> {
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

impl Model {
    pub fn update(&self, item: &mut ActiveModel) {
        item.id = Set(self.id);
        item.pid = Set(self.pid.clone());
        item.user_id = Set(self.user_id.clone());
        item.credit_amount = Set(self.credit_amount.clone());
        item.model_amount = Set(self.model_amount.clone());
    }
    pub async fn load_item(&self, db: &impl ConnectionTrait) -> Result<Self> {
        let item = Entity::find_by_id(self.id).one(db).await?;
        item.ok_or_else(|| Error::NotFound).map(|item| item.into())
    }
    pub async fn load_item_by_user_id(db: &impl ConnectionTrait, user: &UserModel) -> Result<Self> {
        let item = Model::find_by_user_id(db, user.id).await?;
        Ok(item)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(credit: &Model, db: &impl ConnectionTrait) -> ModelResult<Self> {
        let item = ActiveModel {
            id: ActiveValue::set(credit.id.clone()),
            pid: ActiveValue::set(credit.pid.clone()),
            user_id: ActiveValue::set(credit.user_id.clone()),
            credit_amount: ActiveValue::set(credit.credit_amount.clone()),
            model_amount: ActiveValue::set(credit.model_amount.clone()),
            ..Default::default()
        };

        let item = item.insert(db).await?;

        Ok(item.into())
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
    pub async fn update_credits_with_transaction(
        self,
        txn: &TransactionModel,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        let new_credit_amount = txn.credit_amount.clone() + self.credit_amount.clone();
        let new_model_amount = txn.model_amount.clone() + self.model_amount.clone();
        let mut new = ActiveModel::from(self);
        new.credit_amount = ActiveValue::set(new_credit_amount);
        new.model_amount = ActiveValue::set(new_model_amount);
        let credit = new.update(db).await?;
        Ok(credit)
    }

    pub async fn update_credits_with_image_list(
        self,
        list: &ImageNewList,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        let new_credit_amount = self.credit_amount.clone() - list.as_ref().len() as i32;
        let mut new = ActiveModel::from(self);
        new.credit_amount = ActiveValue::set(new_credit_amount);
        let credit = new.update(db).await?;
        Ok(credit)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
