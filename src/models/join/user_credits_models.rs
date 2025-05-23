use loco_rs::prelude::*;
use migration::IntoCondition;
use sea_orm::prelude::*;
use sea_orm::{query::*, DatabaseConnection, DbErr, JoinType};
use thiserror::Error;
use uuid::Uuid;

use crate::models::UserSettingsModel;
use crate::models::_entities::training_models as training_models_entity;
use crate::models::training_models::{
    self as training_models_model, Model as TrainingModelModel, TrainingModelList,
};
use crate::models::user_credits::{self as user_credits_model, Model as UserCreditModel};
use crate::models::users::{users, Entity as UserEntity, Model as UserModel, UserPid};

use crate::models::_entities::{sea_orm_active_enums::Status, training_models, user_settings};

#[derive(Error, Debug)]
pub enum JoinError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    #[error("User not found for PID: {0}")]
    UserNotFound(String),
    #[error("Data integrity error: User credits unexpectedly missing for User ID: {0}")]
    CreditsMissingInvariant(i32),
    #[error("Data integrity error: User credits unexpectedly missing for User ID: {0}")]
    SettingsMissingInvariant(i32),
    #[error("Invalid PID format: {0}")]
    InvalidPidFormat(String),
    #[error("Conversion Error: {0}")]
    ParseIdError(#[from] uuid::Error),
    #[error("User not found for PID: {0}")]
    ImageNotFound(String),
    #[error("Training not found for ID: {0}")]
    TrainingModelNotFound(Uuid),
    #[error("Training not found for PID: {0}")]
    ModelNotFound(String),
    #[error("Order not found for PID: {0}")]
    OrderNotFound(String),
    #[error("Pack not found for PID: {0}")]
    PackNotFound(String),
}

// Bugged
pub async fn load_user_and_completed_models(
    db: &DatabaseConnection,
    pid: &UserPid,
) -> Result<(UserModel, TrainingModelList), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string)?;

    let user_to_training_relation = users::Relation::TrainingModels.def();

    let query_results = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .join(
            JoinType::LeftJoin,
            user_to_training_relation.on_condition(|_left_alias, right_alias| {
                Expr::col((right_alias, training_models::Column::TrainingStatus))
                    .eq(Status::Completed) //<-------- Bugged
                    .into_condition()
            }),
        )
        .select_also(training_models_model::Entity)
        .all(db)
        .await?;

    if query_results.is_empty() {
        let user_exists = UserEntity::find()
            .filter(users::Column::Pid.eq(pid_uuid))
            .count(db)
            .await?
            > 0;

        if !user_exists {
            return Err(JoinError::UserNotFound(pid_string));
        } else {
            let user_model = UserEntity::find()
                .filter(users::Column::Pid.eq(pid_uuid))
                .one(db)
                .await?
                .ok_or_else(|| JoinError::UserNotFound(pid_string))?;

            return Ok((user_model, TrainingModelList::new(Vec::new())));
        }
    }

    let user_model = query_results[0].0.clone();

    let training_models: Vec<TrainingModelModel> = query_results
        .into_iter()
        .filter_map(|(_user, training_opt)| training_opt)
        .collect();

    let training_model_list = TrainingModelList::new(training_models);

    Ok((user_model, training_model_list))
}

pub async fn load_user_credit_training(
    db: &DatabaseConnection,
    pid: &UserPid,
) -> Result<(UserModel, UserCreditModel, TrainingModelList), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string)?;
    let query_results = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .join(JoinType::InnerJoin, users::Relation::UserCredits.def())
        .left_join(training_models_model::Entity)
        .select_also(training_models_model::Entity)
        .select_also(user_credits_model::Entity)
        .all(db)
        .await?;

    if query_results.is_empty() {
        let user_exists = UserEntity::find()
            .filter(users::Column::Pid.eq(&pid_string))
            .count(db)
            .await?
            > 0;
        if !user_exists {
            return Err(JoinError::UserNotFound(pid_string));
        } else {
            return Err(JoinError::UserNotFound(format!(
                "User {} found, but query with credits failed.",
                pid_string
            )));
        }
    }

    let user_model = query_results[0].0.clone();

    // Aggregate the training models from the third element of the tuple in *all* rows.
    let training_models: Vec<TrainingModelModel> = query_results
        .clone()
        .into_iter()
        .filter_map(|(_, tm_opt, _)| tm_opt) // Extract the Some(model), discard None
        .collect();
    let mut sorted_training_models = training_models;
    sorted_training_models.sort_by_key(|tm| tm.training_status);
    let training_model_list = TrainingModelList::new(sorted_training_models);

    let user_credit_model = query_results[0]
        .clone()
        .2
        .ok_or_else(|| JoinError::CreditsMissingInvariant(user_model.id))?;

    // Return the combined data
    Ok((user_model, user_credit_model, training_model_list))
}

