#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::domain::mailer_options::MailerOptions;
use crate::domain::website::Website;
use crate::mailers::transaction::CheckoutMailer;
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::join::user_credits_models::load_user_and_credits_with_user_id;
use crate::models::{ImageModel, TrainingModelActiveModel, TrainingModelModel, VideoModel};
use crate::service::aws::s3::{AwsS3, S3Key};
use crate::service::fal_ai::fal_client::{FalAiClient, FluxApiWebhookResponse, StatusResponse};
use crate::service::meta::meta::{EventData, UserData};
use crate::workers::downloader::DownloadWorker;
use crate::workers::downloader::DownloadWorkerArgs;
use crate::workers::meta_worker::{MetaConversionApiWorker, MetaConversionApiWorkerArgs};
use crate::{
    service::stripe::stripe::StripeClient,
    service::stripe::stripe_service::{StripeServiceError, StripeWebhookService},
};
use axum::{
    debug_handler, extract::State, http::HeaderMap, http::StatusCode, response::IntoResponse,
    Extension, Json,
};
use loco_rs::prelude::*;
use stripe::Webhook;

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Webhooks;
    impl Webhooks {
        pub const BASE: &'static str = "/api/webhooks";
        pub const API_STRIPE: &'static str = "/stripe";
        pub const API_FAL_AI_TRAINING: &'static str = "/fal-ai/training";
        pub const API_FAL_AI_IMAGE: &'static str = "/fal-ai/image";
        pub const API_FAL_AI_VIDEO: &'static str = "/fal-ai/video";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Webhooks::BASE)
        .add(routes::Webhooks::API_STRIPE, post(stripe))
        .add(routes::Webhooks::API_FAL_AI_TRAINING, post(fal_ai_training))
        .add(routes::Webhooks::API_FAL_AI_IMAGE, post(fal_ai_image))
        .add(routes::Webhooks::API_FAL_AI_VIDEO, post(fal_ai_video))
}

async fn load_image_by_request_id(ctx: &AppContext, id: &str) -> Result<ImageModel> {
    let item = ImageModel::find_by_request_id(&ctx.db, id).await?;
    Ok(item)
}
async fn load_video_by_request_id(ctx: &AppContext, id: &str) -> Result<VideoModel> {
    let item = VideoModel::find_by_request_id(&ctx.db, id).await?;
    Ok(item)
}

#[debug_handler]
pub async fn stripe(
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    headers: HeaderMap,
    body: String,
) -> Result<Response> {
    // 1. Extract the Stripe signature
    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StripeServiceError::SignatureError)?;

    // 2. Construct the Stripe event
    let event = Webhook::construct_event(
        &body,
        signature,
        &stripe_client.settings.stripe_webhook_secret,
    )
    .map_err(|e| StripeServiceError::SignatureVerifyError(e.to_string()))?;

    // 3. Handle the webhook event
    let email_data = match StripeWebhookService::handle_webhook(event, &ctx).await? {
        Some(email_data) => email_data,
        None => return Ok((StatusCode::OK).into_response()),
    };

    //4. Send to Email
    let mailer_options = MailerOptions::new()
        .website(&website)
        .user(&email_data.user.clone().into())
        .transaction(&email_data.transaction.clone().into())
        .plan(&email_data.plan.clone().into())
        .set_stripe_receipt_url(email_data.stripe_receipt_url);
    CheckoutMailer::send_checkout_completed(&ctx, &mailer_options).await?;

    //5. Send to queue for processing for Meta
    if !cfg!(debug_assertions) {
        let user_data = UserData::new(&email_data.user);
        let meta = EventData::purchase(&email_data.user, &email_data.transaction)
            .set_user_data(&user_data);
        let worker_arg =
            MetaConversionApiWorkerArgs::new(meta, website.website_basic_info.meta_pixel);
        if let Err(e) = MetaConversionApiWorker::perform_later(&ctx, worker_arg).await {
            tracing::warn!("⚠️ Failed to queue MetaConversionApiWorker: {e}");
        }
    }

    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
