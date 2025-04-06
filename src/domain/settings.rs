use crate::service::{aws::s3::AwsSettings, fal_ai::fal_client::FalAiSettings};
use serde::{Deserialize, Serialize};

use super::website::WebsiteSettings;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Settings {
    pub aws: AwsSettings,
    pub fal_ai: FalAiSettings,
    pub website: WebsiteSettings,
}
