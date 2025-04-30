#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query, http::HeaderMap, Extension};
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
        users::UserPid,
        PlanModel,
        _entities::sea_orm_active_enums::{CheckOutStatus, PlanNames},
        join::user_credits_models::load_user_and_credits,
        UserModel,
    },
    service::stripe::{stripe::StripeClient, stripe_builder::CheckoutSessionBuilder},
    views,
};
use axum::{http::StatusCode, response::IntoResponse};

use super::auth::routes::Auth as AuthRoutes;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PaymentRoutes {
        pub base: String,
        pub stripe_checkout_route: String,
        pub stripe_checkout_partial_route: String,
        pub stripe_payment_status_route: String,
        pub payment_plans: String,
        pub payment_plans_partial: String,
    }
    impl PaymentRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Payment::BASE),
                stripe_checkout_route: format!("{}{}", Payment::BASE, Payment::STRIPE_CHECKOUT),
                stripe_checkout_partial_route: format!(
                    "{}{}",
                    Payment::BASE,
                    Payment::STRIPE_CHECKOUT_PARTIAL
                ),
                stripe_payment_status_route: format!(
                    "{}{}",
                    Payment::BASE,
                    Payment::STRIPE_PAYMENT_STATUS
                ),
                payment_plans: format!("{}{}", Payment::BASE, Payment::PAYMENT_PLAN),
                payment_plans_partial: format!(
                    "{}{}",
                    Payment::BASE,
                    Payment::PAYMENT_PLAN_PARTIAL
                ),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Payment;
    impl Payment {
        pub const BASE: &'static str = "/payment";
        pub const API_PAYMENT_BASE: &'static str = "/";
        pub const API_STRIPE_PREPARE_ID: &'static str = "/prepare/{pid}/{plan}";
        pub const API_STRIPE_PREPARE: &'static str = "/prepare";
        pub const API_STRIPE_SUCCESS: &'static str = "/success";
        pub const API_STRIPE_CANCEL: &'static str = "/cancel";
        pub const API_STRIPE_RETURN: &'static str = "/return";
        pub const API_STRIPE_PAYMENT_PLAN_REQUEST: &'static str = "/plan/{pid}/{plan}";
        pub const PAYMENT_PLAN: &'static str = "/plan";
        pub const PAYMENT_PLAN_PARTIAL: &'static str = "/partial/plan";
        pub const STRIPE_PAYMENT_STATUS_ID: &'static str = "/processing/status/{session_id}";
        pub const STRIPE_PAYMENT_STATUS: &'static str = "/processing/status";
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
        .add(routes::Payment::PAYMENT_PLAN, get(payment_home))
        .add(
            routes::Payment::PAYMENT_PLAN_PARTIAL,
            get(payment_home_partial),
        )
        .add(
            routes::Payment::API_STRIPE_PAYMENT_PLAN_REQUEST,
            get(create_checkout_session),
        )
        .add(routes::Payment::API_STRIPE_PREPARE_ID, get(prepare_handler))
        .add(routes::Payment::API_STRIPE_SUCCESS, get(success_handler))
        .add(routes::Payment::API_STRIPE_CANCEL, get(cancel_handler))
        .add(
            routes::Payment::STRIPE_PAYMENT_STATUS_ID,
            get(status_handler),
        )
}

// async fn load_user(db: &DatabaseConnection, pid: &UserPid) -> Result<UserModel> {
//     let item = UserModel::find_by_pid(db, &pid.as_ref().to_string()).await?;
//     Ok(item)
// }
async fn load_plan(db: &DatabaseConnection, name: &PlanNames) -> Result<PlanModel> {
    let item = PlanModel::find_by_name(db, &name).await?;
    Ok(item)
}
async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
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

