use crate::service::{
    aws::s3::{AwsS3, AwsSettings},
    fal_ai::fal_client::{FalAiClient, FalAiSettings},
    redis::redis::{RedisCacheDriver, RedisSettings},
    stripe::stripe::{StripeClient, StripeSettings},
};
use loco_rs::app::AppContext;
use serde::Deserialize;

use super::website::WebsiteBasicInfo;
use crate::domain::website::Website;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub website: WebsiteBasicInfo,
    pub aws: AwsSettings,
    pub fal_ai: FalAiSettings,
    pub stripe: StripeSettings,
    pub redis: RedisSettings,
}
impl Settings {
    pub fn init(ctx: &AppContext) -> Self {
        let settings: Self =
            match serde_json::from_value(ctx.config.settings.clone().expect("No Settings found")) {
                Ok(settings) => settings,
                Err(e) => {
                    tracing::error!("Failed to parse settings: {}", e);
                    std::process::exit(1);
                }
            };
        settings
    }
    pub async fn redis(&self) -> RedisCacheDriver {
        let redis_client = match RedisCacheDriver::new(&self.redis).await {
            Ok(client) => client,
            Err(e) => {
                tracing::error!("Failed to create redis client: {}", e);
                std::process::exit(1);
            }
        };
        redis_client
    }
    pub async fn stripe(&self) -> StripeClient {
        StripeClient::new(&self.stripe)
    }
    pub async fn aws_s3(&self) -> AwsS3 {
        AwsS3::new(&self.aws).await
    }
    pub fn fal_ai(&self) -> FalAiClient {
        FalAiClient::new(&self.fal_ai, &self.website)
    }
    pub async fn website(&self, ctx: &AppContext) -> Website {
        Website::init(&self, &ctx).await
    }
}
