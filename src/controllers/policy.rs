#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::domain::website::Website;
use crate::views;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PolicyRoutes {
        pub base: String,
        pub privacy: String,
        pub cookie: String,
        pub terms: String,
        pub model_consent: String,
    }
    impl PolicyRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Policy::BASE),
                privacy: format!("{}{}", Policy::BASE, Policy::PRIVACY),
                cookie: format!("{}{}", Policy::BASE, Policy::COOKIE),
                terms: format!("{}{}", Policy::BASE, Policy::TERMS),
                model_consent: format!("{}{}", Policy::BASE, Policy::MODEL_CONSENT),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Policy;
    impl Policy {
        pub const BASE: &'static str = "/policy";
        pub const PRIVACY: &'static str = "/privacy";
        pub const COOKIE: &'static str = "/cookie";
        pub const TERMS: &'static str = "/terms-and-conditions";
        pub const MODEL_CONSENT: &'static str = "/model-consent";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Policy::BASE)
        .add(routes::Policy::PRIVACY, get(privacy))
        .add(routes::Policy::TERMS, get(terms))
        .add(routes::Policy::COOKIE, get(cookie))
        .add(routes::Policy::MODEL_CONSENT, get(model_consent))
}

#[debug_handler]
pub async fn model_consent(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::policy::model_consent(v, &website)
}

#[debug_handler]
pub async fn cookie(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::policy::cookie(v, &website)
}

#[debug_handler]
pub async fn terms(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::policy::terms(v, &website)
}

#[debug_handler]
pub async fn privacy(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::policy::privacy(v, &website)
}
