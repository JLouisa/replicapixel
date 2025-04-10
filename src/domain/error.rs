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

use super::domain_services::image_generation::GenerationError;

impl From<StripeClientError> for LocoError {
    fn from(err: StripeClientError) -> Self {
        // Log the original error for detailed debugging
        tracing::error!(error.cause = ?err, "Stripe client error occurred");

        // Choose the best LocoError variant based on the specific client error
        match err {
            StripeClientError::StripeApi(_) => {
                // You could potentially inspect stripe_err further (e.g., status code)
                // For simplicity, map to InternalServerError or Message
                LocoError::InternalServerError // Or LocoError::Message(stripe_err.to_string())
            }
            StripeClientError::Database(_) => {
                LocoError::InternalServerError // Database errors are typically internal
            }
            StripeClientError::DbModel(_) => {
                LocoError::InternalServerError // Model errors are typically internal
            }
            StripeClientError::ParseId(parse_err) => {
                // This might indicate bad input, depends on context
                LocoError::CustomError(
                    StatusCode::BAD_REQUEST,
                    ErrorDetail::new("Error", &parse_err.to_string()),
                )
                // Or LocoError::Message(parse_err.to_string())
            }
            StripeClientError::Internal(_) | StripeClientError::Configuration(_) => {
                LocoError::InternalServerError // These seem like internal issues
                                               // Or LocoError::Message(msg)
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
            StripeServiceError::DbErr(_) => LocoError::InternalServerError,
            StripeServiceError::DbModel(_) => LocoError::InternalServerError,
            StripeServiceError::LocoErr(loco_err) => LocoError::from(loco_err),
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
            JoinError::ParseIdError(parse_err) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Error", &parse_err.to_string()),
            ),
            JoinError::ImageNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("ImageNotFound", "Image Not Found"),
            ),
        }
    }
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
            GenerationError::FalAiError(_) => loco_rs::Error::InternalServerError,
            GenerationError::DatabaseError(db_err) => loco_rs::Error::DB(db_err),
            GenerationError::ConfigError(msg) => loco_rs::Error::CustomError(
                StatusCode::PAYMENT_REQUIRED,
                ErrorDetail::new("".to_string(), msg.into()),
            ),
            GenerationError::SaveError(_) => loco_rs::Error::InternalServerError,
            GenerationError::CreditUpdateError(_) => loco_rs::Error::InternalServerError,
            GenerationError::ModelError(model_err) => model_err.into(),
        }
    }
}

// // --- You might also have conversions for other common errors ---
// impl From<sea_orm::DbErr> for LocoError {
//     fn from(err: sea_orm::DbErr) -> Self {
//         tracing::error!(error.cause = ?err, "Database error occurred");
//         // Example: Map specific DB errors if needed
//         // match err {
//         //     sea_orm::DbErr::RecordNotFound(_) => LocoError::NotFound,
//         //     _ => LocoError::InternalServerError,
//         // }
//         LocoError::InternalServerError
//     }
// }
