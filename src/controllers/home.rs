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
    pub struct Home;
    impl Home {
        pub const BASE: &'static str = "/";
    }
}

pub fn routes() -> Routes {
    Routes::new().add(routes::Home::BASE, get(render_home))
}

#[debug_handler]
pub async fn render_home(
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let validate_route = crate::controllers::auth::routes::Auth::VALIDATE_USER;
    views::home::home(v, &website, &validate_route, is_home)
}
