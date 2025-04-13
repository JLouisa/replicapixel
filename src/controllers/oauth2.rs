use std::fmt::Debug;

use loco_oauth2::controllers::middleware::OAuth2CookieUser;
use loco_oauth2::models::oauth2_sessions::OAuth2SessionsTrait;
use loco_oauth2::models::users::OAuth2UserTrait;
use loco_rs::prelude::*;

use axum::extract::Query;
use axum::Extension;
use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie as AxumCookie, SameSite};
use axum_session::{DatabasePool, Session, SessionNullPool};

use serde::de::DeserializeOwned;
use time::Duration;

use loco_oauth2::controllers::oauth2::{callback, callback_jwt};
use loco_oauth2::controllers::oauth2::{get_authorization_url, AuthParams};
use loco_oauth2::controllers::oauth2::{google_authorization_url, google_callback_cookie};
use loco_oauth2::OAuth2ClientStore;

use crate::{
    models::{o_auth2_sessions, users, users::OAuth2UserProfile},
    views::auth::LoginResponse,
};

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/oauth2")
        .add("/google", get(google_authorization_url::<SessionNullPool>))
        .add(
            "/google/callback/jwt",
            get(google_callback_jwt::<
                OAuth2UserProfile,
                users::Model,
                o_auth2_sessions::Model,
                SessionNullPool,
            >),
        )
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
    let user = callback_jwt::<T, U, V, W>(&ctx, session, params, &mut client).await?;
    drop(client);

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let cookie = AxumCookie::build(("auth", token))
        .path("/")
        .http_only(true)
        .secure(!cfg!(debug_assertions)) // set to false in localhost
        .same_site(SameSite::Strict)
        .max_age(time::Duration::seconds(jwt_secret.expiration as i64))
        .build();

    let mut response = Redirect::to("/dashboard").into_response();
    response
        .headers_mut()
        .append("Set-Cookie", cookie.to_string().parse().unwrap());

    Ok(response)
}
