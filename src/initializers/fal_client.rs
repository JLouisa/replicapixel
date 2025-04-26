use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::{domain::settings::Settings, service::fal_ai::fal_client::FalAiClient};

#[allow(clippy::module_name_repetitions)]
pub struct FalAi;

#[async_trait]
impl Initializer for FalAi {
    fn name(&self) -> String {
        "fal_ai_client".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let fal_ai_settings: Settings =
            serde_json::from_value(ctx.config.settings.clone().expect("No settings found"))
                .expect("Failed to parse settings");

        let fal_ai_client = FalAiClient::new(&fal_ai_settings.fal_ai, &fal_ai_settings.website);

        let router = router.layer(Extension(fal_ai_client));
        Ok(router)
    }
}
