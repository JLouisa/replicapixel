use crate::service::{aws::s3::AwsSettings, fal_ai::fal_client::FalAiSettings};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Settings {
    pub aws: AwsSettings,
    pub fal_ai: FalAiSettings,
}
