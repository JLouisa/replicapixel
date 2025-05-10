use loco_rs::prelude::*;
use sea_orm::prelude::*;

use super::user_credits_models::JoinError;

use crate::models::_entities::packs as pack_entity;
use crate::models::training_models::Model as TrainingModelModel;
use crate::models::users::{users, Entity as UserEntity, Model as UserModel, UserPid};
use crate::models::PackEntity;
use crate::models::PackModel;
use crate::models::_entities::training_models;

use futures::try_join;
use sea_orm::DatabaseConnection;
use sea_orm::{query::*, JoinType};
use uuid::Uuid;

pub async fn load_user_and_one_pack(
    db: &DatabaseConnection,
    pid_uuid: &UserPid,
    pack_pid_uuid: &Uuid,
) -> Result<(UserModel, PackModel), JoinError> {
    let pid = Uuid::parse_str(pid_uuid.as_ref())?;

    let (user, pack) = try_join!(
        async {
            UserEntity::find()
                .filter(users::Column::Pid.eq(pid))
                .one(db)
                .await
                .map_err(JoinError::Database)?
                .ok_or_else(|| JoinError::UserNotFound(pid.to_string()))
        },
        async {
            PackEntity::find()
                .filter(pack_entity::Column::Pid.eq(*pack_pid_uuid))
                .one(db)
                .await
                .map_err(JoinError::Database)?
                .ok_or_else(|| JoinError::PackNotFound(pack_pid_uuid.to_string()))
        }
    )?;

    Ok((user, pack))
}

pub async fn load_user_one_training_model_one_pack(
    db: &DatabaseConnection,
    user_pid_uuid: &UserPid,
    training_model_id: &Uuid,
    pid_uuid_pack: &Uuid,
) -> Result<(UserModel, TrainingModelModel, PackModel), JoinError> {
    let user_pid = Uuid::parse_str(user_pid_uuid.as_ref())?;

    let ((user, training_model), pack) = try_join!(
        async {
            // User+training model query
            let result = UserEntity::find()
                .filter(users::Column::Pid.eq(user_pid))
                .join(JoinType::InnerJoin, users::Relation::TrainingModels.def())
                .filter(training_models::Column::Pid.eq(*training_model_id))
                .select_also(training_models::Entity)
                .one(db)
                .await
                .map_err(JoinError::Database)?;

            match result {
                Some((user, Some(model))) => Ok((user, model)),
                _ => Err(JoinError::ModelNotFound(training_model_id.to_string())),
            }
        },
        async {
            // Pack query
            PackEntity::find()
                .filter(pack_entity::Column::Pid.eq(*pid_uuid_pack))
                .one(db)
                .await
                .map_err(JoinError::Database)?
                .ok_or_else(|| JoinError::PackNotFound(pid_uuid_pack.to_string()))
        }
    )?;

    Ok((user, training_model, pack))
}

// pub async fn load_user_and_one_pack(
//     db: &DatabaseConnection,
//     pid_uuid: &UserPid,
//     pid_uuid_pack: &Uuid,
// ) -> Result<(UserModel, PackModel), JoinError> {
//     let pid_uuid = Uuid::parse_str(&pid_uuid.as_ref())?;

//     // Get user first
//     let user = UserEntity::find()
//         .filter(users::Column::Pid.eq(pid_uuid))
//         .one(db)
//         .await?
//         .ok_or(JoinError::UserNotFound(pid_uuid.to_string()))?;

//     // Then get pack
//     let pack = PackEntity::find()
//         .filter(pack_entity::Column::Pid.eq(*pid_uuid_pack))
//         .one(db)
//         .await?
//         .ok_or(JoinError::PackNotFound(pid_uuid_pack.to_string()))?;

//     Ok((user, pack))
// }

// pub async fn load_user_one_training_model_one_pack(
//     db: &DatabaseConnection,
//     pid_uuid: &UserPid,
//     pid_uuid_pack: &Uuid,
//     training_model_id: &Uuid,
// ) -> Result<(UserModel, TrainingModelModel, PackModel), JoinError> {
//     let user_pid = Uuid::parse_str(pid_uuid.as_ref())?;

//     // First check if user exists
//     let user_exists = UserEntity::find()
//         .filter(users::Column::Pid.eq(user_pid))
//         .count(db)
//         .await
//         .map_err(JoinError::Database)?
//         > 0;

//     if !user_exists {
//         return Err(JoinError::UserNotFound(user_pid.to_string()));
//     }

//     // Then get user and training model
//     let (user, training_model) = UserEntity::find()
//         .filter(users::Column::Pid.eq(user_pid))
//         .join(JoinType::InnerJoin, users::Relation::TrainingModels.def())
//         .filter(training_models::Column::Pid.eq(*training_model_id))
//         .select_also(training_models::Entity)
//         .one(db)
//         .await
//         .map_err(JoinError::Database)?
//         .and_then(|(user, model)| model.map(|m| (user, m)))
//         .ok_or(JoinError::ModelNotFound(training_model_id.to_string()))?;

//     // Finally get pack
//     let pack = PackEntity::find()
//         .filter(pack_entity::Column::Pid.eq(*pid_uuid_pack))
//         .one(db)
//         .await
//         .map_err(JoinError::Database)?
//         .ok_or_else(|| JoinError::PackNotFound(pid_uuid_pack.to_string()))?;

//     Ok((user, training_model, pack))
// }
