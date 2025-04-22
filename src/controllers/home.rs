#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::cookie::ExtractConsentState;
use crate::views;
use crate::{domain::website::Website, middleware::cookie::CookieConsentLayer};
use axum::{debug_handler, Extension};
use loco_rs::{db, prelude::*};

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Home;
    impl Home {
        pub const BASE: &'static str = "/";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add(routes::Home::BASE, get(render_home))
        .layer(CookieConsentLayer::new())
}

#[debug_handler]
pub async fn render_home(
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let validate_route = crate::controllers::auth::routes::Auth::VALIDATE_USER;
    views::home::home(v, &website, &validate_route, is_home, &cc_cookie)
}
