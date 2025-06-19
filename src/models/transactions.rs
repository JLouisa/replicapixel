pub use super::_entities::transactions::{ActiveModel, Entity, Model};
use super::{
    PlanModel, TransactionActiveModel, UserModel,
    _entities::{sea_orm_active_enums::Status, transactions},
};
use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, ActiveValue, Condition, QueryOrder};
use serde::Serialize;
use stripe::Currency;
pub type Transactions = Entity;

#[derive(Debug, Serialize, Clone)]
pub struct TransactionDomain {
    pub pid: Uuid,
    pub user_id: i32,
    pub plan_id: i32,
    pub credit_amount: i32,
    pub model_amount: i32,
    pub payment_amount: f64,
    pub currency: String,
    pub payment_id: String,
    pub status: Status,
    pub created_at: String,
}
impl From<Model> for TransactionDomain {
    fn from(value: Model) -> Self {
        Self {
            pid: value.pid,
            user_id: value.user_id,
            plan_id: value.plan_id,
            credit_amount: value.credit_amount,
            model_amount: value.model_amount,
            payment_amount: value.payment_amount as f64 / 100.0,
            currency: value.currency,
            payment_id: value.payment_id,
            status: value.status,
            created_at: value.created_at.format("%B %d, %Y at %H:%M %Z").to_string(),
        }
    }
}
impl TransactionDomain {
    pub fn new(
        user: &UserModel,
        plan: &PlanModel,
        currency: Option<Currency>,
        payment_id: String,
        payment_amount: i64,
        status: Option<Status>,
    ) -> Self {
        Self {
            pid: Uuid::new_v4(),
            user_id: user.id,
            plan_id: plan.id,
            credit_amount: plan.credit_amount,
            model_amount: plan.model_amount,
            payment_amount: payment_amount as f64 / 100.0,
            currency: match currency {
                Some(info) => info.to_string(),
                None => Currency::USD.to_string(),
            },
            payment_id,
            status: status.unwrap_or_default(),
            created_at: chrono::Utc::now()
                .format("%B %d, %Y at %H:%M %Z")
                .to_string(),
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

#[derive(Debug, Clone, Constructor, AsRef)]
pub struct TransactionModelList(Vec<Model>);

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
    pub async fn save(db: &impl ConnectionTrait, item: &TransactionDomain) -> ModelResult<Model> {
        let payment_amount = (item.payment_amount.clone() * 100.0) as i64;
        let transaction = ActiveModel {
            pid: ActiveValue::set(item.pid.clone()),
            user_id: ActiveValue::set(item.user_id.clone()),
            plan_id: ActiveValue::set(item.plan_id.clone()),
            credit_amount: ActiveValue::set(item.credit_amount.clone()),
            model_amount: ActiveValue::set(item.model_amount.clone()),
            payment_amount: ActiveValue::set(payment_amount),
            currency: ActiveValue::set(item.currency.clone()),
            payment_id: ActiveValue::set(item.payment_id.clone()),
            status: ActiveValue::set(item.status),
            ..Default::default()
        };
        let transaction = transaction.insert(db).await?;
        Ok(transaction)
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
    pub async fn status_completed(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
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
    pub async fn status_failed(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.status = ActiveValue::Set(Status::Failed);
        let updated = new.update(db).await?;
        Ok(updated)
    }
    pub async fn find_by_pid(pid: &Uuid, db: &impl ConnectionTrait) -> ModelResult<Self> {
        let condition = Condition::all().add(transactions::Column::Pid.eq(pid.clone()));
        let user = Entity::find().filter(condition).one(db).await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_pid_webhook(
        pid: &Uuid,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Option<Self>> {
        let condition = Condition::all().add(transactions::Column::Pid.eq(pid.clone()));
        let user = Entity::find().filter(condition).one(db).await?;
        Ok(user)
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
    pub async fn find_all_user_txn(
        db: &impl ConnectionTrait,
        user_id: i32,
    ) -> ModelResult<TransactionModelList> {
        let condition = Condition::all().add(transactions::Column::UserId.eq(user_id));
        let order_column = transactions::Column::CreatedAt;
        let list = Entity::find()
            .filter(condition)
            .order_by_desc(order_column)
            .all(db)
            .await?;
        Ok(TransactionModelList::new(list))
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
