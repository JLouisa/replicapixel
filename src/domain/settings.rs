use crate::service::{
    aws::s3::AwsSettings, fal_ai::fal_client::FalAiSettings, redis::redis::RedisSettings,
    stripe::stripe::StripeSettings,
};
use serde::Deserialize;

use super::website::WebsiteBasicInfo;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub website: WebsiteBasicInfo,
    pub aws: AwsSettings,
    pub fal_ai: FalAiSettings,
    pub stripe: StripeSettings,
    pub redis: RedisSettings,
}
