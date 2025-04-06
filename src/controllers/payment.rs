#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

use crate::{domain::website::Website, views};

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Payment;
    impl Payment {
        pub const BASE: &'static str = "/";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("payments")
        .add(routes::Payment::BASE, get(payment_home))
}

#[debug_handler]
pub async fn payment_home(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    views::payment::payment_home(v, &website)
}
