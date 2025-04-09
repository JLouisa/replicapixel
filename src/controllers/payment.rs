#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query, response::Redirect, Extension};
use derive_more::Constructor;
use loco_rs::{controller::ErrorDetail, prelude::*};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stripe::{
    CheckoutSession, CheckoutSessionId, CheckoutSessionPaymentStatus, CheckoutSessionStatus,
};
pub struct PaymentController;
use crate::{
    domain::website::Website,
    models::{
        transactions::TransactionDomain, users::UserPid, PlanModel, TransactionActiveModel,
        UserModel, _entities::sea_orm_active_enums::PlanNames,
    },
    service::stripe::{
        stripe::StripeClient, stripe_builder::CheckoutSessionBuilder,
        stripe_status_service::StripeStatusService,
    },
    views,
};
use axum::{http::StatusCode, response::IntoResponse};

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Payment;
    impl Payment {
        pub const BASE: &'static str = "/payment";
        pub const API_PAYMENT_BASE: &'static str = "/";
        pub const API_STRIPE_SUCCESS: &'static str = "/success";
        pub const API_STRIPE_CANCEL: &'static str = "/cancel";
        pub const API_STRIPE_RETURN: &'static str = "/return";
        pub const API_STRIPE_PAYMENT_PLAN_REQUEST: &'static str = "/plan/{pid}/{plan}";
        pub const API_STRIPE_PAYMENT_PLAN: &'static str = "/plan";
        pub const API_STRIPE_PAYMENT_PLAN_PARTIAL: &'static str = "/partial/plan";
        pub const STRIPE_PAYMENT_STATUS: &'static str = "/processing/status/{session_id}";

        pub const STRIPE_CREATE_CHECKOUT: &'static str = "/create-checkout-session";
        pub const STRIPE_CHECKOUT: &'static str = "/checkout";
        pub const STRIPE_CHECKOUT_PARTIAL: &'static str = "/checkout/partial";
        pub const STRIPE_CHECKOUT_ID: &'static str = "/checkout/{pid}/{plan}";
        pub const STRIPE_CHECKOUT_ID_PARTIAL: &'static str = "/checkout/partial/{pid}/{plan}";
        pub const STRIPE_CHECKOUT_RETURN: &'static str = "v1/return";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Payment::BASE)
        .add(routes::Payment::API_STRIPE_PAYMENT_PLAN, get(payment_home))
        .add(
            routes::Payment::API_STRIPE_PAYMENT_PLAN_PARTIAL,
            get(payment_home_partial),
        )
        .add(routes::Payment::API_STRIPE_SUCCESS, get(success_handler)) // Route for the success URL
        .add(routes::Payment::API_STRIPE_CANCEL, get(cancel_handler)) // Route for the cancel URL
        .add(routes::Payment::STRIPE_PAYMENT_STATUS, get(status_handler)) // Route for the cancel URL
        .add(
            routes::Payment::API_STRIPE_PAYMENT_PLAN_REQUEST,
            get(payment_request),
        )
        .add(routes::Payment::STRIPE_CHECKOUT_ID, get(checkout_handler)) // Route for the cancel URL
        .add(
            routes::Payment::STRIPE_CHECKOUT_ID_PARTIAL,
            get(checkout_partial_handler),
        ) // Route for the cancel URL
        .add(
            routes::Payment::STRIPE_CREATE_CHECKOUT,
            post(create_checkout_session_handler),
        )
        .add(
            routes::Payment::STRIPE_CHECKOUT_RETURN,
            get(checkout_return_handler),
        ) // Route for the cancel URL
}

async fn load_user(db: &DatabaseConnection, pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, &pid.as_ref().to_string()).await?;
    Ok(item)
}
async fn load_plan(db: &DatabaseConnection, name: &PlanNames) -> Result<PlanModel> {
    let item = PlanModel::find_by_name(db, &name).await?;
    Ok(item)
}
async fn save_txn(
    db: &DatabaseConnection,
    transaction: &TransactionDomain,
) -> Result<TransactionActiveModel> {
    let item = TransactionActiveModel::save(db, &transaction).await?;
    Ok(item)
}

#[derive(Deserialize, Debug)]
struct PaymentRedirectParams {
    session_id: Option<String>,
}

#[derive(Serialize, Debug, Clone, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecret {
    pub client_secret: String,
}

async fn checkout_return_handler(
    params: Query<PaymentRedirectParams>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    tracing::error!("User successfully completed payment process.");
    views::payment::return_session(v, &website, &params.session_id)
}

async fn checkout_partial_handler(
    Path((pid, plan)): Path<(Uuid, PlanNames)>,
    Extension(website): Extension<Website>,
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let plan = load_plan(&ctx.db, &plan).await?;
    let transaction_id = uuid::Uuid::new_v4();
    let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
        .user(&user)
        .plan(&plan.plan_name)
        .transaction_id(&transaction_id)
        .embedded()
        .build()
        .await?;
    let transaction = TransactionDomain::new(
        transaction_id,
        &user,
        plan,
        stripe_checkout.currency.clone(),
        stripe_checkout.id.to_string(),
    );
    let _transaction = save_txn(&ctx.db, &transaction).await?;

    let secret = match stripe_checkout.client_secret {
        Some(secret) => secret,
        None => {
            return Err(loco_rs::Error::Message(
                "Unable to create checkout session".to_string(),
            ));
        }
    };

    let secret = ClientSecret::new(secret);
    views::payment::checkout_session_partial(
        v,
        &website,
        &stripe_client.stripe_publishable_key,
        secret,
    )
}

