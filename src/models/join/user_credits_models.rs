use std::collections::HashSet;

use loco_rs::prelude::*;
use migration::IntoCondition;
use sea_orm::prelude::*;
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr, JoinType};
use thiserror::Error;
use uuid::Uuid;

use crate::models::images::{self as images_model, ImagesModelList, Model as ImageModel};
use crate::models::packs::{self as pack_model, Model as PackModel};
use crate::models::training_models::{
    self as training_models_model, Model as TrainingModelModel, TrainingModelList,
};
use crate::models::user_credits::{self as user_credits_model, Model as UserCreditModel};
use crate::models::users::{
    self as users_model, users, Entity as UserEntity, Model as UserModel, UserPid,
};

use crate::models::_entities::{
    packs,
    sea_orm_active_enums::{BasedOn, Ethnicity, EyeColor, ImageFormat, Sex, Status},
    training_models,
};

#[derive(Error, Debug)]
pub enum JoinError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    #[error("User not found for PID: {0}")]
    UserNotFound(String),
    #[error("Data integrity error: User credits unexpectedly missing for User ID: {0}")]
    CreditsMissingInvariant(i32),
    #[error("Invalid PID format: {0}")]
    InvalidPidFormat(String),
    #[error("Conversion Error: {0}")]
    ParseIdError(#[from] uuid::Error),
    #[error("User not found for PID: {0}")]
    ImageNotFound(String),
}

type CombinedUserUserCredits = (UserModel, Option<UserCreditModel>);

type CombinedResult = (
    UserModel,
    Option<UserCreditModel>,
    Option<TrainingModelModel>,
);

type CombinedWithPacksResult = (
    UserModel,
    Option<UserCreditModel>,
    Option<TrainingModelModel>,
    Option<Vec<PackModel>>,
);

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

// pub async fn load_user_credit_packs(
//     db: &DatabaseConnection,
//     pid: &UserPid,
// ) -> Result<(UserModel, UserCreditModel, Vec<PackModel>), JoinError> {
//     let pid_string = pid.as_ref().to_string();
//     let pid_uuid = Uuid::parse_str(&pid_string).unwrap();
//     let query_results = UserEntity::find()
//         .filter(users::Column::Pid.eq(pid_uuid))
//         .join(JoinType::InnerJoin, users::Relation::UserCredits.def())
//         .select_also(user_credits_model::Entity)
//         .select_also(pack_model::Entity)
//         .all(db)
//         .await?;

//     if query_results.is_empty() {
//         let user_exists = UserEntity::find()
//             .filter(users::Column::Pid.eq(&pid_string))
//             .count(db)
//             .await?
//             > 0;
//         if !user_exists {
//             return Err(JoinError::UserNotFound(pid_string));
//         } else {
//             return Err(JoinError::UserNotFound(format!(
//                 "User {} found, but query with credits failed.",
//                 pid_string
//             )));
//         }
//     }

//     let user_model = query_results[0].0.clone();

//     let user_credit_model = query_results[0]
//         .1
//         .clone()
//         .ok_or_else(|| JoinError::CreditsMissingInvariant(user_model.id))?;
//     let pack_model_vec: Vec<PackModel> = query_results
//         .into_iter()
//         .filter_map(|(_, _, tm_opt)| tm_opt)
//         .collect();
//     Ok((user_model, user_credit_model, pack_model_vec))
// }

pub async fn load_user_credit_training(
    db: &DatabaseConnection,
    pid: &UserPid,
) -> Result<(UserModel, UserCreditModel, TrainingModelList), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string).unwrap();
    let query_results = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid))
        .join(JoinType::InnerJoin, users::Relation::UserCredits.def())
        .left_join(training_models_model::Entity)
        .select_also(user_credits_model::Entity)
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

    let user_credit_model = query_results[0]
        .1 // Option<UserCreditModel>
        .clone()
        .ok_or_else(|| JoinError::CreditsMissingInvariant(user_model.id))?;

    // Aggregate the training models from the third element of the tuple in *all* rows.
    let training_models: Vec<TrainingModelModel> = query_results
        .into_iter()
        // The third element is Option<TrainingModelModel> due to select_also + left_join
        .filter_map(|(_, _, tm_opt)| tm_opt) // Extract the Some(model), discard None
        .collect();

    // Wrap the collected training models
    let training_model_list = TrainingModelList::new(training_models);

    // Return the combined data
    Ok((user_model, user_credit_model, training_model_list))
}

pub async fn load_user_and_training(
    db: &DatabaseConnection,
    pid: &UserPid,
) -> Result<(UserModel, TrainingModelList), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string).unwrap();
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
    let training_model_list = TrainingModelList::new(training_models);

    Ok((user_model, training_model_list))
}

pub async fn load_user_and_credits(
    db: &DatabaseConnection,
    pid: &UserPid,
) -> Result<(UserModel, UserCreditModel), JoinError> {
    let pid_string = pid.as_ref().to_string();
    let pid_uuid = Uuid::parse_str(&pid_string).unwrap();
    // --- Build the Query ---
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
