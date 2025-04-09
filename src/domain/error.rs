use crate::{
    controllers::payment::routes,
    domain::url::Url,
    models::{join::user_credits_models::JoinError, UserActiveModel, UserModel},
    service::stripe::{
        stripe::StripeClientError, stripe_builder::StripeCheckoutBuilderErr,
        stripe_service::StripeServiceError,
    },
};
use axum::http::StatusCode;
use axum::{extract::Json, routing::post, Router};
use derive_more::{AsRef, Constructor, From};
use loco_rs::prelude::Error as LocoError;
use loco_rs::{controller::ErrorDetail, model::ModelError};
use sea_orm::entity::prelude::*;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, Customer, CustomerId, ParseIdError, PriceId,
    StripeError,
};

use thiserror::Error;

use super::domain_services::image_generation::GenerationError;

impl From<StripeClientError> for LocoError {
    fn from(err: StripeClientError) -> Self {
        // Log the original error for detailed debugging
        tracing::error!(error.cause = ?err, "Stripe client error occurred");

        // Choose the best LocoError variant based on the specific client error
        match err {
            StripeClientError::StripeApi(stripe_err) => {
                // You could potentially inspect stripe_err further (e.g., status code)
                // For simplicity, map to InternalServerError or Message
                LocoError::InternalServerError // Or LocoError::Message(stripe_err.to_string())
            }
            StripeClientError::Database(db_err) => {
                LocoError::InternalServerError // Database errors are typically internal
            }
            StripeClientError::DbModel(model_err) => {
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
            StripeClientError::Internal(msg) | StripeClientError::Configuration(msg) => {
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
            StripeCheckoutBuilderErr::MissingField(field) => LocoError::InternalServerError,
            StripeCheckoutBuilderErr::ParseIdError(parse_err) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Error", &parse_err.to_string()),
            ),
            StripeCheckoutBuilderErr::ClientOperation(client_err) => LocoError::from(client_err),
            StripeCheckoutBuilderErr::StripeError(stripe_err) => LocoError::InternalServerError,
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
            StripeServiceError::SignatureVerifyError(field) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("SignatureError", "Signature Verification Failed"),
            ),
            StripeServiceError::Unauthorized => {
                LocoError::Unauthorized("Unauthorized Request".to_string())
            }
            StripeServiceError::MetadataMissing => LocoError::NotFound,
            StripeServiceError::MissingMetadataField(field) => LocoError::NotFound,
            StripeServiceError::UnexpectedObject(field) => LocoError::CustomError(
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
            StripeServiceError::DbErr(db_err) => LocoError::InternalServerError,
            StripeServiceError::DbModel(model_err) => LocoError::InternalServerError,
            StripeServiceError::LocoErr(loco_err) => LocoError::from(loco_err),
        }
    }
}

impl From<JoinError> for LocoError {
    fn from(err: JoinError) -> Self {
        tracing::error!(error.cause = ?err, "Checkout builder error occurred");
        match err {
            JoinError::Database(db_err) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Internal Server Error", "Internal Server Error DB"),
            ),
            JoinError::UserNotFound(field) => LocoError::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("UserNotFound", "User Not Found"),
            ),
            JoinError::InvalidPidFormat(field) => LocoError::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("Invalid Id Format", "User Signature Invalid"),
            ),
            JoinError::CreditsMissingInvariant(field) => LocoError::NotFound,
            JoinError::ParseIdError(parse_err) => LocoError::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new("Error", &parse_err.to_string()),
            ),
            JoinError::ImageNotFound(field) => LocoError::CustomError(
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
            GenerationError::FalAiError(msg) => loco_rs::Error::InternalServerError,
            GenerationError::DatabaseError(db_err) => loco_rs::Error::DB(db_err),
            GenerationError::ConfigError(msg) => loco_rs::Error::CustomError(
                StatusCode::PAYMENT_REQUIRED,
                ErrorDetail::new("".to_string(), msg.into()),
            ),
            GenerationError::SaveError(msg) => loco_rs::Error::InternalServerError,
            GenerationError::CreditUpdateError(msg) => loco_rs::Error::InternalServerError,
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
