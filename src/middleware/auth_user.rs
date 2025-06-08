// in a new file, e.g., src/auth/extractor.rs
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{header, request::Parts, HeaderMap},
    response::{IntoResponse, Redirect},
};
use loco_rs::prelude::*;

use crate::{
    controllers::auth::HxRedirect,
    models::{_entities::users, users::UserPid, UserModel},
};

const HX_REQUEST_HEADER: &str = "HX-Request";

// This struct will be our extractor. It holds the authenticated user.
pub struct Auth(pub UserModel);
// Implement the FromRequestParts trait for the Auth struct
impl<S> FromRequestParts<S> for Auth
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        let ctx: AppContext = AppContext::from_ref(state);

        // 1. Run the original JWT extractor.
        let auth_result = auth::JWT::from_request_parts(parts, state).await;

        let auth = match auth_result {
            Ok(auth) => auth,
            Err(_) => {
                // 2. If it fails, create the appropriate redirect.
                // Check if the request is from HTMX.
                let is_htmx_request = parts.headers.contains_key(HX_REQUEST_HEADER);
                let response = if is_htmx_request {
                    HxRedirect::login().into_response()
                } else {
                    Redirect::to("/login").into_response()
                };
                return Ok(response);
            }
        };

        // 3. If JWT is valid, load the user from the database.
        let user = UserModel::find_by_pid(&ctx.db, &auth.claims.pid)
            .await
            .map_err(|e| {
                tracing::error!("User from valid JWT not found: {}", e);
                // Even with a valid JWT, if the user doesn't exist, redirect to login.
                Redirect::to("/login").into_response()
            })?;

        // 4. If all is well, return the user wrapped in our Auth struct.
        Ok(Auth(user))
    }
}