async fn checkout_handler(
    Path((pid, plan)): Path<(Uuid, PlanNames)>,
    Extension(website): Extension<Website>,
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let plan = load_plan(&ctx.db, &plan).await?;
    let transaction_id = uuid::Uuid::new_v4();
    let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
        .user(&user)
        .plan(&plan.plan_name)
        .transaction_id(&transaction_id)
        .embedded()
        .build()
        .await?;
    let transaction = TransactionDomain::new(
        transaction_id,
        &user,
        plan,
        stripe_checkout.currency.clone(),
        stripe_checkout.id.to_string(),
    );
    let _transaction = save_txn(&ctx.db, &transaction).await?;

    let secret = match stripe_checkout.client_secret {
        Some(secret) => secret,
        None => {
            return Err(loco_rs::Error::Message(
                "Unable to create checkout session".to_string(),
            ));
        }
    };

    let secret = ClientSecret::new(secret);
    views::payment::checkout_session(v, &website, &stripe_client.stripe_publishable_key, secret)
}

#[debug_handler]
pub async fn create_checkout_session_handler(
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("ab5e796c-a2cd-458e-ad6b-c3a898f44bd1");
    let user = load_user(&ctx.db, &user_pid).await?;
    let plan = PlanNames::Basic;
    let plan = load_plan(&ctx.db, &plan).await?;
    let transaction_id = uuid::Uuid::new_v4();
    let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
        .user(&user)
        .plan(&plan.plan_name)
        .transaction_id(&transaction_id)
        .embedded()
        .build()
        .await?;
    let transaction = TransactionDomain::new(
        transaction_id,
        &user,
        plan,
        stripe_checkout.currency.clone(),
        stripe_checkout.id.to_string(),
    );
    let _transaction = save_txn(&ctx.db, &transaction).await?;

    let secret = match stripe_checkout.client_secret {
        Some(secret) => secret,
        None => {
            return Err(loco_rs::Error::Message(
                "Unable to create checkout session".to_string(),
            ));
        }
    };

    let secret = ClientSecret::new(secret);
    format::json(secret)
}

async fn success_handler(
    // auth: auth::JWT,
    params: Query<PaymentRedirectParams>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    tracing::error!("User successfully completed payment process.");
    views::payment::payment_status(v, &website, &params.session_id)
}

#[debug_handler]
pub async fn payment_request(
    auth: auth::JWT,
    Path((pid, plan)): Path<(Uuid, PlanNames)>,
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    if user.pid != pid {
        return Err(loco_rs::Error::Message(
            "You are not authorized to access this page".to_string(),
        ));
    }
    let plan = load_plan(&ctx.db, &plan).await?;
    let transaction_id = uuid::Uuid::new_v4();
    let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
        .user(&user)
        .plan(&plan.plan_name)
        .transaction_id(&transaction_id)
        .build()
        .await?;
    let transaction = TransactionDomain::new(
        transaction_id,
        &user,
        plan,
        stripe_checkout.currency.clone(),
        stripe_checkout.id.to_string(),
    );
    let _transaction = save_txn(&ctx.db, &transaction).await?;

    let session = match stripe_checkout.url {
        Some(session) => session,
        None => {
            return Err(loco_rs::Error::Message(
                "Stripe Checkout Session created without a URL".to_string(),
            ))
        }
    };
    Ok(Redirect::to(&session).into_response())
}

async fn status_embedded_handler(
    Path(session_id_str): Path<String>,
    Extension(stripe_client): Extension<StripeClient>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    // 2. Parse Session ID
    let session_id = CheckoutSessionId::from_str(session_id_str.as_str()).map_err(|_| {
        tracing::warn!(session_id = %session_id_str, "Received invalid session_id format.");
        loco_rs::Error::BadRequest(format!("Invalid session_id format: {}", session_id_str))
    })?;

    let session = StripeStatusService::handle_status(&session_id, &stripe_client).await?;

    format::json(session)
}

async fn status_handler(
    Path(session_id_str): Path<String>,
    Extension(stripe_client): Extension<StripeClient>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    dbg!(&session_id_str);
    // 2. Parse Session ID
    let session_id = CheckoutSessionId::from_str(session_id_str.as_str()).map_err(|_| {
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
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    tracing::info!("User cancelled payment process.");
    // Render a cancellation view/template
    views::payment::payment_cancel(v, &website)
}

#[debug_handler]
pub async fn payment_home_partial(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let route = format!(
        "{}{}",
        routes::Payment::BASE,
        routes::Payment::STRIPE_CHECKOUT
    );
    views::payment::payment_home_partial(v, &website, &user.into(), route)
}

#[debug_handler]
pub async fn payment_home(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let route = format!(
        "{}{}",
        routes::Payment::BASE,
        routes::Payment::STRIPE_CHECKOUT
    );
    views::payment::payment_home(v, &website, &user.into(), route)
}
