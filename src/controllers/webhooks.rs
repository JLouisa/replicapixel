#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::_entities::training_models;
use crate::models::users;
use crate::service::stripe::stripe::StripeClient;
use crate::service::stripe::stripe_service::{StripeServiceError, StripeWebhookService};
use axum::body;
use axum::{
    debug_handler, http::HeaderMap, http::StatusCode, response::IntoResponse, Extension, Json,
};
use loco_rs::prelude::*;
use training_models::Model as TrainingModel;

use crate::service::fal_ai::fal_client::{
    FalAiClient, FluxApiWebhookResponse, StatusResponse, SuccessfulPayload,
};

use crate::{
    models::{
        _entities::training_models::{ActiveModel, Entity, Model},
        // training_models::Params as TrainingParams,
    },
    service::aws::s3::PresignedUrlRequest,
};

use axum::{
    body::Bytes, // Use Bytes to get the raw request body
    extract::State,
};
use stripe::{Event, EventObject, EventType, Webhook};

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

async fn load_item_by_request_id(ctx: &AppContext, id: &Uuid) -> Result<Model> {
    let item = training_models::Model::find_by_request_id(&ctx.db, id).await?;
    Ok(item)
}

#[debug_handler]
pub async fn stripe(
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
    body: String,
) -> Result<Response> {
    // 1. Extract the Stripe signature
    let signature = match headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
    {
        Some(sig) => sig,
        None => return Err(StripeServiceError::SignatureError)?,
    };

    // 2. Extract the Stripe event
    let event = match Webhook::construct_event(
        &body,
        signature,
        &stripe_client.settings.stripe_webhook_secret,
    ) {
        Ok(evt) => evt,
        Err(e) => return Err(StripeServiceError::SignatureVerifyError(e.to_string()))?,
    };

    // 3. Handle the specific event type
    StripeWebhookService::handle_webhook(event, &ctx).await?;

    // 4. Acknowledge receipt to Stripe
    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
pub async fn fal_ai_image(
    State(ctx): State<AppContext>,
    // Extension(fal_ai_client): Extension<FalAiClient>,
    Json(response): Json<FluxApiWebhookResponse>,
) -> impl IntoResponse {
    let training_model = match load_item_by_request_id(&ctx, &response.request_id).await {
        Ok(model) => model,
        Err(e) => {
            return (StatusCode::LENGTH_REQUIRED, "Model not found".to_string()).into_response()
        }
    };
    // let user = users::Model::find_by_pid(&ctx.db, &training_model.)
    //     .await
    //     .unwrap();
    // Check the status of the response
    match response.status {
        StatusResponse::Ok => {
            // If the status is OK, check if there's a payload
            if let Some(ref payload) = response.payload {
                let success_payload = response.successful_img();
                (StatusCode::OK, Json(success_payload)).into_response()
            } else {
                // If there's no payload, return the payload error
                (StatusCode::OK, Json(response.payload_error)).into_response()
            }
        }
        StatusResponse::Error => {
            // If the status is Error, return the error payload
            let error_payload = response.error();
            (StatusCode::OK, Json(error_payload)).into_response()
        }
    }
}

#[debug_handler]
pub async fn fal_ai_training(
    State(ctx): State<AppContext>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Json(response): Json<FluxApiWebhookResponse>,
) -> Result<Response> {
    let train_model = TrainingModel::find_by_request_id(&ctx.db, &response.request_id).await?;
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
                    .request_result(&response.request_id)
                    .await
                    .map_err(|_| {
                        loco_rs::Error::Message("Error processing Result Request: 103".to_string())
                    })?;
                result.diffusers_lora_file.url
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
