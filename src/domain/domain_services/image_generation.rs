use crate::{
    controllers::images::{ImageGenerationTrait, IMAGE_COST},
    models::{images::ImageNewList, TrainingModelModel, UserCreditModel, UserModel},
    service::fal_ai::fal_client::{FalAiClient, FalAiClientError},
};
use loco_rs::prelude::*;
use sea_orm::{DbErr, TransactionTrait};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageGenerationError {
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
    #[error("Database error: {0}")]
    DatabaseError(#[from] DbErr),
    #[error("Database error: {0}")]
    ModelError(#[from] ModelError),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Failed to update credits: {0}")]
    CreditUpdateError(String),
    #[error("Fal AI client error: {0}")]
    FalAiClientErr(#[from] FalAiClientError),
}

pub struct ImageGenerationService;

impl ImageGenerationService {
    pub async fn generate(
        ctx: &AppContext,
        fal_ai_client: &FalAiClient,
        request: impl ImageGenerationTrait,
        user: &UserModel,
        training_model: &Option<TrainingModelModel>,
    ) -> Result<(UserCreditModel, ImageNewList), ImageGenerationError> {
        // Start a DB transaction
        let txn = ctx.db.begin().await?;

        // Load user credits
        let user_credits = UserCreditModel::find_by_user_id(&txn, user.id).await?;

        // Cost and image amount from request
        let mut credits_needed = request.cost();
        tracing::info!("Credits needed: {}", credits_needed);
        let expected_image_amount = request.num_images();

        // Check if user has enough credits
        if user_credits.credit_amount < credits_needed {
            txn.rollback().await?;
            return Err(ImageGenerationError::InsufficientCredits);
        }

        // Prepare image list
        let image_list = request.process(training_model, user);

        // Call external API
        let fal_response = fal_ai_client
            .send_image_queue_many_async(&image_list)
            .await?;
        let fal_response = fal_ai_client.retry(fal_response, &image_list).await?;

        // Save response to DB
        fal_response.save_all(&txn).await?;

        // Handle refunds if fewer images were generated than expected
        let actual_image_amount = fal_response.amount();
        if expected_image_amount > actual_image_amount {
            let refund_amount = (expected_image_amount - actual_image_amount) * IMAGE_COST;
            credits_needed -= refund_amount;
        }

        // Deduct final credit cost
        // user_credits.credit_amount -= credits_needed;

        // Update user's credits in DB
        let updated_credits_model = user_credits
            .update_new_credits(credits_needed, &txn)
            .await?;

        // Commit the transaction
        txn.commit().await?;

        Ok((updated_credits_model, fal_response))
    }
}

// pub struct ImageGenerationService;

// impl ImageGenerationService {
//     pub async fn generate(
//         ctx: &AppContext,
//         fal_ai_client: &FalAiClient,
//         request: impl ImageGenerationTrait,
//         user: &UserModel,
//         training_model: &Option<TrainingModelModel>,
//     ) -> Result<(UserCreditModel, ImageNewList), ImageGenerationError> {
//         let txn = ctx.db.begin().await?;

//         let mut user_credits = UserCreditModel::find_by_user_id(&txn, user.id).await?;

//         let mut credits_needed = request.cost();
//         let image_amount = request.num_images();

//         if user_credits.credit_amount < credits_needed {
//             txn.rollback().await?;
//             return Err(ImageGenerationError::InsufficientCredits);
//         }

//         let image_list = request.process(&training_model, &user);

//         // External API Interaction
//         let fal_response = fal_ai_client
//             .send_image_queue_many_async(&image_list)
//             .await?;
//         let fal_response = fal_ai_client.retry(fal_response, &image_list).await?;

//         // Persist Results
//         fal_response.save_all(&txn).await?;

//         if image_amount != fal_response.amount() {
//             let refund_amount = (image_amount - fal_response.amount()) * IMAGE_COST;
//             credits_needed -= refund_amount;
//         }

//         // Business Logic: Update Credits
//         user_credits.credit_amount -= credits_needed;

//         // Update credits using an active model
//         let updated_credits_model = user_credits
//             .update_credits_with_image_list(&fal_response, &txn)
//             .await?;

//         txn.commit().await?;

//         Ok((updated_credits_model, fal_response))
//     }
// }
