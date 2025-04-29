use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::{domain::settings::Settings, service::redis::redis::RedisCacheDriver};

#[allow(clippy::module_name_repetitions)]
pub struct RedisClient;

#[async_trait]
impl Initializer for RedisClient {
    fn name(&self) -> String {
        "redis".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let redis_client = Settings::init(&ctx).redis().await;
        let router = router.layer(Extension(redis_client));
        Ok(router)
    }
}
