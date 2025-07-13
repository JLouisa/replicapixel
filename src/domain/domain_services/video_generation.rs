use crate::{
    domain::{
        dal::load_user_credits_by_user_id, domain_services::image_generation::MediaGenerationError,
    },
    models::{
        images::UserPrompt, UserCreditModel, UserModel,
        _entities::sea_orm_active_enums::AspectRatio, videos::VideoNew, VideoModel,
    },
    service::fal_ai::fal_client::{FalAiClient, FalVideoSend, QueueResponse, WebhookPayload},
};
use loco_rs::prelude::*;
use sea_orm::TransactionTrait;

pub trait VideoGenerationTrait {
    fn formatted_prompt(&self) -> UserPrompt;
    fn aspect_ratio(&self) -> AspectRatio;
    fn cost(&self) -> i32;
    fn quality_model(&self) -> WebhookPayload;
    fn process(self, user: &UserModel) -> VideoNew;
}

pub struct VideoGenerationService;

impl VideoGenerationService {
    pub async fn generate(
        ctx: &AppContext,
        fal_ai_client: &FalAiClient,
        request: impl VideoGenerationTrait,
        user: &UserModel,
    ) -> Result<(UserCreditModel, VideoModel), MediaGenerationError> {
        // Prepare Video request
        let video_request = request.process(user);

        // Cost and Video amount from request
        let txn = ctx.db.begin().await?;
        let user_credits = load_user_credits_by_user_id(&txn, user.id).await?;
        if user_credits.credit_amount < video_request.cost() {
            txn.rollback().await?;
            return Err(MediaGenerationError::InsufficientCredits);
        }

        // Retry sending to Fal AI
        let fal_video_send: FalVideoSend = video_request.clone().into();
        let fal_response = fal_ai_client
            .send_queue_webhook_with_retries::<FalVideoSend, QueueResponse>(&fal_video_send, 3)
            .await?;

        // Update user's credits in DB
        let video = video_request.save(&txn, &fal_response).await?;
        let updated_user_credits = user_credits.deduct_credits(&txn, &video).await?;

        txn.commit().await?;
        Ok((updated_user_credits, video))
    }
    pub async fn test(
        ctx: &AppContext,
        request: impl VideoGenerationTrait,
        user: &UserModel,
    ) -> Result<(UserCreditModel, VideoModel), MediaGenerationError> {
        // Prepare Video request
        let video_request = request.process(user);

        // Cost and Video amount from request
        let txn = ctx.db.begin().await?;
        let user_credits = load_user_credits_by_user_id(&txn, user.id).await?;
        if user_credits.credit_amount < video_request.cost() {
            txn.rollback().await?;
            return Err(MediaGenerationError::InsufficientCredits);
        }

        // Retry sending to Fal AI
        let fal_response = QueueResponse::test();

        // Update user's credits in DB
        let video = video_request.save(&txn, &fal_response).await?;
        let updated_user_credits = user_credits.deduct_credits(&txn, &video).await?;

        txn.commit().await?;
        Ok((updated_user_credits, video))
    }
}
