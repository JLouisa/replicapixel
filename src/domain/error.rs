use super::domain_services::image_generation::MediaGenerationError;
use crate::{
    domain::cookie::CookieError,
    models::join::user_credits_models::JoinError,
    service::{
        aws::s3::AwsError,
        fal_ai::fal_client::FalAiClientError,
        meta::meta::MetaConversionApiError,
        stripe::{
            stripe::StripeClientError, stripe_builder::StripeCheckoutBuilderErr,
            stripe_service::StripeServiceError,
        },
    },
};
use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::Error as LocoError;

impl From<CookieError> for LocoError {
    fn from(err: CookieError) -> Self {
        tracing::error!(error.cause = ?err, "Stripe client error occurred");

        match err {
            CookieError::JwtCreationError(e) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Cookie Creation Error", &e.to_string()),
            ),
            CookieError::TokenCreationError(e) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Cookie Creation Error", &e.to_string()),
            ),
            CookieError::MissingConfig => LocoError::InternalServerError,
            CookieError::InvalidLanguage => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Cookie Lang Creation Error", "Invalid language"),
            ),
            CookieError::Unknown => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Unknown cookie error", "Unknown cookie error"),
            ),
        }
    }
}

impl From<StripeClientError> for LocoError {
    fn from(err: StripeClientError) -> Self {
        tracing::error!(error.cause = ?err, "Stripe client error occurred");

        match err {
            StripeClientError::StripeApi(e) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &e.to_string()),
            ),
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
            JoinError::Database(e) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Internal Server Error", &e.to_string()),
            ),
            JoinError::UserNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("UserNotFound", "User Not Found"),
            ),
            JoinError::OrderNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("OrderNotFound", "Order Not Found"),
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
            JoinError::VideoNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("VideoNotFound", "Video Not Found"),
            ),
            JoinError::TrainingModelNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("TrainingNotFound", "Model Not Found"),
            ),
            JoinError::PackNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("PackNotFound", "Pack Not Found"),
            ),
            JoinError::ModelNotFound(_) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("ModelNotFound", "TrainingModel Not Found"),
            ),
            JoinError::OtherInternal => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("UserNotFound", "User Not Found"),
            ),
        }
    }
}

// Implement conversion from our Domain Error to Loco's Error
impl From<MediaGenerationError> for loco_rs::Error {
    fn from(err: MediaGenerationError) -> Self {
        match err {
            MediaGenerationError::Unauthorized => loco_rs::Error::Unauthorized(err.to_string()),
            MediaGenerationError::InsufficientCredits => {
                loco_rs::Error::BadRequest(err.to_string())
            }
            MediaGenerationError::ModelNotFound | MediaGenerationError::UserNotFound => {
                loco_rs::Error::NotFound
            }
            MediaGenerationError::UserCreditsNotFound => loco_rs::Error::NotFound,
            MediaGenerationError::FalAiClientErr(e) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &e.to_string()),
            ),
            MediaGenerationError::DatabaseError(db_err) => loco_rs::Error::DB(db_err),
            MediaGenerationError::ConfigError(msg) => loco_rs::Error::CustomError(
                StatusCode::PAYMENT_REQUIRED,
                ErrorDetail::new("".to_string(), msg.into()),
            ),
            MediaGenerationError::CreditUpdateError(_) => loco_rs::Error::InternalServerError,
            MediaGenerationError::ModelError(model_err) => model_err.into(),
        }
    }
}

// Implement conversion from our Domain Error to Loco's Error
impl From<FalAiClientError> for loco_rs::Error {
    fn from(err: FalAiClientError) -> Self {
        match err {
            FalAiClientError::JsonParse(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err_str),
            ),
            FalAiClientError::LocoError(err) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err.to_string()),
            ),
            FalAiClientError::ReqwestErr(e) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &e.to_string()),
            ),
            FalAiClientError::RequestFailed(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Request Failed", &err_str),
            ),
            FalAiClientError::Other(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err_str),
            ),
            FalAiClientError::SerdeErr(err) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Serde Error", &err.to_string()),
            ),
            FalAiClientError::FalApiError(err) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err.to_string()),
            ),
        }
    }
}

impl From<MetaConversionApiError> for loco_rs::Error {
    fn from(err: MetaConversionApiError) -> Self {
        match err {
            MetaConversionApiError::JsonParse(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err_str),
            ),
            MetaConversionApiError::LocoError(err) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err.to_string()),
            ),
            MetaConversionApiError::ReqwestErr(e) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &e.to_string()),
            ),
            MetaConversionApiError::RequestFailed(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Request Failed", &err_str),
            ),
            MetaConversionApiError::Other(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err_str),
            ),
            MetaConversionApiError::SerdeErr(err) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Serde Error", &err.to_string()),
            ),
        }
    }
}

impl From<AwsError> for loco_rs::Error {
    fn from(err: AwsError) -> Self {
        match err {
            AwsError::S3Err(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err_str.to_string()),
            ),
            AwsError::LocoError(err) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err.to_string()),
            ),
            AwsError::PutRequest(e) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Storage Saved Failed", &e.to_string()),
            ),
            AwsError::RequestFailed(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Storage Request Failed", &err_str.to_string()),
            ),
            AwsError::Other(err_str) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Error", &err_str),
            ),
            AwsError::S3DeletionError(err) => loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Serde Error", &err.to_string()),
            ),
        }
    }
}
