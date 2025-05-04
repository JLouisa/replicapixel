#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::cookie::ExtractConsentState;
use crate::views;
use crate::{domain::website::Website, middleware::cookie::CookieConsentLayer};
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct HomeRoutes {
        pub base: String,
    }
    impl HomeRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Home::BASE),
            }
        }
    }

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
    views::home::home(v, &website, is_home, &cc_cookie)
}
