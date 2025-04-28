#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::domain::website::Website;
use crate::mailers::transaction::{CheckoutCompletedEmailData, CheckoutMailer};
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::_entities::{images, training_models};
use crate::models::{users, ImageModel, TrainingModelModel};
use crate::service::fal_ai::fal_client::{
    FalAiClient, FluxApiWebhookResponse, StatusResponse, SuccessfulPayload,
};
use crate::{
    models::_entities::training_models::{ActiveModel, Entity, Model},
    service::aws::s3::PresignedUrlRequest,
    service::stripe::stripe::StripeClient,
    service::stripe::stripe_service::{StripeServiceError, StripeWebhookService},
};
use axum::{
    body,
    body::{Bytes, HttpBody},
    debug_handler,
    extract::State,
    http::HeaderMap,
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use loco_rs::prelude::*;
use stripe::{Event, EventObject, EventType, Webhook};

use axum::extract::FromRequest;
use axum::http::Request;
use hyper::Body;

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Webhooks;
    impl Webhooks {
        pub const BASE: &'static str = "/api/webhooks";
        pub const API_FAL_AI_TRAINING: &'static str = "/fal-ai/training";
        pub const API_FAL_AI_IMAGE: &'static str = "/fal-ai/image";
        pub const API_STRIPE: &'static str = "/stripe";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Webhooks::BASE)
        .add(routes::Webhooks::API_FAL_AI_TRAINING, post(fal_ai_training))
        .add(routes::Webhooks::API_FAL_AI_IMAGE, post(fal_ai_image))
        .add(routes::Webhooks::API_STRIPE, post(stripe))
}

async fn load_model_by_request_id(ctx: &AppContext, id: &str) -> Result<TrainingModelModel> {
    let item = TrainingModelModel::find_by_request_id(&ctx.db, id).await?;
    Ok(item)
}
async fn load_image_by_request_id(ctx: &AppContext, id: &str) -> Result<ImageModel> {
    let item = ImageModel::find_by_request_id(&ctx.db, id).await?;
    Ok(item)
}

// async fn webhook_training_handler(bytes: Bytes) -> Result<Response> {
//     let body_string = String::from_utf8_lossy(&bytes);

//     // Debug the raw body
//     dbg!("Training", &body_string);

//     // Log the raw body
//     tracing::info!("Received Fal Training Webhook: {}", &body_string);
//     tracing::warn!("Received Fal Training Webhook: {}", &body_string);
//     tracing::error!("Received Fal Training Webhook: {}", &body_string);

//     Ok((StatusCode::OK).into_response())
// }

// async fn webhook_image_handler(bytes: Bytes) -> Result<Response> {
//     let body_string = String::from_utf8_lossy(&bytes);

//     // Debug the raw body
//     dbg!("Image", &body_string);

//     // Log the raw body
//     tracing::info!("Received Fal Image Webhook: {}", &body_string);
//     tracing::warn!("Received Fal Image Webhook: {}", &body_string);
//     tracing::error!("Received Fal Image Webhook: {}", &body_string);

//     Ok((StatusCode::OK).into_response())
// }

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
    if let Some(email_data) = StripeWebhookService::handle_webhook(event, &ctx).await? {
        CheckoutMailer::send_checkout_completed(&ctx, &website.website_basic_info, &email_data)
            .await?;
    }

    // 4. Acknowledge receipt to Stripe
    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
pub async fn fal_ai_image(
    State(ctx): State<AppContext>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Json(response): Json<FluxApiWebhookResponse>,
) -> Result<Response> {
    let image = match load_image_by_request_id(&ctx, &response.request_id).await {
        Ok(model) => model,
        Err(e) => {
            return Ok((StatusCode::OK, "Model not found".to_string()).into_response());
        }
    };
    // Check the status of the response
    let image_url = match response.status {
        StatusResponse::Ok => {
            // If the status is OK, check if there's a payload
            if let Some(ref payload) = response.payload {
                let image_url = response.successful_img().image_url();
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
            let error_payload = response.error();
            image
                .update_fal_image_url(&ctx.db, None, Status::Failed)
                .await?;
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
    Json(response): Json<FluxApiWebhookResponse>,
) -> Result<Response> {
    let train_model = TrainingModelModel::find_by_request_id(&ctx.db, &response.request_id).await?;
    let train = ActiveModel::from(train_model);

    // Check the status of the response
    let tensor_path_lora = match &response.status {
        StatusResponse::Ok => {
            // If the status is OK, check if there's a payload
            if let Some(ref _payload) = response.payload {
                let tensor_path_lora = response.successful_training().lora();
                tensor_path_lora
            } else {
                // If there's no payload, get payload directly
                let result = fal_ai_client
                    .request_result_training(&response.request_id)
                    .await
                    .map_err(|_| {
                        loco_rs::Error::Message("Error processing Result Request: 103".to_string())
                    })?
                    .lora();
                result
            }
        }
        StatusResponse::Error => {
            // If the status is Error, return the error payload
            let error_payload = response.error();
            train
                .update_fal_ai_webhook_training(&ctx.db, None, Status::Failed)
                .await?;
            return Ok((StatusCode::OK).into_response());
        }
    };

    // If the status is OK, check if there's a payload
    train
        .update_fal_ai_webhook_training(&ctx.db, Some(tensor_path_lora), Status::Completed)
        .await?;

    //Todo Send Email to client that their model is finished training

    Ok((StatusCode::OK, "Payload successfully processed").into_response())
}
