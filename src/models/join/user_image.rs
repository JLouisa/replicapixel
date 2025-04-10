use loco_rs::prelude::*;
use sea_orm::prelude::*;
use sea_orm::{query::*, DatabaseConnection, JoinType};
use uuid::Uuid;

use super::user_credits_models::JoinError;
use crate::models::_entities::images;
use crate::models::images::{self as images_model, Model as ImageModel};
use crate::models::users::{users, Entity as UserEntity, Model as UserModel};

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
