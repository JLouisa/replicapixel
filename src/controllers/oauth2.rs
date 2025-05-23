// use crate::models::join::user_credits_models::load_user_and_credits;
use crate::models::users::RegisterParams;
use crate::models::UserModel;
use crate::service::stripe::stripe::StripeClient;
use crate::views;
use axum::body::Body;
use loco_oauth2::controllers::middleware::OAuth2CookieUser;
use loco_rs::controller::ErrorDetail;
use std::fmt::Debug;

use loco_oauth2::models::oauth2_sessions::OAuth2SessionsTrait;
use loco_oauth2::models::users::OAuth2UserTrait;
use loco_rs::prelude::*;

use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use axum::Extension;
use axum::{debug_handler, http::StatusCode, Json};
use axum_extra::extract::cookie::{Cookie as AxumCookie, SameSite};
use axum_session::{DatabasePool, Session, SessionNullPool};

use serde::de::DeserializeOwned;
use serde::Deserialize;

use super::dashboard::routes::Dashboard;
use crate::domain::website::Website;
use crate::models::{o_auth2_sessions, users, users::OAuth2UserProfile};
use google_oauth::AsyncClient;
use loco_oauth2::controllers::oauth2::{
    callback_jwt as callback_jwt_google, get_authorization_url, google_authorization_url,
    AuthParams,
};
use loco_oauth2::OAuth2ClientStore;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct OAuth2Routes {
        pub base: String,
        pub google: String,
        pub google_ott: String,
        pub github: String,
    }
    impl OAuth2Routes {
        pub fn init() -> Self {
            Self {
                base: String::from(OAuth2::BASE),
                google: format!("{}{}", OAuth2::BASE, OAuth2::GOOGLE),
                google_ott: format!("{}{}", OAuth2::BASE, OAuth2::GOOGLE_OTT),
                github: format!("{}{}", OAuth2::BASE, OAuth2::GITHUB),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct OAuth2;
    impl OAuth2 {
        pub const BASE: &'static str = "/api/oauth2";
        pub const PROTECTED: &'static str = "/protected";
        pub const GOOGLE: &'static str = "/google";
        pub const GOOGLE_CALLBACK_JWT: &'static str = "/google/callback/jwt";
        pub const GOOGLE_OTT: &'static str = "/google-ott";
        pub const GITHUB: &'static str = "/github";
        pub const GITHUB_CALLBACK_JWT: &'static str = "/github/callback/jwt";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::OAuth2::BASE)
        .add(routes::OAuth2::PROTECTED, get(protected))
        .add(
            routes::OAuth2::GOOGLE,
            get(google_authorization_url::<SessionNullPool>),
        )
        .add(
            routes::OAuth2::GOOGLE_CALLBACK_JWT,
            get(google_callback_jwt::<
                OAuth2UserProfile,
                users::Model,
                o_auth2_sessions::Model,
                SessionNullPool,
            >),
        )
        .add(routes::OAuth2::GOOGLE_OTT, post(google_ott))
        .add(
            routes::OAuth2::GITHUB,
            get(github_authorization_url::<SessionNullPool>),
        )
    // .add(
    //     routes::OAuth2::GITHUB_CALLBACK_JWT,
    //     get(github_callback_jwt::<
    //         OAuth2UserProfile,
    //         users::Model,
    //         o_auth2_sessions::Model,
    //         SessionNullPool,
    //     >),
    // )
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenPayload {
    token: String,
}

#[debug_handler]
async fn google_ott(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(stripe_client): Extension<StripeClient>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(token_payload_req): Json<GoogleTokenPayload>,
) -> Result<Response> {
    let google_client_id = website
        .website_basic_info
        .google
        .google_client_id
        .to_owned();
    let client = AsyncClient::new(google_client_id);
    let google_user_data = match client.validate_id_token(token_payload_req.token).await {
        Ok(payload) => payload,
        Err(e) => {
            dbg!(&e);
            return Err(loco_rs::Error::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("Token Error", &e.to_string()),
            ));
        }
    };

    // Optional: Check if email is verified by Google
    if !google_user_data.email_verified.unwrap_or(false) {
        tracing::warn!("Google email not verified");
    };

    let register = RegisterParams::create_with_ott(google_user_data)?;
    dbg!(&register);
    // format::empty()

    let user = UserModel::upsert_with_ott(&ctx.db, &register, &stripe_client).await?;

    let jwt_secret = ctx.config.get_jwt_config()?;
    let expire = jwt_secret.expiration * 7; // 7 days
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
    let cookie_header_value = cookie.to_string();

    // let (loaded_user, user_credits) =
    //     load_user_and_credits(&ctx.db, &UserPid::new(user.pid)).await?;

    let view_response = views::home::google_ott(&v, &website, &user.into())?;

    // Build the final response
    let mut response_builder = Response::builder().status(StatusCode::OK);
    response_builder = response_builder.header("Set-Cookie", cookie_header_value);

    // Add the view response body - Using your original error
    let final_response = response_builder
        .body(Body::from(view_response.into_response().into_body()))
        .map_err(|e| {
            Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("RESPONSE_BUILD_FAILED", &e.to_string()),
            )
        })?;

    Ok(final_response)

    // format::json(register)
}

#[debug_handler]
async fn protected(
    State(ctx): State<AppContext>,
    user: OAuth2CookieUser<OAuth2UserProfile, users::Model, o_auth2_sessions::Model>,
) -> Result<Response> {
    let user: &users::Model = user.as_ref();
    let jwt_secret = ctx.config.get_jwt_config()?;
    // Generate a JWT token
    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let cookie = AxumCookie::build(("auth", token))
        .path("/")
        .http_only(true)
        .secure(!cfg!(debug_assertions)) // set to false in localhost
        .same_site(SameSite::Strict)
        .max_age(time::Duration::seconds(jwt_secret.expiration as i64))
        .build();

    let mut response = Redirect::to(Dashboard::BASE).into_response();
    response
        .headers_mut()
        .append("Set-Cookie", cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn google_callback_jwt<
    T: DeserializeOwned + Send,
    U: OAuth2UserTrait<T> + ModelTrait,
    V: OAuth2SessionsTrait<U>,
    W: DatabasePool + Clone + Debug + Sync + Send + 'static,
>(
    State(ctx): State<AppContext>,
    session: Session<W>,
    Query(params): Query<AuthParams>,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<impl IntoResponse> {
    let mut client = oauth2_store
        .get_authorization_code_client("google")
        .await
        .map_err(|e| {
            tracing::error!("Error getting client: {:?}", e);
            Error::InternalServerError
        })?;
    let jwt_secret = ctx.config.get_jwt_config()?;
    let user = callback_jwt_google::<T, U, V, W>(&ctx, session, params, &mut client).await?;
    drop(client);

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let cookie = AxumCookie::build(("auth", token))
        .path("/")
        .http_only(true)
        .secure(!cfg!(debug_assertions)) // set to false in localhost
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(jwt_secret.expiration as i64))
        .build();

    let mut response = Redirect::to(Dashboard::BASE).into_response();
    response
        .headers_mut()
        .append("Set-Cookie", cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn github_authorization_url<T: DatabasePool + Clone + Debug + Sync + Send + 'static>(
    session: Session<T>,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<String> {
    let mut client = oauth2_store
        .get_authorization_code_client("github")
        .await
        .map_err(|e| {
            tracing::error!("Error getting github client: {:?}", e);
            Error::InternalServerError
        })?;
    let auth_url = get_authorization_url(session, &mut client).await;
    dbg!(&auth_url);
    drop(client);
    Ok(auth_url)
}

// #[derive(Debug, Deserialize)]
// pub struct AuthParams2 {
//     pub code: String,
//     pub state: String,
// }

// pub async fn github_callback_jwt<
//     T: DeserializeOwned + Send,
//     U: OAuth2UserTrait<T> + ModelTrait,
//     V: OAuth2SessionsTrait<U>,
//     W: DatabasePool + Clone + Debug + Sync + Send + 'static,
// >(
//     State(ctx): State<AppContext>,
//     session: Session<SessionNullPool>,
//     Query(params): Query<AuthParams2>,
//     Extension(oauth2_store): Extension<OAuth2ClientStore>,
// ) -> Result<impl IntoResponse> {
//     let mut client = oauth2_store
//         .get_authorization_code_client("github")
//         .await
//         .map_err(|e| {
//             tracing::info!("Error getting github client: {:?}", e);
//             Error::Message(e.to_string())
//         })?;

//     let jwt_secret = ctx.config.get_jwt_config()?;

//     let user = callback_jwt_github::<T, U, V, W>(&ctx, session, params, &mut client).await?;

//     drop(client);

//     let token = user
//         .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
//         .or_else(|_| unauthorized("unauthorized!"))?;

//     let cookie = AxumCookie::build(("auth", token))
//         .path("/")
//         .http_only(true)
//         .secure(!cfg!(debug_assertions)) // set to false in localhost
//         .same_site(SameSite::Strict)
//         .max_age(time::Duration::seconds(jwt_secret.expiration as i64))
//         .build();

//     let mut response = Redirect::to(Dashboard::BASE).into_response();
//     response
//         .headers_mut()
//         .append("Set-Cookie", cookie.to_string().parse().unwrap());

//     Ok(response)
// }

// pub async fn callback_jwt_github<
//     T: DeserializeOwned + Send,
//     U: OAuth2UserTrait<T> + ModelTrait,
//     V: OAuth2SessionsTrait<U>,
//     W: DatabasePool + Clone + Debug + Sync + Send + 'static,
// >(
//     ctx: &AppContext,
//     session: Session<SessionNullPool>,
//     params: AuthParams2,
//     client: &mut MutexGuard<'_, dyn GrantTrait>,
// ) -> Result<U> {
//     // Get the CSRF token from the session
//     let csrf_token = session
//         .get::<String>("CSRF_TOKEN")
//         .ok_or_else(|| Error::BadRequest("CSRF token not found".to_string()))?;
//     // Exchange the code with a token
//     let (token, profile) = client
//         .verify_code_from_callback(params.code, params.state, csrf_token)
//         .await
//         .map_err(|e| Error::BadRequest(e.to_string()))?;
//     // Get the user profile
//     let profile = profile.json::<T>().await.map_err(|e| {
//         tracing::error!("Error getting profile: {:?}", e);
//         Error::Message(e.to_string())
//     })?;
//     let user = U::upsert_with_oauth(&ctx.db, &profile)
//         .await
//         .map_err(|_err| {
//             tracing::error!("Error creating user");
//             Error::Message(_err.to_string())
//         })?;
//     V::upsert_with_oauth2(&ctx.db, &token, &user)
//         .await
//         .map_err(|_k| {
//             tracing::error!("Error creating session");
//             Error::Message(_k.to_string())
//         })?;

//     Ok(user)
// }

// use std::{collections::HashMap, time::Instant};

// use async_trait::async_trait;
// use oauth2::basic::{
//     BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
// };
// use oauth2::{
//     basic::{BasicClient, BasicTokenResponse},
//     url,
//     url::Url,
//     AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
//     PkceCodeVerifier, RedirectUrl, Scope, StandardRevocableToken, TokenResponse, TokenUrl,
// };
// use reqwest::{header, Response};
// use serde::Serialize;

// fn constant_time_compare(a: &str, b: &str) -> bool {
//     // Convert the strings to bytes for comparison.
//     a.as_bytes().ct_eq(b.as_bytes()).into()
// }

// async fn verify_code_from_callback(
//     foo: &mut MutexGuard<'_, dyn GrantTrait>,
//     code: String,
//     state: String,
//     csrf_token: String,
// ) -> OAuth2ClientResult<(BasicTokenResponse, Response)> {
//     let client = foo.get_authorization_code_client();
//     // Clear outdated flow states
//     client.remove_expire_flow();
//     // Compare csrf token, use subtle to prevent time attack
//     if constant_time_compare(&csrf_token, &state) {
//         return Err(OAuth2ClientError::CsrfTokenError);
//     }
//     // Get the pkce_verifier for exchanging code
//     let (pkce_verifier, _) = match client.flow_states.remove(&csrf_token) {
//         None => {
//             return Err(OAuth2ClientError::CsrfTokenError);
//         }
//         Some(item) => item,
//     };
//     // Exchange the code with a token
//     let token = client
//         .oauth2
//         .exchange_code(AuthorizationCode::new(code))?
//         .set_pkce_verifier(pkce_verifier)
//         .request_async(&oauth2::reqwest::Client::new())
//         .await?;
//     let profile = client
//         .http_client
//         .get(client.profile_url.clone())
//         .bearer_auth(token.access_token().secret().to_owned())
//         .header(header::CONTENT_TYPE, "application/json")
//         .send()
//         .await
//         .map_err(OAuth2ClientError::ProfileError)?;
//     Ok((token, profile))
// }