#[debug_handler]
pub async fn prepare_handler(
    _auth: auth::JWT,
    Path((pid, plan)): Path<(Uuid, PlanNames)>,
    State(_ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let link = format!(
        "{}{}/{}/{}",
        routes::Payment::BASE,
        routes::Payment::PAYMENT_PLAN,
        pid,
        plan
    );
    views::payment::prepare(v, &website, &link)
}

#[debug_handler]
pub async fn create_checkout_session(
    auth: auth::JWT,
    Path((pid, plan)): Path<(Uuid, PlanNames)>,
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;

    let mut headers = HeaderMap::new();
    if user.pid != pid {
        headers.insert("HX-Redirect", AuthRoutes::LOGIN.parse().unwrap());
        return Ok((StatusCode::OK, headers).into_response());
    }

    // Load plan, but give nicer error if not found
    let plan = load_plan(&ctx.db, &plan).await.map_err(|e| {
        tracing::error!("Failed to load plan: {:?}", e);
        loco_rs::Error::BadRequest("Invalid plan selected.".into())
    })?;

    let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
        .user(&user)
        .plan(&plan.plan_name)
        .metadata()
        .build()
        .await?;

    let session = stripe_checkout.url.ok_or_else(|| {
        loco_rs::Error::Message("Stripe Checkout Session created without a URL".to_string())
    })?;

    let session_url = session
        .parse()
        .map_err(|_| loco_rs::Error::Message("Failed to parse Stripe session URL".into()))?;

    headers.insert("HX-Redirect", session_url);
    Ok((StatusCode::OK, headers).into_response())
}

async fn success_handler(
    _auth: auth::JWT,
    params: Query<PaymentRedirectParams>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(_): State<AppContext>,
) -> Result<impl IntoResponse> {
    tracing::info!("User successfully completed payment process.");
    let status = CheckOutStatus::Processing;
    views::payment::stripe_status(v, &website, &params.session_id, status)
}

async fn cancel_handler(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    tracing::info!("User cancelled payment process.");
    let status = CheckOutStatus::Cancelled;
    views::payment::stripe_status(v, &website, &None, status)
}

async fn status_handler(
    Path(session_id_str): Path<String>,
    Extension(stripe_client): Extension<StripeClient>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    dbg!(&session_id_str);
    // 1. Parse Session ID
    let session_id = CheckoutSessionId::from_str(session_id_str.as_str()).map_err(|_| {
        tracing::warn!(session_id = %session_id_str, "Received invalid session_id format.");
        loco_rs::Error::BadRequest(format!("Invalid session_id format: {}", session_id_str))
    })?;

    // 2. Retrieve Session from Stripe
    tracing::info!(session_id = %session_id_str, "Retrieving checkout session from Stripe...");
    let session = CheckoutSession::retrieve(&stripe_client.client, &session_id, &[])
        .await
        .map_err(|_| loco_rs::Error::Message(String::from("Error checking storage: 101")))?;

    // 3. Check Session Status (Crucial!)
    let is_successful = session.payment_status == CheckoutSessionPaymentStatus::Paid
        || session.status == Some(CheckoutSessionStatus::Complete);

    // 4. Redirect Appropriately
    if is_successful {
        tracing::info!(session_id = %session_id_str, "Checkout session verified successfully via redirect.");
        let status = CheckOutStatus::Succeeded;
        views::payment::stripe_status_partials(v, &website, status)
    } else {
        tracing::warn!(session_id = %session_id_str, status = ?session.status, payment_status = ?session.payment_status, "Redirected session is not yet successful.");
        Err(loco_rs::Error::CustomError(
            StatusCode::NO_CONTENT,
            ErrorDetail::new("NO CONTENT", "Redirected session is not yet successful."),
        ))
    }
}

#[debug_handler]
pub async fn payment_home(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    views::payment::payment_home(v, &website, &user.into(), &user_credits.into())
}
#[debug_handler]
pub async fn payment_home_partial(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    views::payment::payment_home_partial(v, &website, &user.into(), &user_credits.into())
}

// ===================== Embedded Code ======================
// async fn checkout_return_handler(
//     Extension(website): Extension<Website>,
//     ViewEngine(v): ViewEngine<TeraView>,
// ) -> Result<impl IntoResponse> {
//     tracing::error!("User successfully completed payment process.");
//     views::payment::return_session(v, &website)
// }

// async fn checkout_partial_handler(
//     Path((pid, plan)): Path<(Uuid, PlanNames)>,
//     Extension(website): Extension<Website>,
//     Extension(stripe_client): Extension<StripeClient>,
//     State(ctx): State<AppContext>,
//     ViewEngine(v): ViewEngine<TeraView>,
// ) -> Result<impl IntoResponse> {
//     let user_pid = UserPid::new(pid);
//     let user = load_user(&ctx.db, &user_pid).await?;
//     let plan = load_plan(&ctx.db, &plan).await?;
//     let transaction_id = uuid::Uuid::new_v4();
//     let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
//         .user(&user)
//         .plan(&plan.plan_name)
//         .transaction_id(&transaction_id)
//         .embedded()
//         .build()
//         .await?;
//     let transaction = TransactionDomain::new(
//         transaction_id,
//         &user,
//         plan,
//         stripe_checkout.currency.clone(),
//         stripe_checkout.id.to_string(),
//     );
//     let _transaction = save_txn(&ctx.db, &transaction).await?;

//     let secret = match stripe_checkout.client_secret {
//         Some(secret) => secret,
//         None => {
//             return Err(loco_rs::Error::Message(
//                 "Unable to create checkout session".to_string(),
//             ));
//         }
//     };

//     let secret = ClientSecret::new(secret);
//     views::payment::checkout_session_partial(
//         v,
//         &website,
//         &stripe_client.stripe_publishable_key,
//         secret,
//     )
// }

// async fn checkout_handler(
//     Path((pid, plan)): Path<(Uuid, PlanNames)>,
//     Extension(website): Extension<Website>,
//     Extension(stripe_client): Extension<StripeClient>,
//     State(ctx): State<AppContext>,
//     ViewEngine(v): ViewEngine<TeraView>,
// ) -> Result<impl IntoResponse> {
//     let user_pid = UserPid::new(pid);
//     let user = load_user(&ctx.db, &user_pid).await?;
//     let plan = load_plan(&ctx.db, &plan).await?;
//     let transaction_id = uuid::Uuid::new_v4();
//     let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
//         .user(&user)
//         .plan(&plan.plan_name)
//         .transaction_id(&transaction_id)
//         .embedded()
//         .build()
//         .await?;
//     let transaction = TransactionDomain::new(
//         transaction_id,
//         &user,
//         plan,
//         stripe_checkout.currency.clone(),
//         stripe_checkout.id.to_string(),
//     );
//     let _transaction = save_txn(&ctx.db, &transaction).await?;

//     let secret = match stripe_checkout.client_secret {
//         Some(secret) => secret,
//         None => {
//             return Err(loco_rs::Error::Message(
//                 "Unable to create checkout session".to_string(),
//             ));
//         }
//     };

//     let secret = ClientSecret::new(secret);
//     views::payment::checkout_session(v, &website, &stripe_client.stripe_publishable_key, secret)
// }

// #[debug_handler]
// pub async fn create_checkout_session_handler(
//     Extension(stripe_client): Extension<StripeClient>,
//     State(ctx): State<AppContext>,
// ) -> Result<impl IntoResponse> {
//     let user_pid = UserPid::new("ab5e796c-a2cd-458e-ad6b-c3a898f44bd1");
//     let user = load_user(&ctx.db, &user_pid).await?;
//     let plan = PlanNames::Basic;
//     let plan = load_plan(&ctx.db, &plan).await?;
//     let transaction_id = uuid::Uuid::new_v4();
//     let stripe_checkout = CheckoutSessionBuilder::new(&stripe_client, &ctx.db)
//         .user(&user)
//         .plan(&plan.plan_name)
//         .transaction_id(&transaction_id)
//         .build()
//         .await?;
//     let transaction = TransactionDomain::new(
//         transaction_id,
//         &user,
//         plan,
//         stripe_checkout.currency.clone(),
//         stripe_checkout.id.to_string(),
//     );
//     let _transaction = save_txn(&ctx.db, &transaction).await?;

//     let secret = match stripe_checkout.client_secret {
//         Some(secret) => secret,
//         None => {
//             return Err(loco_rs::Error::Message(
//                 "Unable to create checkout session".to_string(),
//             ));
//         }
//     };

//     let secret = ClientSecret::new(secret);
//     format::json(secret)
// }
