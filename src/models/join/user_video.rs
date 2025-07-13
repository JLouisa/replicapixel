use loco_rs::prelude::*;
use sea_orm::prelude::*;
use sea_orm::{query::*, DatabaseConnection, JoinType};
use uuid::Uuid;

use super::user_credits_models::JoinError;
use crate::models::_entities::videos;
use crate::models::users::{users, Entity as UserEntity, Model as UserModel};
use crate::models::videos::{self as videos_model, Model as VideoModel};

pub async fn load_user_and_video(
    db: &DatabaseConnection,
    pid_uuid: &str,
    video_pid: &Uuid,
) -> Result<(UserModel, VideoModel), JoinError> {
    let pid_uuid = Uuid::parse_str(&pid_uuid)?;
    if let Some((user, video)) = UserEntity::find()
        .filter(users::Column::Pid.eq(pid_uuid.to_owned()))
        .join(JoinType::InnerJoin, users::Relation::Videos.def())
        .filter(videos::Column::Pid.eq(*video_pid))
        .select_also(videos_model::Entity)
        .one(db)
        .await?
    {
        match video {
            Some(video) => {
                return Ok((user, video));
            }
            None => {
                return Err(JoinError::VideoNotFound(video_pid.to_string()));
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

    Err(JoinError::VideoNotFound(video_pid.to_string()))
}
