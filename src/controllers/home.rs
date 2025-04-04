#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::views;
use axum::debug_handler;
use loco_rs::prelude::*;

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Home;
    impl Home {
        pub const BASE: &'static str = "/";
    }
}

pub fn routes() -> Routes {
    Routes::new().add(routes::Home::BASE, get(render_home))
}

#[debug_handler]
pub async fn render_home(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::home::home(v)
}
