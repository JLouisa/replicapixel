use crate::{
    controllers::images::{ImageGenerationTrait, IMAGE_COST},
    domain::domain_services::video_generation::VideoProcessingError,
    models::{images::ImageNewList, TrainingModelModel, UserCreditModel, UserModel},
    service::fal_ai::fal_client::{FalAiClient, FalAiClientError},
};
use loco_rs::prelude::*;
use sea_orm::{DbErr, TransactionTrait};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MediaGenerationError {
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
    #[error("Video Processing error: {0}")]
    VideoProcessingError(#[from] VideoProcessingError),
}

pub struct ImageGenerationService;

impl ImageGenerationService {
    pub async fn generate(
        ctx: &AppContext,
        fal_ai_client: &FalAiClient,
        request: impl ImageGenerationTrait,
        user: &UserModel,
        training_model: &Option<TrainingModelModel>,
    ) -> Result<(UserCreditModel, ImageNewList), MediaGenerationError> {
        // Cost and image amount from request and user has enough credits
        let txn = ctx.db.begin().await?;
        let user_credits = UserCreditModel::find_by_user_id(&txn, user.id).await?;
        let mut credits_needed = request.cost();
        let expected_image_amount = request.num_images();
        if user_credits.credit_amount < credits_needed {
            txn.rollback().await?;
            return Err(MediaGenerationError::InsufficientCredits);
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

        // Update user's credits in DB
        let updated_credits_model = user_credits
            .update_new_credits(credits_needed, &txn)
            .await?;

        // Commit the transaction
        txn.commit().await?;

        Ok((updated_credits_model, fal_response))
    }
}
