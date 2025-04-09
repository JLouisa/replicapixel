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
    images,
    sea_orm_active_enums::{BasedOn, Ethnicity, EyeColor, ImageFormat, Sex, Status},
    training_models,
};

use super::user_credits_models::JoinError;

pub async fn load_user_and_image(
    db: &DatabaseConnection,
    pid_uuid: &str,
    img_pid: &Uuid,
) -> Result<(UserModel, ImageModel), JoinError> {
    let pid_uuid = Uuid::parse_str(&pid_uuid)?;
    if let Some((user, image)) = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid.to_owned()))
        .join(JoinType::InnerJoin, users::Relation::Images.def())
        .filter(images::Column::Pid.eq(*img_pid))
        .select_also(images_model::Entity)
        .one(db)
        .await?
    {
        match image {
            Some(image) => {
                return Ok((user, image));
            }
            None => {
                return Err(JoinError::ImageNotFound(img_pid.to_string()));
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

    Err(JoinError::ImageNotFound(img_pid.to_string()))
}
