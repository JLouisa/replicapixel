use crate::{
    controllers::images::ImageGenRequestParams,
    models::{
        TrainingModelModel, UserCreditModel, UserModel,
        _entities::{training_models, user_credits, users},
        images::{Entity, ImageNew, ImageNewList},
        user_credits::UserCredits,
        ImageActiveModel,
    },
    service::fal_ai::fal_client::FalAiClient,
};
use axum::http::StatusCode;
use loco_rs::prelude::*;
use loco_rs::{controller::ErrorDetail, prelude::*};
use sea_orm::{entity::prelude::*, QueryOrder};
use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Unauthorized access to training model")]
    Unauthorized,
    #[error("Not enough credits")]
    InsufficientCredits,
    #[error("Training model not found")]
    ModelNotFound,
    #[error("User not found")]
    UserNotFound,
    #[error("User not found")]
    UserCreditsNotFound,
    #[error("Fal.ai API error: {0}")]
    FalAiError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] DbErr),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Failed to save generated images: {0}")]
    SaveError(String),
    #[error("Failed to update credits: {0}")]
    CreditUpdateError(String),
}

// Implement conversion from our Domain Error to Loco's Error
impl From<GenerationError> for loco_rs::Error {
    fn from(err: GenerationError) -> Self {
        match err {
            GenerationError::Unauthorized => loco_rs::Error::Unauthorized(err.to_string()),
            GenerationError::InsufficientCredits => loco_rs::Error::BadRequest(err.to_string()),
            GenerationError::ModelNotFound | GenerationError::UserNotFound => {
                loco_rs::Error::NotFound
            }
            GenerationError::UserCreditsNotFound => loco_rs::Error::NotFound,
            GenerationError::FalAiError(msg) => loco_rs::Error::InternalServerError,
            GenerationError::DatabaseError(db_err) => loco_rs::Error::DB(db_err),
            GenerationError::ConfigError(msg) => loco_rs::Error::CustomError(
                StatusCode::PAYMENT_REQUIRED,
                ErrorDetail::new("".to_string(), msg.into()),
            ),
            GenerationError::SaveError(msg) => loco_rs::Error::InternalServerError,
            GenerationError::CreditUpdateError(msg) => loco_rs::Error::InternalServerError,
        }
    }
}

pub struct ImageGenerationService;

impl ImageGenerationService {
    pub async fn generate(
        ctx: &AppContext,
        fal_ai_client: &FalAiClient,
        user_pid: &str,
        params: ImageGenRequestParams,
    ) -> Result<(UserCreditModel, ImageNewList), GenerationError> {
        // Basic validation can also live on the ImageGenRequestParams struct itself using `validator`
        if params.num_images < 1 || params.num_images > 20 {
            // Ideally use validator crate on the struct itself before it gets here
            return Err(GenerationError::ConfigError(
                "Wrong image amount (1-20 allowed)".to_string(),
            ));
        }

        // --- Load Entities ---
        let user = UserModel::find_by_pid(&ctx.db, user_pid)
            .await
            .map_err(|_| GenerationError::UserNotFound)?;

        let train_model = TrainingModelModel::find_by_id(&ctx.db, params.training_model_id)
            .await
            .map_err(|_| GenerationError::ModelNotFound)?;

        // --- Authorization (Moved from Controller) ---
        if train_model.user_id != user.id {
            return Err(GenerationError::Unauthorized);
        }

        // --- Transaction for Credits and Saving ---
        let txn = ctx.db.begin().await?;

        // Load credits *within* transaction for safety (SELECT ... FOR UPDATE could be better)
        let mut user_credits = UserCreditModel::find_by_user_id(&txn, user.id)
            .await
            .map_err(|e| GenerationError::FalAiError(format!("Retry failed: {}", e)))?;

        // --- Business Rule: Check Credits (Moved from Controller) ---
        if (user_credits.credit_amount) < params.num_images as i32 {
            txn.rollback().await?;
            return Err(GenerationError::InsufficientCredits);
        }

        // --- Prepare External API Call ---
        let image_gen_payload = params.process(&train_model);

        // --- External API Interaction (Moved from Controller) ---
        let fal_response = fal_ai_client
            .send_image_queue_many_async(image_gen_payload.clone())
            .await
            .map_err(|e| GenerationError::FalAiError(format!("Initial send failed: {}", e)))?;

        let fal_response = fal_ai_client
            .retry(fal_response, image_gen_payload)
            .await
            .map_err(|e| GenerationError::FalAiError(format!("Retry failed: {}", e)))?;

        // --- Persist Results ---
        fal_response
            .save_all(&txn)
            .await
            .map_err(|e| GenerationError::SaveError(e.to_string()))?;

        // --- Business Logic: Update Credits (Moved from Controller) ---
        let credits_to_deduct = fal_response.as_ref().len() as i32;
        user_credits.credit_amount -= credits_to_deduct;

        // Update credits using an active model
        let updated_credits_model = user_credits::ActiveModel {
            id: Set(user_credits.id),
            credit_amount: Set(user_credits.credit_amount),
            ..Default::default()
        };

        let user_credits = UserCreditModel::find_by_user_id(&txn, user_credits.user_id)
            .await
            .map_err(|e| GenerationError::CreditUpdateError(e.to_string()))?;

        // --- Commit Transaction ---
        txn.commit().await?;

        Ok((user_credits, fal_response))
    }
}

