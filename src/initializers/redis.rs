use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::{domain::settings::Settings, service::redis::redis::Redis};

#[allow(clippy::module_name_repetitions)]
pub struct RedisClient;

#[async_trait]
impl Initializer for RedisClient {
    fn name(&self) -> String {
        "redis".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let settings: Settings = serde_json::from_value(
            ctx.config
                .settings
                .clone()
                .expect("No redis settings found"),
        )
        .expect("Failed to parse redis settings");

        let redis_client = Redis::new(&settings.redis)
            .await
            .expect("Failed to create redis client");

        let router = router.layer(Extension(redis_client));
        Ok(router)
    }
}
