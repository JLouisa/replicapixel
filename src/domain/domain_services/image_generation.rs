use crate::{
    controllers::images::ImageGenRequestParams,
    models::{
        images::ImageNewList, PackModel, TrainingModelModel, UserCreditModel, UserModel,
        _entities::sea_orm_active_enums::ImageSize,
    },
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
        params: ImageGenRequestParams,
        user: &UserModel,
        training_model: &TrainingModelModel,
    ) -> Result<(UserCreditModel, ImageNewList), ImageGenerationError> {
        let txn = ctx.db.begin().await?;

        let mut user_credits = UserCreditModel::find_by_user_id(&txn, user.id).await?;

        if user_credits.credit_amount < params.num_images as i32 {
            txn.rollback().await?;
            return Err(ImageGenerationError::InsufficientCredits);
        }

        let image_list = params.process(&training_model, &user.pid);

        // External API Interaction
        let fal_response = fal_ai_client
            .send_image_queue_many_async(image_list.clone())
            .await?;

        let fal_response = fal_ai_client.retry(fal_response, image_list).await?;

        // Persist Results
        fal_response.save_all(&txn).await?;

        // Business Logic: Update Credits
        let credits_to_deduct = fal_response.as_ref().len() as i32;
        user_credits.credit_amount -= credits_to_deduct;

        // // ==================================== Remove ====================================
        // image_list.save_all(&txn).await?;
        // // ==================================== Remove ====================================

        // Update credits using an active model
        let updated_credits_model = user_credits
            .update_credits_with_image_list(&fal_response, &txn)
            .await?;

        dbg!(&updated_credits_model);

        txn.commit().await?;

        // // ==================================== Remove ====================================
        // Ok((updated_credits_model, image_list))
        // // ==================================== Remove ====================================

        Ok((updated_credits_model, fal_response))
    }

    pub async fn generate_with_packs(
        ctx: &AppContext,
        fal_ai_client: &FalAiClient,
        params: PackModel,
        user: &UserModel,
        training_model: &TrainingModelModel,
        image_size: &ImageSize,
    ) -> Result<(UserCreditModel, ImageNewList), ImageGenerationError> {
        let txn = ctx.db.begin().await?;

        let mut user_credits = UserCreditModel::find_by_user_id(&txn, user.id).await?;

        if user_credits.credit_amount < params.credits as i32 {
            txn.rollback().await?;
            return Err(ImageGenerationError::InsufficientCredits);
        }
        let mut credits_to_deduct = params.credits as i32;
        let image_amount = params.amount as i32;

        let image_list = params.process_packs(&training_model, &user.pid, image_size);

        // External API Interaction
        let fal_response = fal_ai_client
            .send_image_queue_many_async(image_list.clone())
            .await?;
        let fal_response = fal_ai_client.retry(fal_response, image_list).await?;

        // Persist Results
        fal_response.save_all(&txn).await?;

        if image_amount != fal_response.as_ref().len() as i32 {
            credits_to_deduct = fal_response.as_ref().len() as i32;
            //Todo refactor to compute credits correctly
        }

        // Business Logic: Update Credits
        user_credits.credit_amount -= credits_to_deduct;

        // Update credits using an active model
        let updated_credits_model = user_credits
            .update_credits_with_image_list(&fal_response, &txn)
            .await?;

        dbg!(&updated_credits_model);

        txn.commit().await?;

        Ok((updated_credits_model, fal_response))
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
//     let training_model = TrainingModel::find_by_id(&ctx.db, request.training_model_id).await?;

//     if training_model.user_id != user.id {
//         return Err(loco_rs::Error::Message("Unauthorized".to_string()).into());
//     }

//     let mut user_credits = UserCreditsClient::load_item_by_user_id(&ctx, &user).await?;

//     if (user_credits.credit_amount - request.num_images as i32) < 1 {
//         return Err(loco_rs::Error::Message("Not enough credits".to_string()).into());
//     }

//     let image_gen = ImageGenRequestParams::process(request, &training_model);

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
//     let training_model: TrainingDomain = TrainingModel::find_by_id(&ctx.db, request.training_model_id)
//         .await?
//         .into();
//     if training_model.user_id != user.id {
//         return Err(loco_rs::Error::Message("Unauthorized".to_string()).into());
//     }
//     // let list_img = request.create_img_many(user.id);
//     let list_img: ImageDomainList = ImageGenerateDomain::process(request, &training_model).into();

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
