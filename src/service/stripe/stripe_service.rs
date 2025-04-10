#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::{StripeEventActiveModel, StripeEventModel, TransactionModel, UserCreditModel};
use loco_rs::prelude::*;

use stripe::{CheckoutSessionPaymentStatus, Event, EventObject, EventType};
use thiserror::Error;

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
async fn load_txn(pid: &uuid::Uuid, db: &impl ConnectionTrait) -> Result<TransactionModel> {
    let item = TransactionModel::find_by_pid(&pid, db).await?;
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
pub struct StripeWebhookService;

impl StripeWebhookService {
    pub async fn handle_webhook(event: Event, ctx: &AppContext) -> Result<(), StripeServiceError> {
        match event.type_ {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    let session_id = session.id;
                    let transaction_id = session
                        .client_reference_id
                        .as_deref() // Get as &str
                        .ok_or(StripeServiceError::TransactionIdMissing)?;
                    let transaction_id = Uuid::parse_str(transaction_id)?;

                    // let session_map = session
                    //     .metadata
                    //     .as_ref()
                    //     .ok_or(StripeServiceError::MetadataMissing)?;
                    // let plan = session_map.get("plan");
                    // let user_id = session_map.get("user_id");
                    // let email = session_map.get("email");
                    match session.payment_status {
                        CheckoutSessionPaymentStatus::Paid => {
                            // 0. Check if the transaction has already been handled
                            let hse = load_hse(&session_id.to_string(), &ctx.db).await;
                            if hse.is_ok() {
                                return Ok(());
                            }

                            // 1. Get the transaction
                            let load_transaction = load_txn(&transaction_id, &ctx.db).await?;
                            if load_transaction.status == Status::Completed {
                                return Ok(());
                            }

                            let db_txn = ctx.db.begin().await?;

                            // 2. Update order/payment record in DB
                            let complete_transaction =
                                update_txn_completed(load_transaction, &db_txn).await?;
                            // 3. Update credits
                            let credits =
                                load_credits(complete_transaction.user_id, &db_txn).await?;
                            // Update credits
                            credits
                                .update_credits_with_transaction(complete_transaction, &db_txn)
                                .await?;
                            // 4. Create a handled stripe event
                            create_hse(&session_id.to_string(), &db_txn).await?;

                            // 5. Optionally send confirmation email
                            // send_confirmation_email(email, &session_id).await?;

                            db_txn.commit().await?;
                            return Ok(());
                        }
                        CheckoutSessionPaymentStatus::NoPaymentRequired => {
                            println!("No payment required for checkout session {}", &session_id);
                            return Ok(());
                        }
                        CheckoutSessionPaymentStatus::Unpaid => {
                            tracing::warn!(
                                "Payment unpaid/failed for checkout session {}. Internal Tx ID: {}",
                                &session_id,
                                transaction_id
                            );

                            // 1. Get the transaction
                            let internal_tx = load_txn(&transaction_id, &ctx.db).await?;

                            if internal_tx.status == Status::Pending {
                                tracing::info!(
                                    "Updating internal transaction {} status to Failed for unpaid session {}.",
                                    transaction_id,
                                    session_id
                                );

                                // 2. Begin DB transaction
                                let db_txn = ctx.db.begin().await?;

                                // 3. Update internal transaction status
                                update_txn_failed(internal_tx, &db_txn).await?;

                                // 4. End DB transaction
                                db_txn.commit().await?;
                            } else {
                                // Transaction was already Completed, Failed, etc. - no action needed.
                                tracing::info!(
                                    "Internal transaction {} already in status {:?}. No status change needed for unpaid session {}.",
                                    transaction_id,
                                    internal_tx.status,
                                    session_id
                                );
                                return Ok(());
                            }
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
