#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{domain::website::Website, views};
use axum::{debug_handler, Extension};

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct StarterRoutes {
        pub base: String,
        pub starter: String,
        pub starter_partial: String,
    }
    impl StarterRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Starter::BASE),
                starter: format!("{}{}", Starter::BASE, Starter::STARTER),
                starter_partial: format!("{}{}", Starter::BASE, Starter::STARTER_PARTIAL),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Starter;
    impl Starter {
        pub const BASE: &'static str = "/starter";
        pub const STARTER: &'static str = "";
        pub const STARTER_PARTIAL: &'static str = "/partial";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Starter::BASE)
        .add(routes::Starter::STARTER, get(starter))
        .add(routes::Starter::STARTER_PARTIAL, get(starter_partial))
}

#[debug_handler]
pub async fn starter(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::starter::starter(v, &website)
}

#[debug_handler]
pub async fn starter_partial(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::starter::starter_partial(v, &website)
}