// #[debug_handler]
// pub async fn generate_test(
//     auth: auth::JWT,
//     State(ctx): State<AppContext>,
//     Extension(fal_ai_client): Extension<FalAiClient>,
//     ViewEngine(v): ViewEngine<TeraView>,
//     Json(request): Json<ImageGenRequestParams>,
// ) -> Result<Response> {
//     request.validate()?;
//     if request.num_images < 1 || request.num_images > 20 {
//         return Err(loco_rs::Error::Message("Wrong image amount".to_string()).into());
//     }

//     let user = User::load_user(&ctx.db, &auth.claims.pid).await?;
//     let train_model = TrainingModel::find_by_id(&ctx.db, request.training_model_id).await?;

//     if train_model.user_id != user.id {
//         return Err(loco_rs::Error::Message("Unauthorized".to_string()).into());
//     }

//     let mut user_credits = UserCreditsClient::load_item_by_user_id(&ctx, &user).await?;

//     if (user_credits.credit_amount - request.num_images as i32) < 1 {
//         return Err(loco_rs::Error::Message("Not enough credits".to_string()).into());
//     }

//     let image_gen = ImageGenRequestParams::process(request, &train_model);

//     // Send Image to FAL AI API Queue
//     let mut response = fal_ai_client
//         .send_image_queue_many_async(image_gen.clone())
//         .await?;
//     let mut response = fal_ai_client.retry(response, image_gen.clone()).await?;

//     // Save in DB
//     &response.save_all(&ctx).await?;

//     user_credits.credit_amount = user_credits.credit_amount - response.as_ref().len() as i32;
//     // user_credits.credit_amount = user_credits.credit_amount - image_gen.as_ref().len() as i32;

//     let user_credits = user_credits.save(&ctx).await?;

//     let check_route = format!(
//         "{}{}/test",
//         routes::Images::BASE,
//         routes::Images::IMAGE_CHECK
//     );

//     // views::images::one(&v, &user_credits, &image_list)
//     views::images::one(&v, &user_credits, response, check_route)
// }

// #[debug_handler]
// pub async fn generate_img(
//     auth: auth::JWT,
//     State(ctx): State<AppContext>,
//     Extension(fal_ai_client): Extension<FalAiClient>,
//     Json(request): Json<ImageGenRequestForm>,
// ) -> Result<Response> {
//     let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
//     let train_model: TrainingDomain = TrainingModel::find_by_id(&ctx.db, request.training_model_id)
//         .await?
//         .into();
//     if train_model.user_id != user.id {
//         return Err(loco_rs::Error::Message("Unauthorized".to_string()).into());
//     }
//     // let list_img = request.create_img_many(user.id);
//     let list_img: ImageDomainList = ImageGenerateDomain::process(request, &train_model).into();

//     //Send Image to FAL AI API Queue
//     let mut response = fal_ai_client
//         .send_image_queue_many_async(list_img.clone())
//         .await?;
//     let mut response = fal_ai_client.retry(response, list_img).await?;

//     // Bulk insert
//     let models = response.save_all(&ctx).await?;

//     let mut headers = HeaderMap::new();
//     headers.insert("HX-Redirect", "/dashboard/partial/camera".parse().unwrap()); // HTMX Redirect

//     Ok((StatusCode::OK, headers).into_response())
// }
