use loco_rs::app::AppContext;
use loco_rs::prelude::*;

use crate::models::{
    users::UserPid, videos::VideoModelList, ImageModel, UserCreditModel, UserModel, VideoModel,
};

// ================ User =================
pub async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
    Ok(item)
}
// ================ User Credit =================
pub async fn load_user_credit(db: &DatabaseConnection, user_id: i32) -> Result<UserCreditModel> {
    let item = UserCreditModel::find_by_user_id(db, user_id).await?;
    Ok(item)
}
pub async fn load_user_credits_by_user_id(
    db: &impl ConnectionTrait,
    user_id: i32,
) -> Result<UserCreditModel, ModelError> {
    let item = UserCreditModel::find_by_user_id(db, user_id).await?;
    Ok(item)
}
// ================ Image =================
pub async fn load_image_by_request_id(ctx: &AppContext, id: &str) -> Result<ImageModel> {
    let item = ImageModel::find_by_request_id(&ctx.db, id).await?;
    Ok(item)
}
// ================ Video =================
pub async fn load_video_by_request_id(ctx: &AppContext, id: &str) -> Result<VideoModel> {
    let item = VideoModel::find_by_request_id(&ctx.db, id).await?;
    Ok(item)
}
pub async fn load_first_videos(
    db: &DatabaseConnection,
    id: i32,
    fav: bool,
    del: bool,
) -> Result<VideoModelList> {
    let list = VideoModel::find_x_videos_by_user_id(db, id, fav, del, 30).await?;
    Ok(VideoModelList::new(list))
}
