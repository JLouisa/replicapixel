#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query, Extension};
use axum_extra::{headers::UserAgent, TypedHeader};
use derive_more::Constructor;
use loco_rs::{controller::ErrorDetail, prelude::*};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stripe::{
    Charge, CheckoutSession, CheckoutSessionId, CheckoutSessionPaymentStatus,
    CheckoutSessionStatus, Client, Expandable, PaymentIntent,
};
pub struct PaymentController;
use crate::{
    controllers::auth::HxRedirect,
    domain::website::{Website, WebsiteOptions},
    middleware::{client_ip::ClientIp, cookie::ExtractConsentState, i18nv2::LangEngine},
    models::{
        users::UserPid,
        PlanModel,
        _entities::sea_orm_active_enums::{CheckOutStatus, PlanNames},
        join::{user_credits_models::load_user_and_credits, user_order::load_user_and_order},
        UserModel,
    },
    service::{
        meta::meta::{EventData, UserData},
        stripe::{
            stripe::{StripeClient, StripeClientError},
            stripe_builder::StripeOptionsBuilder,
        },
    },
    views,
    workers::meta_worker::{MetaConversionApiWorker, MetaConversionApiWorkerArgs},
};
use axum::{http::StatusCode, response::IntoResponse};

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
        pub payment_stripe_url_from_order: String,
        pub payment_prepare_route: String,
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
                payment_stripe_url_from_order: format!(
                    "{}{}",
                    Payment::BASE,
                    Payment::STRIPE_RECEIPT_FROM_ORDER
                ),
                payment_prepare_route: format!("{}{}", Payment::BASE, Payment::API_STRIPE_PREPARE),
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
        pub const STRIPE_RECEIPT_FROM_ORDER_ID: &'static str = "/order/receipt/{order_id}";
        pub const STRIPE_RECEIPT_FROM_ORDER: &'static str = "/order/receipt";
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
        .add(
            routes::Payment::STRIPE_RECEIPT_FROM_ORDER_ID,
            get(get_receipt_url_for_order),
        )
}

async fn load_plan(db: &DatabaseConnection, name: &PlanNames) -> Result<PlanModel> {
    let item = PlanModel::find_by_name(db, &name).await?;
    Ok(item)
}
async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
    Ok(item)
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct StripeWebOptions {
    status: Option<CheckOutStatus>,
    session_id: Option<String>,
    client_secret: Option<String>,
}
impl StripeWebOptions {
    pub fn new() -> Self {
        StripeWebOptions::default()
    }
    pub fn status(self, status: CheckOutStatus) -> Self {
        Self {
            status: Some(status),
            ..self
        }
    }
    pub fn session_id<S: Into<String>>(self, key: S) -> Self {
        Self {
            session_id: Some(key.into()),
            ..self
        }
    }
    pub fn set_session_id(self, key: Option<String>) -> Self {
        Self {
            session_id: key,
            ..self
        }
    }
    pub fn client_secret<C: Into<String>>(self, client_secret: C) -> Self {
        Self {
            client_secret: Some(client_secret.into()),
            ..self
        }
    }
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
async fn get_receipt_url_for_order(
    auth: auth::JWT,
    Path(order_pid): Path<Uuid>,
    State(_ctx): State<AppContext>,
    Extension(stripe_client): Extension<StripeClient>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let client = Client::new(stripe_client.settings.stripe_secret_key);
    let (_, order) = load_user_and_order(&_ctx.db, &user_pid, &order_pid).await?;

    // 1. Retrieve the checkout session
    let checkout_session = CheckoutSessionId::from_str(&order.payment_id)
        .map_err(|e| StripeClientError::ParseId(e))?;
    let session = CheckoutSession::retrieve(&client, &checkout_session, &[])
        .await
        .map_err(|e| StripeClientError::StripeApi(e))?;

    // 2. Extract the PaymentIntent ID
    let payment_intent_expandable = session.payment_intent.ok_or(StripeClientError::Internal(
        "Payment intent missing".to_owned(),
    ))?;
    let pi_id = match payment_intent_expandable {
        Expandable::Id(id) => id,
        Expandable::Object(pi) => pi.id,
    };

    // 3. Retrieve the PaymentIntent
    let payment_intent = PaymentIntent::retrieve(&client, &pi_id, &[])
        .await
        .map_err(|e| StripeClientError::StripeApi(e))?;

    // 4. Extract the *latest* Charge ID from the PaymentIntent
    let latest_charge_expandable =
        payment_intent
            .latest_charge
            .ok_or(StripeClientError::Internal(
                "Payment intent has no associated charge (payment might be pending or failed)"
                    .to_owned(),
            ))?;
    let charge_id = match latest_charge_expandable {
        Expandable::Id(id) => id,
        Expandable::Object(ch) => ch.id,
    };

    // 5. Retrieve the Charge object using the Charge ID
    let charge = Charge::retrieve(&client, &charge_id, &[])
        .await
        .map_err(|e| StripeClientError::StripeApi(e))?;

    // 6. Get receipt_url from the charge
    let receipt_url = charge.receipt_url.ok_or(StripeClientError::Internal(
        "Charge found, but it does not have a receipt URL (charge might be uncaptured, failed, or type doesn't support receipts)"
            .to_owned(),
    ))?;

    return Ok((StatusCode::OK, receipt_url).into_response());
}

#[debug_handler]
pub async fn prepare_handler(
    Path((pid, plan)): Path<(Uuid, PlanNames)>,
    // State(ctx): State<AppContext>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let link = format!(
        "{}{}/{}/{}",
        routes::Payment::BASE,
        routes::Payment::PAYMENT_PLAN,
        pid,
        plan
    );

