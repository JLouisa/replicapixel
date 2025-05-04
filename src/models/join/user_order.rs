use loco_rs::prelude::*;
use sea_orm::prelude::*;
use sea_orm::{query::*, JoinType};
use uuid::Uuid;

use crate::models::_entities::transactions;
use crate::models::users::{users, Entity as UserEntity, Model as UserModel, UserPid};
use crate::models::TransactionModel;

use super::user_credits_models::JoinError;

pub async fn load_user_and_order(
    db: &impl ConnectionTrait,
    pid: &UserPid,
    txn_pid: &Uuid,
) -> Result<(UserModel, TransactionModel), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string)?;
    let (user, order_opt) = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .join(JoinType::InnerJoin, users::Relation::Transactions.def())
        .filter(transactions::Column::Pid.eq(*txn_pid))
        .select_also(transactions::Entity)
        .one(db)
        .await
        .map_err(|e| JoinError::Database(e))?
        .ok_or_else(|| JoinError::UserNotFound("User not found".to_string()))?;

    let order = match order_opt {
        Some(order) => order,
        None => return Err(JoinError::OrderNotFound(txn_pid.to_string())),
    };

    Ok((user, order))
}
