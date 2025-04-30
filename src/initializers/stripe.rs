use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::domain::settings::Settings;

#[allow(clippy::module_name_repetitions)]
pub struct Stripe;

#[async_trait]
impl Initializer for Stripe {
    fn name(&self) -> String {
        "stripe".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let stripe_client = Settings::init(&ctx).stripe().await;
        let router = router.layer(Extension(stripe_client));
        Ok(router)
    }
}
