#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    controllers::dashboard::CurrentPage,
    domain::{
        cookie::{AppCookie, CookieTrait, UserCookieTrait},
        mailer_options::MailerOptions,
        website::{Website, WebsiteOptions},
    },
    mailers::{auth::AuthMailer, transaction::CheckoutMailer},
    middleware::{cookie::ExtractConsentState, i18nv2::LangEngine},
    models::{
        _entities::{
            sea_orm_active_enums::{Account, Language},
            users,
        },
        join::user_credits_models::{load_user_and_settings, load_user_credit_training},
        users::{LoginParams, PasswordChangeParams, RegisterParams, UserPid},
        PlanModel, TransactionModel, UserModel, UserSettingsModel,
    },
    service::{
        redis::redis::{RedisCacheDriver, RedisKey},
        stripe::stripe::StripeClient,
    },
    views::{self, auth::CurrentResponse, payment::PricingView},
};
use axum::{
    debug_handler,
    extract::{Json, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Extension,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use derive_more::Constructor;
use loco_rs::{controller::ErrorDetail, prelude::*};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, sync::OnceLock};
use validator::ValidationErrorsKind;

use crate::controllers::auth::routes as AuthRoutes;
use crate::controllers::payment::routes::Payment as PaymentRoutes;

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
        pub api_password_reset: String,
        pub api_auth_register_stripe: String,
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
                api_password_reset: String::from(Auth::API_MAGIC_LINK),
                api_auth_register_stripe: String::from(Auth::API_AUTH_REGISTER_STRIPE),
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
        pub const MAGIC_LINK: &'static str = "/auth/magic";
        pub const API_MAGIC_LINK: &'static str = "/api/auth/magic";
        pub const API_MAGIC_LINK_TOKEN: &'static str = "/api/auth/magic/{token}";
        pub const API_PASSWORD_CHANGE_ID: &'static str = "/api/auth/password-change/{id}";
        pub const API_PASSWORD_CHANGE: &'static str = "/api/auth/password-change";
        pub const API_CHECK_USER: &'static str = "/api/auth/check-user";
        pub const API_SET_LANGUAGE: &'static str = "/api/auth/language";
        pub const API_AUTH_REGISTER_STRIPE: &'static str = "/api/auth/register/prepare";
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
        .add(routes::Auth::API_FORGOT, post(api_forgot))
        .add(routes::Auth::API_RESET, post(reset))
        .add(routes::Auth::API_CURRENT, get(current))
        .add(routes::Auth::API_MAGIC_LINK_TOKEN, get(get_password))
        .add(routes::Auth::API_MAGIC_LINK_TOKEN, post(set_password))
        .add(routes::Auth::API_PASSWORD_CHANGE_ID, post(change_password))
        .add(routes::Auth::API_SET_LANGUAGE, post(set_language))
        .add(
            routes::Auth::API_AUTH_REGISTER_STRIPE,
            post(auth_stripe_register_handler),
        )
    // // .add(routes::Auth::API_CHECK_USER, get(check_user))
    //// .add("/api/auth/test/welcome", get(test_welcome_mail))
    //// .add("/api/auth/test/forgot_password", get(test_forgot_password))
    //// .add("/api/auth/test/magic_link", get(test_magic_link))
    //// .add("/api/auth/test/transaction", get(test_transaction))
}

pub struct HxRedirect(String);
impl HxRedirect {
    pub fn new(url: &str) -> Self {
        Self(String::from(url))
    }
    pub fn login() -> Self {
        Self(String::from(routes::Auth::LOGIN))
    }
    pub fn payment(user: &UserModel, plan: &PricingView) -> Self {
        let link = format!(
            "{}{}/{}/{}",
            PaymentRoutes::BASE,
            PaymentRoutes::API_STRIPE_PREPARE,
            user.pid,
            plan.plan_name.to_string()
        );
        Self(String::from(link))
    }
}
impl IntoResponse for HxRedirect {
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Redirect", self.0.parse().unwrap());
        (headers, StatusCode::OK).into_response()
    }
}

