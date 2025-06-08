#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::views;
use crate::{controllers::dashboard::WebsiteOptions, domain::website::Website};
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct OtherRoutes {
        pub documentation: String,
    }
    impl OtherRoutes {
        pub fn init() -> Self {
            Self {
                documentation: String::from(Other::DOCUMENTATION),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Other;
    impl Other {
        pub const DOCUMENTATION: &'static str = "/documentation";
    }
}

pub fn routes() -> Routes {
    Routes::new().add(routes::Other::DOCUMENTATION, get(documentation))
}

#[debug_handler]
pub async fn documentation(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let website_options: WebsiteOptions = WebsiteOptions::new().website(&website);
    views::other::documentation(v, &website_options)
}
