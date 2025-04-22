#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::domain::website::Website;
use crate::views;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Policy;
    impl Policy {
        pub const BASE: &'static str = "/policy";
        pub const PRIVACY: &'static str = "/privacy";
        pub const COOKIE: &'static str = "/cookie";
        pub const TERMS: &'static str = "/terms-and-conditions";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Policy::BASE)
        .add(routes::Policy::PRIVACY, get(privacy))
        .add(routes::Policy::TERMS, get(terms))
        .add(routes::Policy::COOKIE, get(cookie))
}

#[debug_handler]
pub async fn cookie(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let validate_route = crate::controllers::auth::routes::Auth::VALIDATE_USER;
    views::policy::cookie(v, &website)
}

#[debug_handler]
pub async fn terms(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let validate_route = crate::controllers::auth::routes::Auth::VALIDATE_USER;
    views::policy::terms(v, &website)
}

#[debug_handler]
pub async fn privacy(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let validate_route = crate::controllers::auth::routes::Auth::VALIDATE_USER;
    views::policy::privacy(v, &website)
}
