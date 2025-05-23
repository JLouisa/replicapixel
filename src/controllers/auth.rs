#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    domain::website::Website,
    mailers::{
        auth::AuthMailer,
        transaction::{CheckoutCompletedEmailData, CheckoutMailer},
    },
    middleware::cookie::ExtractConsentState,
    models::{
        _entities::users,
        join::user_credits_models::{load_user_and_credits, load_user_credit_training},
        users::{LoginParams, PasswordChangeParams, RegisterParams, UserPid},
        PlanModel, TransactionModel, UserModel,
    },
    service::stripe::stripe::StripeClient,
    views::{
        self,
        auth::{CurrentResponse, LoginResponse},
    },
};
use axum::{
    body::Body,
    debug_handler,
    extract::{Json, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use chrono::{Duration, Utc};
use derive_more::Constructor;
use loco_rs::{controller::ErrorDetail, prelude::*};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, sync::OnceLock};
use validator::ValidationErrorsKind;

use axum_extra::extract::cookie::{Cookie as AxumCookie, SameSite};

pub static EMAIL_DOMAIN_RE: OnceLock<Regex> = OnceLock::new();

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AuthRoutes {
        pub login: String,
        pub login_partial: String,
        pub api_login: String,
        pub register: String,
        pub register_partial: String,
        pub api_register: String,
        pub forgot: String,
        pub forgot_partial: String,
        pub api_forgot: String,
        pub api_logout: String,
        pub logout_partial: String,
        pub change_password: String,
        pub api_check_user: String,
    }
    impl AuthRoutes {
        pub fn init() -> Self {
            Self {
                login: String::from(Auth::LOGIN),
                login_partial: String::from(Auth::LOGIN_PARTIAL),
                api_login: String::from(Auth::API_LOGIN),
                register: String::from(Auth::REGISTER),
                register_partial: String::from(Auth::REGISTER_PARTIAL),
                api_register: String::from(Auth::API_REGISTER),
                forgot: String::from(Auth::FORGOT),
                forgot_partial: String::from(Auth::FORGOT_PARTIAL),
                api_forgot: String::from(Auth::API_FORGOT),
                api_logout: String::from(Auth::API_LOGOUT),
                logout_partial: String::from(Auth::LOGOUT_PARTIAL),
                change_password: String::from(Auth::API_PASSWORD_CHANGE),
                api_check_user: String::from(Auth::API_CHECK_USER),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Auth;
    impl Auth {
        pub const LOGIN: &'static str = "/login";
        pub const REGISTER: &'static str = "/register";
        pub const FORGOT: &'static str = "/forgot";
        pub const LOGIN_PARTIAL: &'static str = "/partial/login";
        pub const LOGIN_HOME_PARTIAL: &'static str = "/partial/home/login";
        pub const REGISTER_PARTIAL: &'static str = "/partial/register";
        pub const FORGOT_PARTIAL: &'static str = "/partial/forgot";
        pub const LOGOUT_PARTIAL: &'static str = "/partial/logout";
        pub const API_REGISTER: &'static str = "/api/auth/register";
        pub const API_VERIFY_TOKEN: &'static str = "/api/auth/verify/{token}";
        pub const API_VERIFY_W_TOKEN: &'static str = "/api/auth/verify";
        pub const API_VERIFY_RESEND: &'static str = "/auth/resend-verification";
        pub const API_LOGIN: &'static str = "/api/auth/login";
        pub const API_LOGOUT: &'static str = "/api/auth/logout";
        pub const API_FORGOT: &'static str = "/api/auth/forgot";
        pub const API_RESET: &'static str = "/api/auth/reset";
        pub const API_CURRENT: &'static str = "/api/auth/current";
        pub const API_MAGIC_LINK: &'static str = "/api/auth/magic-link";
        pub const API_MAGIC_LINK_W_TOKEN: &'static str = "/api/auth/magic";
        pub const API_MAGIC_LINK_TOKEN: &'static str = "/api/auth/magic/{token}";
        pub const API_PASSWORD_CHANGE_ID: &'static str = "/api/auth/password-change/{id}";
        pub const API_PASSWORD_CHANGE: &'static str = "/api/auth/password-change";
        pub const API_CHECK_USER: &'static str = "/api/auth/check-user";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add(routes::Auth::LOGIN, get(get_login))
        .add(routes::Auth::REGISTER, get(get_register))
        .add(routes::Auth::FORGOT, get(get_forgot))
        .add(routes::Auth::LOGIN_PARTIAL, get(partial_login))
        .add(routes::Auth::REGISTER_PARTIAL, get(partial_register))
        .add(routes::Auth::FORGOT_PARTIAL, get(partial_forgot))
        .add(routes::Auth::LOGOUT_PARTIAL, get(logout_partial))
        .add(routes::Auth::API_REGISTER, post(register))
        .add(routes::Auth::API_VERIFY_TOKEN, get(verify))
        .add(
            routes::Auth::API_VERIFY_RESEND,
            post(resent_verification_token),
        )
        .add(routes::Auth::API_LOGIN, post(api_login))
        .add(routes::Auth::API_LOGOUT, get(logout))
        .add(routes::Auth::API_FORGOT, post(forgot))
        .add(routes::Auth::API_RESET, post(reset))
        .add(routes::Auth::API_CURRENT, get(current))
        .add(routes::Auth::API_MAGIC_LINK, post(magic_link))
        .add(routes::Auth::API_MAGIC_LINK_W_TOKEN, get(magic_link_verify))
        .add(routes::Auth::API_PASSWORD_CHANGE_ID, post(change_password))
        .add(routes::Auth::API_CHECK_USER, get(check_user))
    // .add("/api/auth/test/welcome", get(test_welcome_mail))
    // .add("/api/auth/test/forgot_password", get(test_forgot_password))
    // .add("/api/auth/test/magic_link", get(test_magic_link))
    // .add("/api/auth/test/transaction", get(test_transaction))
}

pub struct HxRedirect(String);
impl IntoResponse for HxRedirect {
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Redirect", self.0.parse().unwrap());
        (headers, StatusCode::OK).into_response()
    }
    // Ok(HxRedirect(routes::Auth::LOGIN_PARTIAL.to_string()).into_response())
}

#[derive(Debug, Deserialize, Constructor, Serialize, Default)]
struct AuthError {
    general: Option<String>,
    login: Option<String>,
    register: Option<String>,
    register_name: Option<String>,
    register_email: Option<String>,
    register_password: Option<String>,
    verify: Option<String>,
    forgot: Option<String>,
    logout: Option<String>,
}
impl AuthError {
    pub fn login_error(&self) -> Self {
        Self {
            login: Some(String::from("Email or password is incorrect")),
            ..Default::default()
        }
    }
    pub fn register_msg(&self, err: &str) -> Self {
        Self {
            general: Some(String::from(err)),
            ..Default::default()
        }
    }
    pub fn register_email(&self, err: &str) -> Self {
        Self {
            register_email: Some(String::from(err)),
            ..Default::default()
        }
    }
    pub fn register_error(&self, error: &HashMap<Cow<'static, str>, ValidationErrorsKind>) -> Self {
        Self {
            register_name: error.get("name").and_then(|kind| match kind {
                ValidationErrorsKind::Field(vec) => vec
                    .get(0)
                    .and_then(|f| f.message.as_ref())
                    .map(|m| m.to_string()),
                _ => None,
            }),
            register_email: error.get("email").and_then(|kind| match kind {
                ValidationErrorsKind::Field(vec) => vec
                    .get(0)
                    .and_then(|f| f.message.as_ref())
                    .map(|m| m.to_string()),
                _ => None,
            }),
            register_password: error.get("password").and_then(|kind| match kind {
                ValidationErrorsKind::Field(vec) => vec
                    .get(0)
                    .and_then(|f| f.message.as_ref())
                    .map(|m| m.to_string()),
                _ => None,
            }),

            ..Default::default()
        }
    }
    pub fn verify_error(&self) -> Self {
        Self {
            general: Some(String::from("Email is not verified")),
            ..Default::default()
        }
    }
}

fn get_allow_email_domain_re() -> &'static Regex {
    EMAIL_DOMAIN_RE.get_or_init(|| {
        Regex::new(r"@example\.com$|@gmail\.com$").expect("Failed to compile regex")
    })
}
async fn load_plan(db: &impl ConnectionTrait, name: &String) -> Result<PlanModel> {
    let item = PlanModel::find_by_name_string(db, &name).await?;
    Ok(item)
}
async fn load_transaction(db: &impl ConnectionTrait, name: &Uuid) -> Result<TransactionModel> {
    let item = TransactionModel::find_by_pid(name, db).await?;
    Ok(item)
}
async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
    Ok(item)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagicLinkParams {
    pub email: String,
}

#[debug_handler]
pub async fn test_transaction(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("ab5e796c-a2cd-458e-ad6b-c3a898f44bd1");
    let transaction_pid: Uuid = "c8b9233b-e18d-482d-9307-ed0c1b694cd7".parse().unwrap();
    let plan_name_str = "Premium".to_string();
    let user = load_user(&ctx.db, &user_pid).await?;
    let plan = load_plan(&ctx.db, &plan_name_str).await?;
    let transaction = load_transaction(&ctx.db, &transaction_pid).await?;

    let collection = CheckoutCompletedEmailData {
        user,
        transaction,
        plan,
        stripe_receipt_url: None,
    };

    CheckoutMailer::send_checkout_completed(&ctx, &website.website_basic_info, &collection).await?;
    Ok((StatusCode::OK).into_response())
}
#[debug_handler]
pub async fn test_welcome_mail(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("ab5e796c-a2cd-458e-ad6b-c3a898f44bd1");
    let user = load_user(&ctx.db, &user_pid).await?;
    AuthMailer::send_welcome(&ctx, &user, &website.website_basic_info).await?;
    Ok((StatusCode::OK).into_response())
}
#[debug_handler]
pub async fn test_forgot_password(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("ab5e796c-a2cd-458e-ad6b-c3a898f44bd1");
    let user = load_user(&ctx.db, &user_pid).await?;
    AuthMailer::forgot_password(&ctx, &user, &website.website_basic_info).await?;
    Ok((StatusCode::OK).into_response())
}
#[debug_handler]
pub async fn test_magic_link(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("ab5e796c-a2cd-458e-ad6b-c3a898f44bd1");
    let user = load_user(&ctx.db, &user_pid).await?;
    AuthMailer::send_magic_link(&ctx, &user, &website.website_basic_info).await?;
    Ok((StatusCode::OK).into_response())
}
#[debug_handler]
pub async fn change_password(
    auth: auth::JWT,
    Path(pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(params): Json<PasswordChangeParams>,
) -> Result<impl IntoResponse> {
    params.validate()?;

    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;

    if user.pid != pid {
        return Err(Error::Unauthorized(
            "Unauthorized to change password".to_string(),
        ));
    };

    let valid = user.verify_password(&params.current_password);

    if !valid {
        let msg = Some(String::from("There was an error with your password"));
        return views::settings::password_change(v, &website, &user.into(), msg);
    }

    let user = user
        .into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    views::settings::password_change(v, &website, &user.into(), None)
}

#[debug_handler]
pub async fn check_user(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    if auth.is_err() {
        return format::render().view(
            &v,
            "partials/parts/google_ott.html",
            data!({"website": website, "is_home": true}),
        );
    }
    let user_pid = UserPid::new(&auth.unwrap().claims.pid);
    let (user, user_credits) = match load_user_and_credits(&ctx.db, &user_pid).await {
        Ok((user, user_credits)) => (user, user_credits),
        Err(_) => {
            return format::render().view(
                &v,
                "partials/parts/google_ott.html",
                data!({"website": website, "is_home": true}),
            );
        }
    };
    format::render().view(
        &v,
        "partials/parts/home_validated.html",
        data!({"website": website, "user": user, "credits": user_credits, "is_home": true}),
    )
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    Extension(stripe_client): Extension<StripeClient>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    if let Err(err) = params.validate() {
        let error_msg = AuthError::default().register_error(err.errors());
        return format::render().view(
            &v,
            "auth/register/register_partial.html",
            data!({
                "user_email": params.email,
                "user_name": params.name,
                "website": website,
                "error_msg": error_msg
            }),
        );
    }

    let user = match users::Model::create_with_password(&ctx.db, &params, &stripe_client).await {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            let error_msg = AuthError::default();

            match err {
                ModelError::EntityAlreadyExists { .. } => {
                    let error_msg=  error_msg.register_email(
                        "This email address is already associated with an account. Please use a different email or log in to your existing account.")
                    ;
                    return format::render().view(
                        &v,
                        "auth/register/register_partial.html",
                        data!(
                            {
                                "user_email": params.email, "user_name": params.name,
                                "website": website, "error_msg": error_msg
                            }
                        ),
                    );
                }
                _ => {
                    let error_msg =
                        error_msg.register_msg("Something went wrong. Please try again.");
                    return format::render().view(
                        &v,
                        "auth/register/register_partial.html",
                        data!(
                            {
                                "user_email": params.email, "user_name": params.name,
                                "website": website, "error_msg": error_msg
                            }
                        ),
                    );
                }
            }
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user, &website.website_basic_info).await?;

    // Ok(HxRedirect(routes::Auth::LOGIN_PARTIAL.to_string()).into_response())

    format::render().view(
        &v,
        "auth/login/login_partial.html",
        data!({"website": website}),
    )
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
/// use chrono::{Duration, Local};

#[debug_handler]
async fn verify(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    Path(token): Path<String>,
) -> Result<Response> {
    use chrono::Duration;

    let user = users::Model::find_by_verification_token(&ctx.db, &token).await?;

    if user.email_verified_at.is_some() {
        return format::render().view(
            &v,
            "auth/verify/email_verified.html",
            data!({"website": website, "msg": "Email already verified"}),
        );
    };

    if let Some(sent_at) = user.email_verification_sent_at {
        if Utc::now().naive_utc() > sent_at.naive_utc() + Duration::hours(1) {
            return format::render().view(
                &v,
                "auth/verify/email_verification_expired.html",
                data!({
                    "website": website,
                    "email": user.email,
                }),
            );
        }
    }

    let active_model = user.into_active_model();
    let _user = active_model.verified(&ctx.db).await?;

    format::render().view(
        &v,
        "auth/verify/email_verified.html",
        data!({"website": website}),
    )
}

#[debug_handler]
async fn resent_verification_token(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    Json(email_params): Json<MagicLinkParams>,
) -> Result<Response> {
    let user = UserModel::find_by_email(&ctx.db, &email_params.email).await?;

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_verification_link(&ctx, &user, &website.website_basic_info).await?;

    format::render().view(
        &v,
        "auth/verify/email_verification_send.html",
        data!({"website": website, "email": user.email}),
    )
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::render().view(
            &v,
            "auth/forgot/partials/forgot_msg.html",
            data!({"message": true}),
        );
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user, &website.website_basic_info).await?;

    format::render().view(
        &v,
        "auth/forgot/partials/forgot_msg.html",
        data!({"message": true}),
    )
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

#[debug_handler]
async fn api_login(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    Json(params): Json<LoginParams>,
) -> Result<Response> {
    let user = match users::Model::find_by_email(&ctx.db, &params.email).await {
        Ok(user) => user,
        Err(err) => {
            let user_email = &params.email;
            tracing::info!(message = err.to_string(), user_email, "could not find user",);
            let error_msg = AuthError::default().login_error();
            return format::render().view(
                &v,
                "auth/login/login_partial.html",
                data!(
                    {
                        "user_email": user_email, "website": website,
                        "error_msg": error_msg
                    }
                ),
            );
        }
    };

    if user.email_verified_at.is_none() {
        return format::render().view(
            &v,
            "auth/login/login_partial.html",
            data!(
                {
                    "user_email": user.email, "website": website,
                    "error_msg": AuthError::default().verify_error()
                }
            ),
        );
    }

    if !user.verify_password(&params.password) {
        return format::render().view(
            &v,
            "auth/login/login_partial.html",
            data!(
                {
                    "user_email": user.email, "website": website,
                    "error_msg": AuthError::default().login_error()

                }
            ),
        );
    }

    let jwt_secret = ctx.config.get_jwt_config()?;
    let expire = match params.remember {
        true => 7 * 24 * 60 * 60,
        false => jwt_secret.expiration,
    };
    let token = user
        .generate_jwt(&jwt_secret.secret, expire)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let cookie = AxumCookie::build(("auth", token.clone()))
        .path("/")
        .http_only(true)
        .secure(!cfg!(debug_assertions)) // set to false in localhost for dev
        .same_site(SameSite::Strict)
        .max_age(time::Duration::seconds(expire as i64))
        .build();

    let cookie_value = HeaderValue::from_str(&cookie.to_string())
        .map_err(|_| loco_rs::Error::Unauthorized("failed to build cookie header".to_string()))?;

    let user_pid = UserPid::new(&user.pid.to_string());
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;

    let mut view_response = format::render().view(
        &v,
        "dashboard/dashboard_base_extend_partial.html",
        data!({"website": website, "user": user, "credits": user_credits, "models": training_models}),
    )?;

    view_response
        .headers_mut()
        .insert("Set-Cookie", cookie_value);

    Ok(view_response)
}

#[debug_handler]
async fn logout(State(_ctx): State<AppContext>) -> Result<Response> {
    // Set the expiration date in the past to remove the cookie
    let expiry = Utc::now() - Duration::days(1);
    let expired_cookie = format!(
        "auth=; Path=/; HttpOnly; Secure; SameSite=Strict; Expires={}",
        expiry.to_rfc2822()
    );

    // Create headers for removing the cookie and redirecting
    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", expired_cookie.parse().unwrap());
    headers.insert("HX-Redirect", "/login".parse().unwrap()); // HTMX redirect

    Ok((StatusCode::OK, headers).into_response())
}

#[debug_handler]
async fn logout_partial(
    State(_ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let cookie = AxumCookie::build(("auth", ""))
        .path("/")
        .http_only(true)
        .secure(!cfg!(debug_assertions)) // false in dev
        .same_site(SameSite::Strict)
        .max_age(time::Duration::seconds(0))
        .build();

    let cookie_header_value = cookie.to_string();

    let cookie_header = HeaderValue::from_str(&cookie_header_value)
        .or_else(|_| {
            let expiry = Utc::now() - Duration::days(1);
            let fallback = format!(
                "auth=; Path=/; HttpOnly; Secure; SameSite=Strict; Expires={}",
                expiry.to_rfc2822()
            );
            HeaderValue::from_str(&fallback)
        })
        .map_err(|_| {
            loco_rs::Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("CookieFailed", "failed to build cookie header"),
            )
        })?;

    let mut view_response = format::render().view(
        &v,
        "auth/login/login_partial.html",
        data!({"website": website}),
    )?;

    view_response
        .headers_mut()
        .insert("Set-Cookie", cookie_header);

    Ok(view_response)
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

/// Magic link authentication provides a secure and passwordless way to log in to the application.
///
/// # Flow
/// 1. **Request a Magic Link**:  
///    A registered user sends a POST request to `/magic-link` with their email.  
///    If the email exists, a short-lived, one-time-use token is generated and sent to the user's email.  
///    For security and to avoid exposing whether an email exists, the response always returns 200, even if the email is invalid.
///
/// 2. **Click the Magic Link**:  
///    The user clicks the link (/magic-link/{token}), which validates the token and its expiration.  
///    If valid, the server generates a JWT and responds with a [`LoginResponse`].  
///    If invalid or expired, an unauthorized response is returned.
///
/// This flow enhances security by avoiding traditional passwords and providing a seamless login experience.
async fn magic_link(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Json(params): Json<MagicLinkParams>,
) -> Result<Response> {
    let email_regex = get_allow_email_domain_re();
    if !email_regex.is_match(&params.email) {
        tracing::debug!(
            email = params.email,
            "The provided email is invalid or does not match the allowed domains"
        );
        return bad_request("invalid request");
    }

    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::debug!(email = params.email, "user not found by email");
        return format::empty_json();
    };

    let user = user.into_active_model().create_magic_link(&ctx.db).await?;

    AuthMailer::send_magic_link(&ctx, &user, &website.website_basic_info).await?;

    format::empty_json()
}

/// Verifies a magic link token and authenticates the user.
async fn magic_link_verify(
    Path(token): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return unauthorized("unauthorized!");
    };

    let user = user.into_active_model().clear_magic_link(&ctx.db).await?;

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

#[debug_handler]
pub async fn get_login(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let view_response = if auth.is_ok() {
        let user_pid = UserPid::new(&auth.unwrap().claims.pid);
        let (user, user_credits, training_models) =
            load_user_credit_training(&ctx.db, &user_pid).await?;
        format::render().view(
            &v,
            "dashboard/dashboard_base_extend.html",
            data!(
                  {
                      "website": website, "user": user,
                      "models": training_models, "credits": user_credits,
                      "is_initial_load": true, "cc_cookie": cc_cookie,
                      "is_logged_in": true
                  }
            ),
        )?
    } else {
        format::render().view(
            &v,
            "auth/login/login_form.html",
            data!({"website": website}),
        )?
    };

    let final_response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(view_response.into_response().into_body()))
        .map_err(|e| {
            Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("RESPONSE_BUILD_FAILED", &e.to_string()),
            )
        })?;

    Ok(final_response)
}

#[debug_handler]
pub async fn partial_login(
    auth: Result<auth::JWT>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let view_response = if auth.is_ok() {
        let user_pid = UserPid::new(&auth.unwrap().claims.pid);
        let (user, user_credits, training_models) =
            load_user_credit_training(&ctx.db, &user_pid).await?;
        format::render().view(
            &v,
            "dashboard/dashboard_base_extend_partial.html",
            data!(
                {
                    "website": website, "user": user, "credits": user_credits,
                    "models": training_models, "is_logged_in": true
                }
            ),
        )?
    } else {
        format::render().view(
            &v,
            "auth/login/login_partial.html",
            data!({"website": website}),
        )?
    };

    Ok(view_response)
}

#[debug_handler]
pub async fn get_register(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/register/register_form.html",
        data!({"website": website}),
    )
}

#[debug_handler]
pub async fn partial_register(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/register/register_partial.html",
        data!({"website": website}),
    )
}

#[debug_handler]
pub async fn get_forgot(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/forgot/forgot_form.html",
        data!({"website": website}),
    )
}

#[debug_handler]
pub async fn partial_forgot(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/forgot/forgot_partial.html",
        data!({"website": website}),
    )
}
