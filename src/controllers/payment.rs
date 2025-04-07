#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query, Extension};
use loco_rs::{controller::ErrorDetail, prelude::*};
use serde::Deserialize;
use std::str::FromStr;
use stripe::{
    CheckoutSession, CheckoutSessionId, CheckoutSessionPaymentStatus, CheckoutSessionStatus,
    StripeError,
};
pub struct PaymentController;
use crate::{domain::website::Website, service::stripe::stripe::StripeClient, views};
use axum::{http::HeaderMap, http::StatusCode, response::IntoResponse, Json};

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Payment;
    impl Payment {
        pub const BASE: &'static str = "/payments";
        pub const API_PAYMENT_BASE: &'static str = "/";
        pub const API_STRIPE_SUCCESS: &'static str = "/success";
        pub const API_STRIPE_CANCEL: &'static str = "/cancel";
        pub const API_STRIPE_RETURN: &'static str = "/return";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Payment::BASE)
        .add(routes::Payment::API_PAYMENT_BASE, get(payment_home))
        .add(routes::Payment::API_STRIPE_SUCCESS, get(success_handler)) // Route for the success URL
        .add(routes::Payment::API_STRIPE_CANCEL, get(cancel_handler)) // Route for the cancel URL
}

#[derive(Deserialize, Debug)]
struct PaymentRedirectParams {
    session_id: Option<String>,
}

async fn success_handler(
    params: Query<PaymentRedirectParams>,
    Extension((stripe_client, website)): Extension<(StripeClient, Website)>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    // 1. Extract Session ID
    let session_id_str = params.session_id.as_ref().ok_or_else(|| {
        tracing::warn!("Success redirect received without session_id query parameter.");
        loco_rs::Error::BadRequest("Missing session_id query parameter".to_string())
    })?;

    // 2. Parse Session ID
    let session_id = CheckoutSessionId::from_str(session_id_str).map_err(|_| {
        tracing::warn!(session_id = %session_id_str, "Received invalid session_id format.");
        loco_rs::Error::BadRequest(format!("Invalid session_id format: {}", session_id_str))
    })?;

    // 4. Retrieve Session from Stripe
    tracing::info!(session_id = %session_id_str, "Retrieving checkout session from Stripe...");
    let session =
        CheckoutSession::retrieve(&stripe_client.client, &session_id, &[] /* expand */)
            .await
            .map_err(|_| loco_rs::Error::Message(String::from("Error checking storage: 101")))?;

    // 5. Check Session Status (Crucial!)
    let is_successful = session.payment_status == CheckoutSessionPaymentStatus::Paid
        || session.status == Some(CheckoutSessionStatus::Complete);

    if is_successful {
        tracing::info!(session_id = %session_id_str, "Checkout session verified successfully via redirect.");

        let customer_email = session
            .customer_details
            .and_then(|d| d.email)
            .unwrap_or_else(|| "N/A".to_string());

        views::payment::payment_success(v, &website)
    } else {
        tracing::warn!(session_id = %session_id_str, status = ?session.status, payment_status = ?session.payment_status, "Redirected session is not yet successful.");

        Err(loco_rs::Error::CustomError(
            StatusCode::NO_CONTENT,
            ErrorDetail::new("NO CONTENT", "Redirected session is not yet successful."),
        ))
    }
}

// --- Cancel Handler ---
async fn cancel_handler(
    Extension((stripe_client, website)): Extension<(StripeClient, Website)>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    tracing::info!("User cancelled payment process.");
    // Render a cancellation view/template
    views::payment::payment_cancel(v, &website)
}

#[debug_handler]
pub async fn payment_home(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::payment::payment_home(v, &website)
}
