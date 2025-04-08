pub use super::_entities::transactions::{ActiveModel, Entity, Model};
use super::{
    PlanModel, TransactionActiveModel, TransactionModel, UserModel,
    _entities::{sea_orm_active_enums::Status, transactions},
};
use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, ActiveValue};
use stripe::Currency;
pub type Transactions = Entity;

#[derive(Debug, Clone)]
pub struct TransactionDomain {
    pub pid: Uuid,
    pub user_id: i32,
    pub plan_id: i32,
    pub credit_amount: i32,
    pub model_amount: i32,
    pub currency: String,
    pub payment_id: String,
    pub status: Status,
}
impl TransactionDomain {
    pub fn new(
        pid: uuid::Uuid,
        user: &UserModel,
        plan: PlanModel,
        currency: Option<Currency>,
        payment_id: String,
    ) -> Self {
        Self {
            pid,
            user_id: user.id,
            plan_id: plan.id,
            credit_amount: plan.credit_amount,
            model_amount: plan.model_amount,
            currency: match currency {
                Some(info) => info.to_string(),
                None => Currency::USD.to_string(),
            },
            payment_id,
            status: Status::default(),
        }
    }
    pub fn update(&self, item: &mut TransactionActiveModel) {
        item.pid = Set(self.pid.clone());
        item.plan_id = Set(self.plan_id.clone());
        item.credit_amount = Set(self.credit_amount.clone());
        item.model_amount = Set(self.model_amount.clone() as i32);
        item.payment_id = Set(self.payment_id.clone());
        item.status = Set(self.status.clone());
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

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(db: &impl ConnectionTrait, item: &TransactionDomain) -> ModelResult<Self> {
        let mut transaction = ActiveModel {
            pid: ActiveValue::set(item.pid.clone()),
            user_id: ActiveValue::set(item.user_id.clone()),
            plan_id: ActiveValue::set(item.plan_id.clone()),
            credit_amount: ActiveValue::set(item.credit_amount.clone()),
            model_amount: ActiveValue::set((item.model_amount.clone())),
            currency: ActiveValue::set(item.currency.clone()),
            payment_id: ActiveValue::set(item.payment_id.clone()),
            status: ActiveValue::set(item.status),
            ..Default::default()
        };
        let transaction = transaction.insert(db).await?;
        Ok(transaction.into())
    }

    pub async fn status_completed(mut self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        self.status = ActiveValue::Set(Status::Completed);
        Ok(self.update(db).await?)
    }
    pub async fn status_failed(mut self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        self.status = ActiveValue::Set(Status::Failed);
        Ok(self.update(db).await?)
    }
}
// implement your read-oriented logic here
impl Model {
    pub async fn status_completed(mut self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.status = ActiveValue::Set(Status::Completed);
        let updated = new.update(db).await?;
        Ok(updated)
    }
    pub async fn status_completed_v2(
        pid: &uuid::Uuid,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        let found = Self::find_by_pid(&pid, db).await?;
        let mut new = ActiveModel::from(found);
        new.status = ActiveValue::Set(Status::Completed);
        let updated = new.update(db).await?;
        Ok(updated)
    }
    pub async fn status_failed(mut self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.status = ActiveValue::Set(Status::Failed);
        let updated = new.update(db).await?;
        Ok(updated)
    }
    pub async fn find_by_pid(pid: &Uuid, db: &impl ConnectionTrait) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(transactions::Column::Pid, pid.clone())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(transactions::Column::Id, id)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
