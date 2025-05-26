use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::domain::settings::Settings;

#[allow(clippy::module_name_repetitions)]
pub struct Other;

#[async_trait]
impl Initializer for Other {
    fn name(&self) -> String {
        "other".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let other = Settings::init(&ctx).other();
        let router = router.layer(Extension(other));
        Ok(router)
    }
}
