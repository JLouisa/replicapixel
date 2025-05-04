use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::domain::settings::Settings;

#[allow(clippy::module_name_repetitions)]
pub struct FalAi;

#[async_trait]
impl Initializer for FalAi {
    fn name(&self) -> String {
        "fal_ai_client".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let fal_ai_client = Settings::init(&ctx).fal_ai();
        let router = router.layer(Extension(fal_ai_client));
        Ok(router)
    }
}