#[derive(Debug, Deserialize, Constructor, Serialize, Default)]
pub struct AuthError {
    general: Option<String>,
    login: Option<String>,
    register: Option<String>,
    register_name: Option<String>,
    register_email: Option<String>,
    register_password: Option<String>,
    verify: Option<String>,
    forgot: Option<String>,
    logout: Option<String>,
    password_reset: Option<String>,
}
impl AuthError {
    pub fn general_msg(&self, msg: &str) -> Self {
        Self {
            general: Some(String::from(msg)),
            ..Default::default()
        }
    }
    pub fn password_reset_error(&self) -> Self {
        Self {
            password_reset: Some(String::from("Password and confirm password do not match")),
            ..Default::default()
        }
    }
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
            general: Some(String::from("Email is not verified. Check your inbox")),
            ..Default::default()
        }
    }
}

async fn load_plan(db: &impl ConnectionTrait, name: &String) -> Result<PlanModel> {
    let item = PlanModel::find_by_name_string(db, &name).await?;
    Ok(item)
}
async fn load_plan_pid(db: &DatabaseConnection, pid: &Uuid) -> Result<PlanModel> {
    let item = PlanModel::find_by_pid(db, &pid).await?;
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

async fn get_user_settings(
    db: &DatabaseConnection,
    cache: &RedisCacheDriver,
    user_pid: &UserPid,
) -> Result<UserSettingsModel> {
    let cache_key = RedisKey::UserSetting(user_pid.clone());
    // Try cache first
    if let Ok(Some(cached)) = cache.get::<UserSettingsModel>(&cache_key).await {
        return Ok(cached);
    }
    // Cache miss → load from DB
    let (_, settings) = load_user_and_settings(db, user_pid).await?;

    let time = Some(60 * 60 * 24 * 30); // 30 days

    // Update cache
    let _ = cache.set(&cache_key, &settings, time).await;
    Ok(settings)
}

async fn set_user_settings(
    db: &DatabaseConnection,
    cache: &RedisCacheDriver,
    user_pid: &UserPid,
    user_settings: &UserSettingsModel,
    lang: &Language,
) -> Result<()> {
    let settings = user_settings.set_language_preference(db, lang).await?;
    let cache_key = RedisKey::UserSetting(user_pid.clone());
    let time = Some(60 * 60 * 24 * 7); // 7 days
    let _ = cache.set(&cache_key, &settings, time).await;
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct SetLanguagePayload {
    pub lang: Language,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ForgotParams {
    #[validate(email(message = "Email is invalid"))]
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PasswordMagicParams {
    pub email: String,
    #[validate(must_match(other = "confirm_password", message = "Passwords do not match"))]
    pub password: String,
    pub confirm_password: String,
}

#[debug_handler]
pub async fn test_transaction(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("0aec7a76-c58d-40ba-af47-5c876e310899");
    let transaction_pid: Uuid = "5cab4986-e126-4747-be50-fd4a210eb535".parse().unwrap();
    let plan_name_str = "Basic".to_string();
    let user = load_user(&ctx.db, &user_pid).await?;
    let plan = load_plan(&ctx.db, &plan_name_str).await?;
    let transaction = load_transaction(&ctx.db, &transaction_pid).await?;

    let mailer_options = MailerOptions::new()
        .website(&website)
        .user(&user.into())
        .transaction(&transaction.into())
        .plan(&plan.into());

    CheckoutMailer::send_checkout_completed(&ctx, &mailer_options).await?;
    Ok((StatusCode::OK).into_response())
}
#[debug_handler]
pub async fn test_welcome_mail(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("0aec7a76-c58d-40ba-af47-5c876e310899");
    let user = load_user(&ctx.db, &user_pid).await?;
    let mailer_options = MailerOptions::new().website(&website).into_user(&user);
    AuthMailer::send_welcome(&ctx, &mailer_options).await?;
    Ok((StatusCode::OK).into_response())
}
#[debug_handler]
pub async fn test_forgot_password(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("0aec7a76-c58d-40ba-af47-5c876e310899");
    let user = load_user(&ctx.db, &user_pid).await?;
    let mailer_options = MailerOptions::new().website(&website).into_user(&user);
    AuthMailer::forgot_password(&ctx, &mailer_options).await?;
    Ok((StatusCode::OK).into_response())
}
#[debug_handler]
pub async fn test_magic_link(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new("0aec7a76-c58d-40ba-af47-5c876e310899");
    let user = load_user(&ctx.db, &user_pid).await?;
    let mailer_options = MailerOptions::new().website(&website).into_user(&user);
    AuthMailer::send_magic_link(&ctx, &mailer_options).await?;
    Ok((StatusCode::OK).into_response())
}

#[debug_handler]
async fn set_language(
    auth: Result<auth::JWT>, // Use the lightweight extractor
    jar: CookieJar,
    State(ctx): State<AppContext>,
    Extension(cache): Extension<RedisCacheDriver>,
    Json(payload): Json<SetLanguagePayload>,
) -> Result<Response> {
    // If user is logged in, update their DB record for future logins.
    let lang_code = match auth {
        Ok(auth) => {
            let user_pid = UserPid::new(&auth.claims.pid);
            let (_, settings) = load_user_and_settings(&ctx.db, &user_pid).await?;
            let _ = set_user_settings(&ctx.db, &cache, &user_pid, &settings, &payload.lang).await?;
            settings.language
        }
        Err(_) => payload.lang,
    };

    // Always update the cookie for the current session.
    let cookie = AppCookie::create_language_cookie(lang_code);

    Ok((jar.add(cookie), StatusCode::NO_CONTENT).into_response())
}

#[debug_handler]
pub async fn auth_stripe_register_handler(
    Extension(stripe_client): Extension<StripeClient>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
    Json(params): Json<RegisterParams>,
) -> Result<impl IntoResponse> {
    let plan: PricingView = match params.plan_id {
        Some(pid) => load_plan_pid(&ctx.db, &pid).await?.into(),
        None => {
            return Ok((StatusCode::BAD_REQUEST).into_response());
        }
    };

    if let Err(err) = params.validate() {
        let error_msg = AuthError::default().register_error(err.errors());
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .register(&params)
            .auth_error(&error_msg);
        return format::render().view(
            &v,
            "home/sections/partials/pricing_auth_signup_partial.html",
            data!({"options": website_options, "pricing": plan}),
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
                    let website_options = WebsiteOptions::new()
                        .website(&website)
                        .language(&lang)
                        .register(&params)
                        .auth_error(&error_msg);
                    return format::render().view(
                        &v,
                        "home/sections/partials/pricing_auth_signup_partial.html",
                        data!({"options": website_options, "pricing": plan}),
                    );
                }
                _ => {
                    let error_msg =
                        error_msg.register_msg("Something went wrong. Please try again.");
                    let website_options = WebsiteOptions::new()
                        .website(&website)
                        .language(&lang)
                        .register(&params)
                        .auth_error(&error_msg);
                    return format::render().view(
                        &v,
                        "home/sections/partials/pricing_auth_signup_partial.html",
                        data!({"options": website_options, "pricing": plan}),
                    );
                }
            }
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    let mailer_options = MailerOptions::new().website(&website).into_user(&user);
    AuthMailer::send_welcome(&ctx, &mailer_options).await?;

    let redirect = HxRedirect::payment(&user, &plan);

    Ok(redirect.into_response())
}

#[debug_handler]
pub async fn change_password(
    auth: auth::JWT,
    Path(pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
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
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .user(user.into())
            .message("There was an error with your password");
        return views::settings::password_change(v, &website_options);
    }

    let user = user
        .into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into());

    views::settings::password_change(v, &website_options)
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    Extension(stripe_client): Extension<StripeClient>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    if let Err(err) = params.validate() {
        let error_msg = AuthError::default().register_error(err.errors());
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .register(&params)
            .auth_error(&error_msg);
        return format::render().view(
            &v,
            "auth/register/register_partial.html",
            data!({"options": website_options}),
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
                    let website_options = WebsiteOptions::new()
                        .website(&website)
                        .language(&lang)
                        .register(&params)
                        .auth_error(&error_msg);
                    return format::render().view(
                        &v,
                        "auth/register/register_partial.html",
                        data!({"options": website_options}),
                    );
                }
                _ => {
                    let error_msg =
                        error_msg.register_msg("Something went wrong. Please try again.");
                    let website_options = WebsiteOptions::new()
                        .website(&website)
                        .language(&lang)
                        .register(&params)
                        .auth_error(&error_msg);
                    return format::render().view(
                        &v,
                        "auth/register/register_partial.html",
                        data!({"options": website_options}),
                    );
                }
            }
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    let mailer_options = MailerOptions::new().website(&website).into_user(&user);
    AuthMailer::send_welcome(&ctx, &mailer_options).await?;

    // Ok(HxRedirect(routes::Auth::LOGIN_PARTIAL.to_string()).into_response())

    let website_options = WebsiteOptions::new().website(&website).language(&lang);
    format::render().view(
        &v,
        "auth/login/login_partial.html",
        data!({"options": website_options}),
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
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    use chrono::Duration;

    let user = users::Model::find_by_verification_token(&ctx.db, &token).await?;

    if user.email_verified_at.is_some() {
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .message("Email already verified");
        return format::render().view(
            &v,
            "auth/verify/email_verified.html",
            data!({"options": website_options}),
        );
    };

    if let Some(sent_at) = user.email_verification_sent_at {
        if Utc::now().naive_utc() > sent_at.naive_utc() + Duration::hours(1) {
            let website_options = WebsiteOptions::new()
                .website(&website)
                .language(&lang)
                .message("Email already verified");
            return format::render().view(
                &v,
                "auth/verify/email_verification_expired.html",
                data!({
                    "options": website_options,
                    "email": user.email,
                }),
            );
        }
    }

    let active_model = user.into_active_model();
    let _user = active_model.verified(&ctx.db).await?;
    let website_options = WebsiteOptions::new().website(&website).language(&lang);
    format::render().view(
        &v,
        "auth/verify/email_verified.html",
        data!({"options": website_options}),
    )
}

#[debug_handler]
async fn resent_verification_token(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
    Json(email_params): Json<MagicLinkParams>,
) -> Result<Response> {
    let user = UserModel::find_by_email(&ctx.db, &email_params.email).await?;

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    let mailer_options = MailerOptions::new().website(&website).into_user(&user);
    AuthMailer::send_verification_link(&ctx, &mailer_options).await?;

    let website_options = WebsiteOptions::new().website(&website).language(&lang);
    format::render().view(
        &v,
        "auth/verify/email_verification_send.html",
        data!({"options": website_options}),
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
    Extension(cache): Extension<RedisCacheDriver>,
    cookie_jar: CookieJar,
    LangEngine(lang): LangEngine,
    Json(params): Json<LoginParams>,
) -> Result<impl IntoResponse> {
    let user = match users::Model::find_by_email(&ctx.db, &params.email).await {
        Ok(user) => user,
        Err(err) => {
            let user_email = &params.email;
            tracing::info!(message = err.to_string(), user_email, "could not find user",);
            let error_msg = AuthError::default().login_error();
            let website_options = WebsiteOptions::new()
                .website(&website)
                .language(&lang)
                .auth_error(&error_msg)
                .login(&params);
            return format::render().view(
                &v,
                "auth/login/login_partial.html",
                data!({"options": website_options}),
            );
        }
    };

    if user.email_verified_at.is_none() {
        let error_msg = AuthError::default().verify_error();
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .auth_error(&error_msg)
            .user(user.into());
        return format::render().view(
            &v,
            "auth/login/login_partial.html",
            data!({"options": website_options}),
        );
    }

    if !user.verify_password(&params.password) {
        let error_msg = AuthError::default().verify_error();
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .auth_error(&error_msg)
            .user(user.into());
        return format::render().view(
            &v,
            "auth/login/login_partial.html",
            data!({"options": website_options}),
        );
    }

    let user_pid = UserPid::new(&user.pid.to_string());
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;
    let settings = get_user_settings(&ctx.db, &cache, &user_pid).await?;

    let cookie = user.create_cookie_strict(&ctx)?;
    let cookie_lang = AppCookie::create_language_cookie(settings.language);

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .language(&settings.language)
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .current_page(CurrentPage::Models);

    let view_response = format::render().view(
        &v,
        "dashboard/dashboard_base_extend_partial.html",
        data!({"options": website_options}),
    )?;

    Ok((cookie_jar.add(cookie).add(cookie_lang), view_response).into_response())
}

#[debug_handler]
async fn logout(State(_ctx): State<AppContext>) -> Result<Response> {
    let cookie = AppCookie::logout_cookie();
    let cookie_str = cookie.to_string();

    // Create headers for removing the cookie and redirecting
    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie_str.parse().unwrap());
    headers.insert("HX-Redirect", AuthRoutes::Auth::LOGIN.parse().unwrap()); // HTMX redirect

    Ok((StatusCode::OK, headers).into_response())
}

#[debug_handler]
async fn logout_partial(
    State(_ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let cookie = AppCookie::logout_cookie();

    let cookie_header_value = cookie.to_string();
    let cookie_header = HeaderValue::from_str(&cookie_header_value).map_err(|_| {
        loco_rs::Error::CustomError(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorDetail::new("CookieFailed", "failed to build cookie header"),
        )
    })?;

    let website_options = WebsiteOptions::new().website(&website).language(&lang);
    let view_response = format::render().view(
        &v,
        "auth/login/login_partial.html",
        data!({"options": website_options}),
    )?;

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie_header);

    Ok((headers, view_response))
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
// async fn magic_link(
//     State(ctx): State<AppContext>,
//     Extension(website): Extension<Website>,
//     Json(params): Json<MagicLinkParams>,
// ) -> Result<Response> {
//     let email_regex = get_allow_email_domain_re();
//     if !email_regex.is_match(&params.email) {
//         tracing::debug!(
//             email = params.email,
//             "The provided email is invalid or does not match the allowed domains"
//         );
//         return bad_request("invalid request");
//     }

//     let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
//         // we don't want to expose our users email. if the email is invalid we still
//         // returning success to the caller
//         tracing::debug!(email = params.email, "user not found by email");
//         return format::empty_json();
//     };

//     let user = user.into_active_model().create_magic_link(&ctx.db).await?;

//     AuthMailer::send_magic_link(&ctx, &user, &website.website_basic_info).await?;

//     format::empty_json()
// }

// /// Verifies a magic link token and authenticates the user.
// async fn magic_link_verify(
//     Path(token): Path<String>,
//     State(ctx): State<AppContext>,
// ) -> Result<Response> {
//     let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token).await else {
//         // we don't want to expose our users email. if the email is invalid we still
//         // returning success to the caller
//         return unauthorized("unauthorized!");
//     };

//     let user = user.into_active_model().clear_magic_link(&ctx.db).await?;

//     let jwt_secret = ctx.config.get_jwt_config()?;

//     let token = user
//         .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
//         .or_else(|_| unauthorized("unauthorized!"))?;

//     format::json(LoginResponse::new(&user, &token))
// }

#[debug_handler]
pub async fn get_login(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let view_response = if let Ok(auth) = auth {
        let user_pid = UserPid::new(&auth.claims.pid);
        let (user, user_credits, training_models) =
            match load_user_credit_training(&ctx.db, &user_pid).await {
                Ok((user, user_credits, training_models)) => (user, user_credits, training_models),
                Err(_) => {
                    let view = format::render().view(
                        &v,
                        "auth/login/login_form.html",
                        data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
                    )?;
                    let cookie = AppCookie::logout_cookie();
                    let cookie_jar = CookieJar::new().add(cookie);
                    return Ok((cookie_jar, view));
                }
            };
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .cc_cookie(&cc_cookie)
            .user(user.into())
            .user_credits(user_credits.into())
            .training_models(training_models.into())
            .current_page(CurrentPage::Album)
            .is_logged_in()
            .is_initial_load();
        format::render().view(
            &v,
            "dashboard/dashboard_base_extend.html",
            data!({"options": website_options}),
        )?
    } else {
        format::render().view(
            &v,
            "auth/login/login_form.html",
            data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
        )?
    };

    Ok((CookieJar::new(), view_response))
}

#[debug_handler]
pub async fn partial_login(
    auth: Result<auth::JWT>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let view_response = if auth.is_ok() {
        let user_pid = UserPid::new(&auth.unwrap().claims.pid);
        let (user, user_credits, training_models) =
            match load_user_credit_training(&ctx.db, &user_pid).await {
                Ok((user, user_credits, training_models)) => (user, user_credits, training_models),
                Err(_) => {
                    let view = format::render().view(
                        &v,
                        "auth/login/login_partial.html",
                        data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
                    )?;
                    let cookie = AppCookie::logout_cookie();
                    let cookie_jar = CookieJar::new().add(cookie);
                    return Ok((cookie_jar, view));
                }
            };
        let website_options = WebsiteOptions::new()
            .website(&website)
            .language(&lang)
            .user(user.into())
            .user_credits(user_credits.into())
            .training_models(training_models.into())
            .is_logged_in();
        format::render().view(
            &v,
            "dashboard/dashboard_base_extend_partial.html",
            data!({"options": website_options}),
        )?
    } else {
        format::render().view(
            &v,
            "auth/login/login_partial.html",
            data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
        )?
    };

    Ok((CookieJar::new(), view_response))
}

#[debug_handler]
pub async fn get_register(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/register/register_form.html",
        data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
    )
}

#[debug_handler]
pub async fn partial_register(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/register/register_partial.html",
        data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
    )
}

#[debug_handler]
pub async fn get_forgot(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/forgot/forgot_form.html",
        data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
    )
}

#[debug_handler]
pub async fn partial_forgot(
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(_ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
) -> Result<Response> {
    format::render().view(
        &v,
        "auth/forgot/forgot_partial.html",
        data!({"options": WebsiteOptions::new().website(&website).language(&lang)}),
    )
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn api_forgot(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    LangEngine(lang): LangEngine,
    Json(params): Json<ForgotParams>,
) -> Result<impl IntoResponse> {
    // let email_regex = get_allow_email_domain_re();
    // if !email_regex.is_match(&params.email) {
    //     tracing::warn!(
    //         email = params.email,
    //         "The provided email is invalid or does not match the allowed domains"
    //     );
    //     return views::auth::forgot(&v);
    // }
    let website_options = WebsiteOptions::new().language(&lang);
    match params.validate() {
        Ok(()) => {}
        Err(_) => {
            return views::auth::forgot(&v, &website_options);
        }
    };

    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return views::auth::forgot(&v, &website_options);
    };

    let is_oauth = user.account != Account::Website;
    if is_oauth {
        return views::auth::forgot(&v, &website_options);
    }

    let user = user.into_active_model().create_magic_link(&ctx.db).await?;

    let mailer_options = MailerOptions::new().website(&website).into_user(&user);
    AuthMailer::forgot_password(&ctx, &mailer_options).await?;

    views::auth::forgot(&v, &website_options)
}

#[debug_handler]
pub async fn get_password(
    Path(token): Path<String>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    LangEngine(lang): LangEngine,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token).await else {
        return Ok(Redirect::to(AuthRoutes::Auth::LOGIN).into_response());
    };

    if let Some(expired_at) = user.magic_link_expiration {
        if Utc::now().naive_utc() > expired_at.naive_utc() {
            user.into_active_model().clear_magic_link(&ctx.db).await?;
            let auth_error = AuthError::default().general_msg("Password reset link expired");
            let website_options = WebsiteOptions::new()
                .website(&website)
                .language(&lang)
                .auth_error(&auth_error);
            return format::render().view(
                &v,
                "auth/verify/password_reset_failed.html",
                data!({"options": website_options}),
            );
        }
    }
    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .message(&token);
    format::render().view(
        &v,
        "auth/verify/password_reset.html",
        data!({"options": website_options, "email": user.email}),
    )
}

#[debug_handler]
pub async fn set_password(
    Path(token): Path<String>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    LangEngine(lang): LangEngine,
    Json(params): Json<PasswordMagicParams>,
) -> Result<Response> {
    let error_msg = match params.validate() {
        Ok(()) => AuthError::default(),
        Err(_) => {
            let error = AuthError::default().password_reset_error();
            let website_options = WebsiteOptions::new()
                .website(&website)
                .language(&lang)
                .auth_error(&error);
            return format::render().view(
                &v,
                "auth/verify/password_reset_partial.html",
                data!({"options": website_options}),
            );
        }
    };

    let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return unauthorized("unauthorized!");
    };

    if user.email != params.email {
        user.into_active_model().clear_magic_link(&ctx.db).await?;
        return format::render().view(
            &v,
            "auth/verify/password_reset_failed.html",
            data!({"website": website, "error": error_msg.general_msg("Unauthorized. Something went wrong!")}),
        );
    }

    let website_options = WebsiteOptions::new().website(&website).language(&lang);

    if user.account != Account::Website {
        user.into_active_model().clear_magic_link(&ctx.db).await?;
        return format::render().view(
            &v,
            "auth/verify/password_reset_success.html",
            data!({"options": website_options}),
        );
    }

    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?
        .into_active_model()
        .clear_magic_link(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "auth/verify/password_reset_success.html",
        data!({"options": website_options}),
    )
}
