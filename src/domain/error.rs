use super::domain_services::image_generation::ImageGenerationError;
use crate::{
    models::join::user_credits_models::JoinError,
    service::stripe::{
        stripe::StripeClientError, stripe_builder::StripeCheckoutBuilderErr,
        stripe_service::StripeServiceError,
    },
};
use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::Error as LocoError;

impl From<StripeClientError> for LocoError {
    fn from(err: StripeClientError) -> Self {
        tracing::error!(error.cause = ?err, "Stripe client error occurred");

        match err {
            StripeClientError::StripeApi(_) => LocoError::InternalServerError,
            StripeClientError::Database(_) => LocoError::InternalServerError,
            StripeClientError::DbModel(_) => LocoError::InternalServerError,
            StripeClientError::ParseId(parse_err) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Error", &parse_err.to_string()),
            ),
            StripeClientError::Internal(_) | StripeClientError::Configuration(_) => {
                LocoError::InternalServerError
            }
        }
    }
}

impl From<StripeCheckoutBuilderErr> for LocoError {
    fn from(err: StripeCheckoutBuilderErr) -> Self {
        tracing::error!(error.cause = ?err, "Checkout builder error occurred");
        match err {
            StripeCheckoutBuilderErr::MissingField(_) => LocoError::InternalServerError,
            StripeCheckoutBuilderErr::ParseIdError(parse_err) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Error", &parse_err.to_string()),
            ),
            StripeCheckoutBuilderErr::ClientOperation(client_err) => LocoError::from(client_err),
            StripeCheckoutBuilderErr::StripeError(_) => LocoError::InternalServerError,
        }
    }
}

impl From<StripeServiceError> for LocoError {
    fn from(err: StripeServiceError) -> Self {
        tracing::error!(error.cause = ?err, "Checkout builder error occurred");
        match err {
            StripeServiceError::SignatureError => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("SignatureError", "Signature Missing"),
            ),
            StripeServiceError::SignatureVerifyError(_) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("SignatureError", "Signature Verification Failed"),
            ),
            StripeServiceError::Unauthorized => {
                LocoError::Unauthorized("Unauthorized Request".to_string())
            }
            StripeServiceError::MetadataMissing => LocoError::NotFound,
            StripeServiceError::MissingMetadataField(_) => LocoError::NotFound,
            StripeServiceError::UnexpectedObject(_) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Unexpected Object", "Unexpected Event Type"),
            ),
            StripeServiceError::TransactionIdMissing => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("TransactionIdMissing", "TransactionId Missing"),
            ),
            StripeServiceError::ParseId(field) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("ParseIdError", &field.to_string()),
            ),
            StripeServiceError::DbErr(e) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Db", &e.to_string()),
            ),
            StripeServiceError::DbModel(e) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Db2", &e.to_string()),
            ),
            StripeServiceError::LocoErr(loco_err) => LocoError::from(loco_err),
            StripeServiceError::JoinError(loco_err) => LocoError::from(loco_err),
        }
    }
}

impl From<JoinError> for LocoError {
    fn from(err: JoinError) -> Self {
        tracing::error!(error.cause = ?err, "Checkout builder error occurred");
        match err {
            JoinError::Database(_) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Internal Server Error", "Internal Server Error DB"),
            ),
            JoinError::UserNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("UserNotFound", "User Not Found"),
            ),
            JoinError::InvalidPidFormat(_) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Invalid Id Format", "User Signature Invalid"),
            ),
            JoinError::CreditsMissingInvariant(_) => LocoError::NotFound,
            JoinError::SettingsMissingInvariant(_) => LocoError::NotFound,
            JoinError::ParseIdError(parse_err) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Error", &parse_err.to_string()),
            ),
            JoinError::ImageNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("ImageNotFound", "Image Not Found"),
            ),
            JoinError::TrainingModelNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("TrainingNotFound", "Model Not Found"),
            ),
        }
    }
}

// Implement conversion from our Domain Error to Loco's Error
impl From<ImageGenerationError> for loco_rs::Error {
    fn from(err: ImageGenerationError) -> Self {
        match err {
            ImageGenerationError::Unauthorized => loco_rs::Error::Unauthorized(err.to_string()),
            ImageGenerationError::InsufficientCredits => {
                loco_rs::Error::BadRequest(err.to_string())
            }
            ImageGenerationError::ModelNotFound | ImageGenerationError::UserNotFound => {
                loco_rs::Error::NotFound
            }
            ImageGenerationError::UserCreditsNotFound => loco_rs::Error::NotFound,
            ImageGenerationError::FalAiClientError(e) => e,
            ImageGenerationError::DatabaseError(db_err) => loco_rs::Error::DB(db_err),
            ImageGenerationError::ConfigError(msg) => loco_rs::Error::CustomError(
                StatusCode::PAYMENT_REQUIRED,
                ErrorDetail::new("".to_string(), msg.into()),
            ),
            ImageGenerationError::CreditUpdateError(_) => loco_rs::Error::InternalServerError,
            ImageGenerationError::ModelError(model_err) => model_err.into(),
        }
    }
}
