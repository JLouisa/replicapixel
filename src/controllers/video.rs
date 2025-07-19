#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use reqwest::StatusCode;

use crate::domain::dal::{load_user, load_user_credit};
use crate::middleware::i18nv2::LangEngine;
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::join::user_video::load_user_and_video;
use crate::models::videos::{VideoGenRequestParams, VideoModelList};
use crate::models::{UserModel, VideoModel};
use crate::service::aws::s3::AwsS3;
use crate::service::redis::redis::RedisCacheDriver;
use crate::views;
use crate::views::videos::VideoViewList;
use crate::{
    domain::{
        domain_services::video_generation::VideoGenerationService,
        website::{Website, WebsiteOptions},
    },
    models::users::UserPid,
    service::fal_ai::fal_client::FalAiClient,
};

pub const VIDEO_COST_PER_SECOND: i32 = 10;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct VideoRoutes {
        pub base: String,
        pub generation: String,
        pub check: String,
        pub api_delete: String,
        pub api_video_infinite: String,
    }
    impl VideoRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Videos::BASE),
                generation: format!("{}{}", Videos::BASE, Videos::VIDEO_GENERATE),
                check: format!("{}{}", Videos::BASE, Videos::VIDEO_CHECK),
                api_delete: format!("{}{}", Videos::BASE, Videos::API_DELETE),
                api_video_infinite: format!("{}{}", Videos::BASE, Videos::API_INFINITE),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Videos;
    impl Videos {
        pub const BASE: &'static str = "/api/videos";
        pub const VIDEO_GENERATE: &'static str = "/generate";
        pub const VIDEO_CHECK_ID: &'static str = "/check/{id}/{status}";
        pub const VIDEO_CHECK: &'static str = "/check";
        pub const API_DELETE_ID: &'static str = "/delete/{pid}";
        pub const API_DELETE: &'static str = "/delete";
        pub const API_INFINITE_ID: &'static str = "/infinite/{id}";
        pub const API_INFINITE: &'static str = "/infinite";
    }
}

pub fn routes() -> Routes {
    let mut routes = Routes::new()
        .prefix(routes::Videos::BASE)
        .add(routes::Videos::VIDEO_CHECK_ID, get(check_video_status))
        .add(routes::Videos::VIDEO_GENERATE, post(generate))
        .add(routes::Videos::API_DELETE_ID, delete(delete_video))
        .add(routes::Videos::API_INFINITE_ID, get(video_infinite_handler));

    if cfg!(debug_assertions) {
        routes = routes;
    }
    routes
}

async fn load_videos_inf(
    db: &DatabaseConnection,
    user: &UserModel,
    anchor_image_id: &Uuid,
    // params: InfiniteLoadingParams,
) -> Result<VideoModelList> {
    let list = VideoModel::get_next_20_images_after(db, user.id, anchor_image_id, 20).await?;
    Ok(VideoModelList::new(list))
}

async fn video_infinite_handler(
    auth: auth::JWT,
    Path(anchor_image_pid): Path<Uuid>,
    // Query(params): Query<InfiniteLoadingParams>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    Extension(s3_client): Extension<AwsS3>,
    State(ctx): State<AppContext>,
    ViewEngine(view_engine): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    // use crate::controllers::images::InfiniteLoadingParams;

    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let videos: VideoModelList = load_videos_inf(&ctx.db, &user, &anchor_image_pid)
        .await?
        .into();
    let videos = videos.into_view(&cache, &s3_client).await;
    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .videos(&videos)
        .is_infinite()
        .build();
    views::videos::video_router(&view_engine, &website_options)
}

#[debug_handler]
pub async fn delete_video(
    auth: auth::JWT,
    Path(video_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(s3_client): Extension<AwsS3>,
) -> Result<impl IntoResponse> {
    let (_, video) = load_user_and_video(&ctx.db, &auth.claims.pid, &video_pid).await?;
    let txn = ctx.db.begin().await?;
    s3_client.remove_video(&video).await?;
    let _ = video.delete(&txn).await?;
    txn.commit().await?;
    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
pub async fn generate(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(s3_client): Extension<AwsS3>,
    ViewEngine(view_engine): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
    Json(mut params): Json<VideoGenRequestParams>,
) -> Result<impl IntoResponse> {
    // 0. Validate request payload format
    params.validate()?;
    params.sanitize();

    // 1. Load User and Training Model
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;

    // 2. Call the Domain Service to perform the core logic
    let (user_credits, video) =
        VideoGenerationService::generate(&ctx, &fal_ai_client, params, &user).await?;

    // 3. Render the view using the View Models
    let video_view = video.into_view(&cache, &s3_client).await;
    let video_view_list = VideoViewList::one(video_view);
    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .videos(&video_view_list)
        .user_credits(user_credits.into())
        .is_image_gen()
        .is_oob_credits()
        .build();
    views::videos::video_router(&view_engine, &website_options)
}

#[debug_handler]
pub async fn check_video_status(
    auth: auth::JWT,
    Path((video_pid, status)): Path<(Uuid, Status)>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(s3_client): Extension<AwsS3>,
    ViewEngine(view_engine): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    let (user, video) = load_user_and_video(&ctx.db, &auth.claims.pid, &video_pid).await?;

    if video.user_id != user.id {
        return Err(Error::Unauthorized("Unauthorized".to_string()));
    }

    if video.status == status {
        return Ok((StatusCode::NO_CONTENT).into_response());
    }

    let user_credits = load_user_credit(&ctx.db, user.id).await?;
    let video_view = match video.status {
        Status::Completed => video.into_view(&cache, &s3_client).await,
        Status::Processing => s3_client.video_save_pre_url(video).await,
        Status::Failed => video.into(),
        _ => return Ok((StatusCode::NO_CONTENT).into_response()),
    };

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .video(&video_view)
        .user_credits(user_credits.into())
        .build();
    views::videos::video_router(&view_engine, &website_options)
}