    let website_options = WebsiteOptions::new()
        .website(&website)
        .cc_cookie(&cc_cookie)
        .is_marketing_initiate_checkout()
        .language(&lang)
        .link(&link)
        .build();
    views::payment::prepare(v, &website_options)
}

#[debug_handler]
pub async fn create_checkout_session(
    ClientIp(client_ip): ClientIp,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Path((pid, plan)): Path<(Uuid, PlanNames)>,
    Extension(stripe_client): Extension<StripeClient>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&pid.to_string());
    let user = load_user(&ctx.db, &user_pid).await?;

    if user.pid != pid {
        return Ok(HxRedirect::login().into_response());
    }

    // Load plan, but give nicer error if not found
    let plan = load_plan(&ctx.db, &plan).await.map_err(|e| {
        tracing::error!("Failed to load plan: {:?}", e);
        loco_rs::Error::BadRequest("Invalid plan selected.".into())
    })?;

    let stripe_options = StripeOptionsBuilder::new()
        .user(&user)
        .plan(&plan)
        .metadata()
        .build();

    let stripe_checkout = stripe_client
        .create_checkout(&stripe_options, &ctx.db)
        .await?;

    let session = stripe_checkout.url.ok_or_else(|| {
        loco_rs::Error::Message("Stripe Checkout Session created without a URL".to_string())
    })?;

    // Send to queue for processing for Meta
    let user_data = UserData::new(&user)
        .client_user_agent(&user_agent)
        .client_ip_address(&client_ip);
    let meta = EventData::initiate_checkout(&user, &plan).set_user_data(&user_data);
    let worker_arg = MetaConversionApiWorkerArgs::new(meta, website.website_basic_info.meta_pixel);
    if let Err(e) = MetaConversionApiWorker::perform_later(&ctx, worker_arg).await {
        tracing::warn!("⚠️ Failed to queue MetaConversionApiWorker: {e}");
    }

    Ok(HxRedirect::new(&session).into_response())
}

async fn success_handler(
    // _auth: auth::JWT,
    params: Query<PaymentRedirectParams>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    tracing::info!(
        "User successfully completed payment process. session_id: {:?}",
        &params.session_id
    );
    let stripe_options = StripeWebOptions::new()
        .set_session_id(params.session_id.clone())
        .status(CheckOutStatus::Processing);
    let website_options = WebsiteOptions::new()
        .website(&website)
        .cc_cookie(&cc_cookie)
        .language(&lang)
        .stripe_options(&stripe_options)
        .build();
    views::payment::stripe_status(v, &website_options)
}

async fn cancel_handler(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    tracing::info!("User cancelled payment process.");
    let stripe_options = StripeWebOptions::new().status(CheckOutStatus::Cancelled);
    let website_options = WebsiteOptions::new()
        .website(&website)
        .cc_cookie(&cc_cookie)
        .language(&lang)
        .stripe_options(&stripe_options)
        .build();
    views::payment::stripe_status(v, &website_options)
}

async fn status_handler(
    Path(session_id_str): Path<String>,
    Extension(stripe_client): Extension<StripeClient>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    LangEngine(lang): LangEngine,
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
        let amount = match session.amount_total {
            Some(amount) => amount as f64 / 100.0,
            None => 9.99,
        };
        let stripe_options = StripeWebOptions::new().status(CheckOutStatus::Succeeded);
        let website_options = WebsiteOptions::new()
            .website(&website)
            .cc_cookie(&cc_cookie)
            .language(&lang)
            .stripe_options(&stripe_options)
            .is_marketing_purchase()
            .marketing_purchase(amount)
            .build();
        views::payment::stripe_status_partials(v, &website_options)
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
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let website_options = WebsiteOptions::new()
        .website(&website)
        .cc_cookie(&cc_cookie)
        .language(&lang)
        .is_marketing_initiate_checkout()
        .user(user.into())
        .user_credits(user_credits.into())
        .build();
    views::payment::payment_home(v, &website_options)
}
#[debug_handler]
pub async fn payment_home_partial(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?;
    let website_options = WebsiteOptions::new()
        .website(&website)
        .cc_cookie(&cc_cookie)
        .language(&lang)
        .is_marketing_initiate_checkout()
        .user(user.into())
        .user_credits(user_credits.into())
        .build();
    views::payment::payment_home_partial(v, &website_options)
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
