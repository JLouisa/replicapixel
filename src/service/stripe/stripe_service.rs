#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::models::_entities::sea_orm_active_enums::{PlanNames, Status};
use crate::models::join::user_credits_models::{load_user_and_credits, JoinError};
use crate::models::transactions::TransactionDomain;
use crate::models::users::UserPid;
use crate::models::{
    PlanModel, StripeEventActiveModel, StripeEventModel, TransactionActiveModel, TransactionModel,
    UserCreditModel, UserModel,
};

use std::collections::HashMap;
use stripe::{CheckoutSessionPaymentStatus, Event, EventObject, EventType};
use thiserror::Error;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum StripeServiceError {
    #[error("Stripe-signature missing")]
    SignatureError,
    #[error("Stripe-signature failed verification: {0}")]
    SignatureVerifyError(String),
    #[error("Unauthorized access to training model")]
    Unauthorized,
    #[error("No transactionId found")]
    TransactionIdMissing,
    #[error("Failed to parse ID: {0}")]
    ParseId(#[from] uuid::Error),
    #[error("Unexpected object type: {0}")]
    UnexpectedObject(String),
    #[error("Missing metadata")]
    MetadataMissing,
    #[error("Missing metadata field: {0}")]
    MissingMetadataField(String),
    #[error("Failed to create transaction: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Database interaction failed: {0}")]
    DbModel(#[from] ModelError),
    #[error("Internal client error: {0}")]
    LocoErr(#[from] loco_rs::Error),
    #[error("Internal client error: {0}")]
    JoinError(#[from] JoinError),
}

async fn update_txn_completed(
    item: TransactionModel,
    db: &impl ConnectionTrait,
) -> Result<TransactionModel> {
    let item = TransactionModel::status_completed(item, db).await?;
    Ok(item)
}
async fn update_txn_failed(
    item: TransactionModel,
    db: &impl ConnectionTrait,
) -> Result<TransactionModel> {
    let item = TransactionModel::status_failed(item, db).await?;
    Ok(item)
}
async fn load_txn_webhook(
    pid: &uuid::Uuid,
    db: &impl ConnectionTrait,
) -> Result<Option<TransactionModel>> {
    let item = TransactionModel::find_by_pid_webhook(&pid, db).await?;
    Ok(item)
}
async fn load_credits(id: i32, db: &impl ConnectionTrait) -> Result<UserCreditModel> {
    let item = UserCreditModel::find_by_user_id(db, id).await?;
    Ok(item)
}
async fn load_hse(id: &str, db: &impl ConnectionTrait) -> Result<StripeEventModel> {
    let item = StripeEventModel::find_by_session_id(id, db).await?;
    Ok(item)
}
async fn create_hse(id: &str, db: &impl ConnectionTrait) -> Result<StripeEventActiveModel> {
    let item = StripeEventActiveModel::save(id, db).await?;
    Ok(item)
}
async fn load_plan(db: &impl ConnectionTrait, name: &String) -> Result<PlanModel> {
    let item = PlanModel::find_by_name_string(db, &name).await?;
    Ok(item)
}
async fn save_txn(
    db: &impl ConnectionTrait,
    transaction: &TransactionDomain,
) -> Result<TransactionModel> {
    let item = TransactionActiveModel::save(db, &transaction).await?;
    Ok(item)
}

async fn extract_and_process_metadata(
    db_txn: &impl ConnectionTrait,
    session: &stripe::CheckoutSession,
) -> Result<(UserModel, UserCreditModel, PlanModel), StripeServiceError> {
    let user_pid = UserPid::new(
        session
            .client_reference_id
            .as_deref()
            .ok_or(StripeServiceError::TransactionIdMissing)?
            .parse::<Uuid>()
            .map_err(|e| {
                tracing::error!("Webhook {}: Invalid user_pid format '{}'", &session.id, e);
                loco_rs::Error::BadRequest("Invalid user_pid format".into())
            })?,
    );

    let metadata = session.metadata.as_ref().ok_or_else(|| {
        tracing::error!("Webhook {}: Missing metadata", &session.id);
        loco_rs::Error::BadRequest("Missing metadata".into())
    })?;
    let plan_name_str = metadata.get("plan_name").ok_or_else(|| {
        tracing::error!("Webhook {}: Missing plan_name in metadata", &session.id);
        loco_rs::Error::BadRequest("Missing plan_name".into())
    })?;

    let (user, user_credits) = load_user_and_credits(db_txn, &user_pid).await?;
    let plan = load_plan(db_txn, &plan_name_str).await?;

    Ok((user, user_credits, plan))
}

pub struct StripeWebhookService;
impl StripeWebhookService {
    pub async fn handle_webhook(event: Event, ctx: &AppContext) -> Result<(), StripeServiceError> {
        match event.type_ {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    let session_id = session.id.clone();
                    match session.payment_status {
                        CheckoutSessionPaymentStatus::Paid => {
                            // --- Idempotency Check (Essential) ---
                            let hse_check = load_hse(&session_id, &ctx.db).await;
                            if hse_check.is_ok() {
                                tracing::info!(
                                    "Webhook {}: Event already handled (HSE found).",
                                    &session_id
                                );
                                return Ok(()); // Success, already done.
                            }

                            // --- Start Database Transaction ---
                            let db_txn = ctx.db.begin().await?;

                            // Extract and process Metadata
                            let (user, mut user_credits, plan) =
                                extract_and_process_metadata(&db_txn, &session).await?;

                            // --- Create the Transaction Record ---
                            let amount = session.amount_total.ok_or_else(|| {
                                let msg = format!(
                                    "Webhook {}: Missing amount_total for Paid session!",
                                    &session.id
                                );
                                tracing::error!(msg);
                                loco_rs::Error::BadRequest(msg.into())
                            })?;

                            let transaction = TransactionDomain::new(
                                &user,
                                plan,
                                session.currency,
                                session_id.to_string(),
                                amount,
                                Some(Status::Completed),
                            );

                            // Save the newly created transaction record
                            let saved_transaction = save_txn(&db_txn, &transaction).await?;

                            // --- Update User Credits/Entitlements ---
                            user_credits
                                .update_credits_with_transaction(&saved_transaction, &db_txn)
                                .await?;

                            // --- Create Handled Stripe Event (for Idempotency) ---
                            create_hse(&session_id.to_string(), &db_txn).await?;

                            // --- Commit Database Transaction ---
                            db_txn.commit().await?;

                            tracing::info!("Webhook {}: Successfully processed. Transaction {} created and credits updated for user {}.", &session_id, &saved_transaction.pid, user.pid);

                            //Todo send_confirmation_email(user.email, &session_id).await?;

                            return Ok(());
                        }
                        CheckoutSessionPaymentStatus::NoPaymentRequired => {
                            tracing::info!(
                                "No payment required for checkout session {}",
                                &session_id
                            );
                            return Ok(());
                        }
                        CheckoutSessionPaymentStatus::Unpaid => {
                            tracing::info!(
                                "Payment unpaid/failed for checkout session {}.",
                                &session_id,
                            );
                            return Ok(());
                        }
                    }
                }
            }
            EventType::PaymentIntentSucceeded => {
                if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                    tracing::info!(
                        "Handling PaymentIntentSucceeded: PI ID {}",
                        payment_intent.id
                    );
                    // TODO: Handle successful payment (e.g., if not using Checkout Sessions)
                } else {
                    tracing::error!("Mismatched event object for PaymentIntentSucceeded");
                    return Ok(());
                }
            }
            // ... handle other events like payment_intent.payment_failed, invoice.paid, etc.
            _ => {
                // Log unhandled event types for debugging/future implementation
                tracing::warn!("Received unhandled Stripe event type: {}", event.type_);
                return Ok(());
            }
        }

        return Ok(());
    }
}
