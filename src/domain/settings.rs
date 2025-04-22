use crate::service::{
    aws::s3::AwsSettings, fal_ai::fal_client::FalAiSettings, redis::redis::RedisSettings,
    stripe::stripe::StripeSettings,
};
use serde::Deserialize;

use super::website::{GoogleAnalytics, MetaPixel, WebsiteSettings};

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub aws: AwsSettings,
    pub fal_ai: FalAiSettings,
    pub website: WebsiteSettings,
    pub stripe: StripeSettings,
    pub redis: RedisSettings,
}
