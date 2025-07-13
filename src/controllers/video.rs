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
use crate::models::videos::VideoGenRequestParams;
use crate::service::aws::s3::{AwsS3, S3Key};
use crate::service::redis::redis::RedisCacheDriver;
use crate::views;
use crate::views::videos::VideoView;
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
        pub upload: String,
        pub completed: String,
    }
    impl VideoRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Videos::BASE),
                generation: format!("{}{}", Videos::BASE, Videos::VIDEO_GENERATE),
                check: format!("{}{}", Videos::BASE, Videos::VIDEO_CHECK),
                upload: format!("{}{}", Videos::BASE, Videos::VIDEO_UPLOAD),
                completed: format!("{}{}", Videos::BASE, Videos::VIDEO_COMPLETED_ID),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Videos;
    impl Videos {
        pub const BASE: &'static str = "/api/videos";
        pub const VIDEO_GENERATE: &'static str = "/generate";
        pub const VIDEO_CHECK: &'static str = "/check";
        pub const VIDEO_UPLOAD: &'static str = "/upload";
        pub const VIDEO_COMPLETED_ID: &'static str = "/completed/{id}";
    }
}

pub fn routes() -> Routes {
    let mut routes = Routes::new()
        .prefix(routes::Videos::BASE)
        .add(routes::Videos::VIDEO_GENERATE, post(generate));

    if cfg!(debug_assertions) {
        pub const VIDEO_GENERATE_TEST: &'static str = "/generate/test";
        pub const VIDEO_CHECK_TEST_ID: &'static str = "/check/test/{id}";
        pub const VIDEO_COMPLETED_TEST_ID: &'static str = "/completed/test/{id}";

        routes = routes
            .add(VIDEO_GENERATE_TEST, post(generate_test))
            .add(VIDEO_CHECK_TEST_ID, get(check_test))
            .add(VIDEO_COMPLETED_TEST_ID, get(upload_completed_test))
    }
    routes
}

#[debug_handler]
pub async fn upload_completed_test(
    auth: auth::JWT,
    Path(video_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    LangEngine(lang): LangEngine,
    ViewEngine(view_engine): ViewEngine<TeraView>,
) -> Result<Response> {
    let (_, video) = load_user_and_video(&ctx.db, &auth.claims.pid, &video_pid).await?;

    let s3_key = S3Key::new(&video.video_s3_key);
    let exists = s3_client
        .check_object_exists(&s3_key)
        .await
        .map_err(|_| loco_rs::Error::Message(String::from("Error checking storage: 101")))?;

    if !exists {
        return Ok((StatusCode::NO_CONTENT).into_response().into_response());
    }

    let video = video
        .upload_s3_completed(&ctx.db)
        .await?
        .into_view(&cache, &s3_client)
        .await;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .video(&video)
        .build();

    views::videos::one(&view_engine, &website_options)
}

// Process or Failed Status
#[debug_handler]
pub async fn check_test(
    auth: auth::JWT,
    Path(video_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(s3_client): Extension<AwsS3>,
    ViewEngine(view_engine): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    let (user, video) = load_user_and_video(&ctx.db, &auth.claims.pid, &video_pid).await?;

    if video.user_id != user.id {
        return Err(Error::Unauthorized("Unauthorized".to_string()));
    }

    if video.status == Status::Processing {
        let video_view = s3_client.video_save_pre_url(video).await;

        let user_credits = load_user_credit(&ctx.db, user.id).await?;

        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .video(&video_view)
            .user_credits(user_credits.into())
            .build();

        return views::videos::one(&view_engine, &website_options);
    }

    if video.status == Status::Failed {
        let user_credits = load_user_credit(&ctx.db, user.id).await?;

        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .user_credits(user_credits.into())
            .build();

        return views::videos::one(&view_engine, &website_options);
    }

    Ok((StatusCode::NO_CONTENT).into_response())
}

// Pending Status
#[debug_handler]
pub async fn generate_test(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(view_engine): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
    Json(request): Json<VideoGenRequestParams>,
) -> Result<impl IntoResponse> {
    // 0. Validate request payload format
    request.validate()?;
    dbg!(&request);

    // 1. Load User and Training Model
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;

    // 2. Call the Domain Service to perform the core logic
    let (user_credits, video) = VideoGenerationService::test(&ctx, request, &user).await?;

    // 3. Render the view using the View Models
    let video_view = video.into();
    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .video(&video_view)
        .user_credits(user_credits.into())
        .build();
    views::videos::one(&view_engine, &website_options)
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
    Json(request): Json<VideoGenRequestParams>,
) -> Result<impl IntoResponse> {
    // 0. Validate request payload format
    request.validate()?;
    dbg!(&request);

    // 1. Load User and Training Model
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;

    // 2. Call the Domain Service to perform the core logic
    let (user_credits, video) =
        VideoGenerationService::generate(&ctx, &fal_ai_client, request, &user).await?;

    let video_view = video.into_view(&cache, &s3_client).await;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .video(&video_view)
        .user_credits(user_credits.into())
        .is_image_gen()
        .build();

    // 3. Render the view using the View Models
    views::videos::one(&view_engine, &website_options)
}
