use crate::models::join::user_credits_models::load_user_and_credits;
use crate::models::users::UserPid;
use crate::models::{OAuth2SessionModel, UserModel};
use crate::views;
use axum::body::Body;
use loco_oauth2::base_oauth2::basic::BasicTokenResponse;
use loco_oauth2::base_oauth2::{
    AccessToken, AuthorizationCode, EmptyExtraTokenFields, ExtraTokenFields, StandardTokenResponse,
    TokenType,
};
use loco_oauth2::controllers::middleware::OAuth2CookieUser;
use loco_oauth2::error::{OAuth2ClientError, OAuth2ClientResult};
use loco_rs::controller::ErrorDetail;
use std::fmt::Debug;
use tokio::sync::MutexGuard;

use loco_oauth2::grants::authorization_code::GrantTrait;
use loco_oauth2::models::oauth2_sessions::OAuth2SessionsTrait;
use loco_oauth2::models::users::OAuth2UserTrait;
use loco_rs::prelude::*;

use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use axum::Extension;
use axum_extra::extract::cookie::{Cookie as AxumCookie, SameSite};
use axum_session::{DatabasePool, Session, SessionNullPool};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::domain::website::Website;
use crate::models::{o_auth2_sessions, users, users::OAuth2UserProfile};
use loco_oauth2::controllers::oauth2::{
    callback_jwt as callback_jwt_google, get_authorization_url, google_authorization_url,
    AuthParams,
};
use loco_oauth2::OAuth2ClientStore;

use super::dashboard::routes::Dashboard;

use axum::{debug_handler, http::StatusCode, Json};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use base64::Engine;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use std::collections::HashSet;

use google_oauth::AsyncClient;

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
        // .add(routes::OAuth2::GOOGLE_OTT, get(google_ott::<users::Model>))
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

// #[derive(Debug, Deserialize)]
// pub struct GoogleClaims {
//     pub name: Option<String>,
//     pub given_name: Option<String>,
//     pub sub: String,
//     pub email: String,
//     pub email_verified: bool,
//     pub picture: Option<String>,
//     pub iss: String,
//     pub aud: String,
//     pub exp: usize,
//     // ... add other fields if needed
// }

// impl From<GoogleClaims> for OAuth2UserProfile {
//     fn from(claims: GoogleClaims) -> Self {
//         let name = match claims.name {
//             Some(name) => name,
//             None => {
//                 let name = match claims.given_name {
//                     Some(name) => name,
//                     None => "John Doe Google".to_string(),
//                 };
//                 name
//             }
//         };
//         Self {
//             name,
//             email: claims.email,
//             sub: claims.sub,
//             email_verified: claims.email_verified,
//             given_name: None,
//             family_name: None,
//             picture: None,
//             locale: None,
//         }
//     }
// }

async fn google_ott2(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Json(token): Json<GoogleTokenPayload>,
) -> Result<Response> {
    let google_client_id = website
        .website_basic_info
        .google
        .google_client_id
        .to_owned();

    let client = AsyncClient::new(google_client_id);
    let payload = client.validate_id_token(token.token).await.unwrap();
    dbg!(&payload);
    format::empty()
}

pub async fn google_ott<
    V: OAuth2SessionsTrait<U>,
    U: OAuth2UserTrait<OAuth2UserProfile> + ModelTrait + Send + Sync,