pub async fn load_user_and_training(
    db: &DatabaseConnection,
    pid: &UserPid,
) -> Result<(UserModel, TrainingModelList), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string)?;
    let query_results = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .left_join(training_models_model::Entity)
        .select_also(training_models_model::Entity)
        .all(db)
        .await?;

    if query_results.is_empty() {
        let user_exists = UserEntity::find()
            .filter(users::Column::Pid.eq(&pid_string))
            .count(db)
            .await?
            > 0;
        if !user_exists {
            return Err(JoinError::UserNotFound(pid_string));
        } else {
            return Err(JoinError::UserNotFound(format!(
                "User {} found, but query with credits failed.",
                pid_string
            )));
        }
    }

    let user_model = query_results[0].0.clone();
    let training_models: Vec<TrainingModelModel> = query_results
        .into_iter()
        .filter_map(|(_, tm_opt)| tm_opt)
        .collect();
    let mut sorted_training_models = training_models;
    sorted_training_models.sort_by_key(|tm| tm.training_status);
    let training_model_list = TrainingModelList::new(sorted_training_models);

    Ok((user_model, training_model_list))
}

pub async fn load_user_credits_settings(
    db: &impl ConnectionTrait,
    pid: &UserPid,
) -> Result<(UserModel, UserCreditModel, UserSettingsModel), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string)?;
    let query_results = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .join(JoinType::InnerJoin, users::Relation::UserCredits.def())
        .join(JoinType::InnerJoin, users::Relation::UserSettings.def())
        .select_also(user_credits_model::Entity)
        .select_also(user_settings::Entity)
        .all(db)
        .await?;

    if query_results.is_empty() {
        let user_exists = UserEntity::find()
            .filter(users::Column::Pid.eq(&pid_string))
            .count(db)
            .await?
            > 0;
        if !user_exists {
            return Err(JoinError::UserNotFound(pid_string));
        } else {
            return Err(JoinError::UserNotFound(format!(
                "User {} found, but query with credits failed.",
                pid_string
            )));
        }
    }

    let user = query_results[0].0.clone();

    let user_credit_model = query_results[0]
        .1
        .clone()
        .ok_or_else(|| JoinError::CreditsMissingInvariant(user.id))?;

    let user_settings = query_results[0]
        .2
        .clone()
        .ok_or_else(|| JoinError::SettingsMissingInvariant(user.id))?;

    // Return the combined data
    Ok((user, user_credit_model, user_settings))
}

pub async fn load_user_and_settings(
    db: &impl ConnectionTrait,
    pid: &UserPid,
) -> Result<(UserModel, UserSettingsModel), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string)?;
    let query_results = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .join(JoinType::InnerJoin, users::Relation::UserSettings.def())
        .select_also(user_settings::Entity)
        .all(db)
        .await?;

    if query_results.is_empty() {
        let user_exists = UserEntity::find()
            .filter(users::Column::Pid.eq(&pid_string))
            .count(db)
            .await?
            > 0;
        if !user_exists {
            return Err(JoinError::UserNotFound(pid_string));
        } else {
            return Err(JoinError::UserNotFound(format!(
                "User {} found, but query with credits failed.",
                pid_string
            )));
        }
    }

    let user = query_results[0].0.clone();

    let user_settings = query_results[0]
        .1 // Option<UserCreditModel>
        .clone()
        .ok_or_else(|| JoinError::CreditsMissingInvariant(user.id))?;

    // Return the combined data
    Ok((user, user_settings))
}

pub async fn load_user_and_credits(
    db: &impl ConnectionTrait,
    pid: &UserPid,
) -> Result<(UserModel, UserCreditModel), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string)?;
    let query_results = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .join(JoinType::InnerJoin, users::Relation::UserCredits.def())
        .select_also(user_credits_model::Entity)
        .all(db)
        .await?;

    if query_results.is_empty() {
        let user_exists = UserEntity::find()
            .filter(users::Column::Pid.eq(&pid_string))
            .count(db)
            .await?
            > 0;
        if !user_exists {
            return Err(JoinError::UserNotFound(pid_string));
        } else {
            return Err(JoinError::UserNotFound(format!(
                "User {} found, but query with credits failed.",
                pid_string
            )));
        }
    }

    let user_model = query_results[0].0.clone();

    let user_credit_model = query_results[0]
        .1 // Option<UserCreditModel>
        .clone()
        .ok_or_else(|| JoinError::CreditsMissingInvariant(user_model.id))?;

    // Return the combined data
    Ok((user_model, user_credit_model))
}

pub async fn load_user_and_one_training_model(
    db: &DatabaseConnection,
    pid_uuid: &UserPid,
    model_pid_uuid: Uuid,
) -> Result<(UserModel, TrainingModelModel), JoinError> {
    let pid_uuid = Uuid::parse_str(&pid_uuid.as_ref())?;
    if let Some((user, training_model)) = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid.to_owned()))
        .join(JoinType::InnerJoin, users::Relation::TrainingModels.def())
        .filter(training_models_entity::Column::Pid.eq(model_pid_uuid))
        .select_also(training_models_model::Entity)
        .one(db)
        .await?
    {
        match training_model {
            Some(training_model) => {
                return Ok((user, training_model));
            }
            None => {
                return Err(JoinError::TrainingModelNotFound(model_pid_uuid));
            }
        }
    };

    // Check if user exists
    let user_exists = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid.to_owned()))
        .count(db)
        .await?
        > 0;

    if !user_exists {
        return Err(JoinError::UserNotFound(pid_uuid.to_string()));
    }

    Err(JoinError::TrainingModelNotFound(model_pid_uuid))
}