pub async fn fal_ai_video(
    State(ctx): State<AppContext>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Json(response): Json<FluxApiWebhookResponse>,
) -> Result<Response> {
    let video = match load_video_by_request_id(&ctx, &response.request_id).await {
        Ok(model) => model,
        Err(_) => {
            return Ok((StatusCode::OK, "Model not found".to_string()).into_response());
        }
    };
    // Check the status of the response
    let video_url = match response.status {
        StatusResponse::Ok => {
            // If the status is OK, check if there's a payload
            if let Some(ref _payload) = response.payload {
                let video_url = response.successful_video_opt();
                video_url
            } else {
                // If there's no payload, get payload directly
                let result = fal_ai_client
                    .request_result_image(&response.request_id)
                    .await
                    .map_err(|_| {
                        loco_rs::Error::Message("Error processing Result Request: 103".to_string())
                    })?
                    .image_url();
                result
            }
        }
        StatusResponse::Error => {
            // If the status is Error, return the error payload
            // let error_payload = response.error();

            let db_txn = ctx.db.begin().await?;

            // Get User and Image
            let (_, user_credits) =
                load_user_and_credits_with_user_id(&db_txn, &None, &Some(video.user_id)).await?;

            // --- Update User Credits/Entitlements ---
            user_credits.failed_update_credits(&db_txn, &video).await?;

            video.update_fal_video_url_failed(&db_txn).await?;

            db_txn.commit().await?;

            return Ok((StatusCode::OK).into_response());
        }
    };

    // Update the video to processing
    let video_model = video
        .update_fal_video_url_processing(&ctx.db, video_url)
        .await?;

    let worker_arg = DownloadWorkerArgs::new(video_model.pid);
    DownloadWorker::perform_later(&ctx, worker_arg).await?;

    Ok((StatusCode::OK, "Payload successfully processed").into_response())
}

#[debug_handler]
pub async fn fal_ai_image(
    State(ctx): State<AppContext>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Json(response): Json<FluxApiWebhookResponse>,
) -> Result<Response> {
    let image = match load_image_by_request_id(&ctx, &response.request_id).await {
        Ok(model) => model,
        Err(_) => {
            return Ok((StatusCode::OK, "Model not found".to_string()).into_response());
        }
    };
    // Check the status of the response
    let image_url = match response.status {
        StatusResponse::Ok => {
            // If the status is OK, check if there's a payload
            if let Some(ref _payload) = response.payload {
                let image_url = response.successful_img_opt();
                image_url
            } else {
                // If there's no payload, get payload directly
                let result = fal_ai_client
                    .request_result_image(&response.request_id)
                    .await
                    .map_err(|_| {
                        loco_rs::Error::Message("Error processing Result Request: 103".to_string())
                    })?
                    .image_url();
                result
            }
        }
        StatusResponse::Error => {
            // If the status is Error, return the error payload
            // let error_payload = response.error();

            let db_txn = ctx.db.begin().await?;

            // Get User and Image
            let (_, user_credits) =
                load_user_and_credits_with_user_id(&db_txn, &None, &Some(image.user_id)).await?;

            // --- Update User Credits/Entitlements ---
            user_credits.failed_update_credits(&db_txn, &image).await?;

            image
                .update_fal_image_url(&db_txn, None, Status::Failed)
                .await?;

            db_txn.commit().await?;

            return Ok((StatusCode::OK).into_response());
        }
    };

    // Update the image
    image
        .update_fal_image_url(&ctx.db, image_url, Status::Processing)
        .await?;

    Ok((StatusCode::OK, "Payload successfully processed").into_response())
}

#[debug_handler]
pub async fn fal_ai_training(
    State(ctx): State<AppContext>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Extension(s3_client): Extension<AwsS3>,
    Json(response): Json<FluxApiWebhookResponse>,
) -> Result<Response> {
    dbg!(&response.request_id);
    let train_model = TrainingModelModel::find_by_request_id(&ctx.db, &response.request_id).await?;
    let train = TrainingModelActiveModel::from(train_model);

    // Check the status of the response
    let tensor_path_lora = match &response.status {
        StatusResponse::Ok => {
            // If the status is OK, check if there's a payload
            if let Some(ref _payload) = response.payload {
                // let tensor_path_lora = response.successful_training().lora();
                let tensor_path_lora = response.successful_training_opt();
                tracing::info!("tensor_path_lora: {:?}", tensor_path_lora);
                tensor_path_lora
            } else {
                // If there's no payload, get payload directly
                let result = fal_ai_client
                    .request_result_training(&response.request_id)
                    .await?
                    .lora();
                tracing::warn!("tensor_path_lora: {:?}", result);
                Some(result)
            }
        }
        StatusResponse::Error => {
            // If the status is Error, return the error payload
            // let error_payload = response.error();

            tracing::error!("Status Error");
            let db_txn = ctx.db.begin().await?;
            let train = train.update_failed_fal_ai_training_webhook(&db_txn).await?;
            let (_, user_credits) =
                load_user_and_credits_with_user_id(&db_txn, &None, &Some(train.user_id)).await?;
            user_credits
                .failed_update_credits_training_model(&db_txn)
                .await?;
            db_txn.commit().await?;

            s3_client
                .remove_object_s3_key(&S3Key::new(&train.s3_key))
                .await
                .map_err(|_| {
                    loco_rs::Error::Message("Error processing Result Request: 103".to_string())
                })?;
            return Ok((StatusCode::OK).into_response());
        }
    };

    // If the status is OK, check if there's a payload
    train
        .update_fal_ai_training_webhook(&ctx.db, tensor_path_lora, Status::Completed)
        .await?;

    //Todo Send Email to client that their model is finished training

    Ok((StatusCode::OK, "Payload successfully processed").into_response())
}