>(
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Json(body): Json<GoogleTokenPayload>,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response<Body>, Error> {
    // Keep explicit Result return

    let token = body.token;

    // let mut client = oauth2_store
    //     .get_authorization_code_client("google")
    //     .await
    //     .map_err(|e| {
    //         tracing::info!("Error getting google client: {:?}", e);
    //         Error::Message(e.to_string())
    //     })?;

    // Decode the header to get the key ID
    let header = decode_header(&token)
        .map_err(|e| Error::Unauthorized(format!("JWT header decode error: {}", e)))?;
    let kid = header
        .kid
        .ok_or_else(|| Error::Unauthorized("JWT header missing 'kid'".to_string()))?;

    // Fetch Google's public keys
    let client = reqwest::Client::new();
    let jwks_response = client
        .get("https://www.googleapis.com/oauth2/v3/certs")
        .send()
        .await
        .unwrap();

    if !jwks_response.status().is_success() {
        // Adjust error type if InternalServerErrorWithMsg doesn't exist
        return Err(Error::CustomError(
            StatusCode::BAD_GATEWAY,
            ErrorDetail::new("JWKS_REQUEST_FAILED", &jwks_response.status().to_string()),
        ));
    }

    let jwks: serde_json::Value = jwks_response.json().await.map_err(|e| {
        Error::CustomError(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorDetail::new("JWKS_PARSE_FAILED", &e.to_string()),
        )
    })?;

    // Find the key more safely
    let key_data = jwks
        .get("keys")
        .and_then(|keys| keys.as_array())
        .ok_or_else(|| {
            Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("JWKS_INVALID_FORMAT", "'keys' array not found"),
            )
        })?
        .iter()
        .find(|k| k.get("kid").and_then(|v| v.as_str()) == Some(&kid))
        .ok_or_else(|| Error::Unauthorized(format!("Key '{}' not found in Google JWKS", kid)))?; // Unauthorized seems appropriate here

    let n = key_data.get("n").and_then(|v| v.as_str()).ok_or_else(|| {
        Error::CustomError(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorDetail::new("JWK_MISSING_N", "JWK missing 'n' component"),
        )
    })?;
    let e = key_data.get("e").and_then(|v| v.as_str()).ok_or_else(|| {
        Error::CustomError(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorDetail::new("JWK_MISSING_E", "JWK missing 'e' component"),
        )
    })?;

    // Step 3: Create decoding key
    let decoding_key = DecodingKey::from_rsa_components(n, e).map_err(|e| {
        Error::CustomError(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorDetail::new("DECODING_KEY_FAILED", &e.to_string()),
        )
    })?;

    let google_client_id = website.website_basic_info.google.google_client_id.clone();

    // Step 4: Setup validation
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&google_client_id]);
    validation.set_issuer(&["https://accounts.google.com"]);

    // Step 5: Decode & validate claims
    let token_data = decode::<OAuth2UserProfile>(&token, &decoding_key, &validation)
        .map_err(|e| Error::Unauthorized(format!("Invalid or expired Google token: {}", e)))?;
    dbg!(&token_data);
    let profile = token_data.claims;

    dbg!(&profile);

    // let access_token = token.clone();
    // let fields = EmptyExtraTokenFields {};
    // let token_type = TokenType {};
    // let token_response = StandardTokenResponse::new(access_token, fields, token_type);

    // // Step 6: Find or create the user
    // let user = UserModel::upsert_with_oauth(&ctx.db, &profile).await?;
    // let user2 = OAuth2SessionModel::upsert_with_oauth2(&ctx.db, &token_response, &user)
    //     .await
    //     .map_err(|_e| {
    //         tracing::error!("Error creating session");
    //         Error::InternalServerError
    //     })?;

    // let jwt_secret = ctx.config.get_jwt_config()?; // Assumes returns Result<_, ConfigError>, handled by '?'
    // let auth_token = user.generate_jwt(&jwt_secret.secret, jwt_secret.expiration.clone())?; // Assumes returns Result<_, Error>, handled by '?'

    // // Step 7: Set cookie
    // let cookie = AxumCookie::build(("auth", auth_token))
    //     .path("/")
    //     .http_only(true)
    //     .secure(!cfg!(debug_assertions))
    //     .same_site(SameSite::Lax)
    //     .max_age(time::Duration::seconds(jwt_secret.expiration as i64))
    //     .build();

    // let user_pid = UserPid::new(user.pid);
    // let (loaded_user, user_credits) = load_user_and_credits(&ctx.db, &user_pid).await?; // Assumes returns Result<_, Error>

    // let view_response =
    //     views::home::google_ott(&v, &website, &loaded_user.into(), &user_credits.into())?; // Assumes returns Result<impl IntoResponse, Error>

    // // Build the final response
    // let mut response_builder = Response::builder().status(StatusCode::OK);

    // // --- Reverted Error Mapping Below ---
    // // Safely parse the cookie string into a HeaderValue - Using your original error
    // let cookie_header_value = cookie.to_string();

    // response_builder = response_builder.header("Set-Cookie", cookie_header_value);

    // // Add the view response body - Using your original error
    // let final_response = response_builder
    //     .body(Body::from(
    //         // Ensure Body::from is available/correct
    //         view_response.into_response().into_body(),
    //     ))
    //     .map_err(|e| {
    //         Error::CustomError(
    //             // Using your original error type
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             ErrorDetail::new("RESPONSE_BUILD_FAILED", &e.to_string()), // Adjusted code slightly
    //         )
    //     })?;
    // // --- End Reverted Error Mapping ---

    // Ok(final_response) // Return Ok(...) as signature is Result<...>
    format::empty()
}

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
