use chrono::Utc;
use reqwest::Client as ReqwestClient;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::domain::website::MetaPixel;
use crate::models::PlanModel;
use crate::models::TransactionModel;
use crate::models::UserModel;

#[derive(Debug, Clone)]
pub struct MetaConversionApiClient {
    client: ReqwestClient,
    meta_url: String,
}

impl MetaConversionApiClient {
    pub fn new(meta: &MetaPixel) -> Self {
        let meta_access_token = meta
            .meta_pixel_secret
            .clone()
            .expect("Meta Pixel Secret is not set");
        let meta_pixel_id = meta.meta_pixel_id.expect("Meta Pixel ID is not set");

        let url = format!(
            "https://graph.facebook.com/v19.0/{}/events?access_token={}",
            &meta_pixel_id, &meta_access_token
        );

        Self {
            client: ReqwestClient::new(),
            meta_url: url,
        }
    }

    pub async fn meta_conversion_api(
        &self,
        user: &UserModel,
        txn: &TransactionModel,
    ) -> Result<(), MetaConversionApiError> {
        let event = ConversionPayload::purchase(user, txn);

        let res = self.client.post(&self.meta_url).json(&event).send().await?;

        let status = res.status();
        let body = res.text().await?;

        if status.is_success() {
            tracing::debug!("✅ Meta Pixel Success: {}", body);
        } else {
            tracing::error!("❌ Meta Pixel Error ({}): {}", status, body);
        }

        Ok(())
    }
    pub async fn meta_conversion_api_test(
        &self,
        user: &UserModel,
        plan: &PlanModel,
    ) -> Result<(), MetaConversionApiError> {
        let event = ConversionPayload::initiate_checkout(user, plan).test_code("");

        let res = self.client.post(&self.meta_url).json(&event).send().await?;

        let status = res.status();
        let body = res.text().await?;

        if status.is_success() {
            tracing::debug!("✅ Meta Pixel Success: {}", body);
        } else {
            tracing::error!("❌ Meta Pixel Error ({}): {}", status, body);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum MetaConversionApiError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Failed to parse JSON: {0}")]
    JsonParse(String),
    #[error("Unexpected error: {0}")]
    Other(String),
    #[error("Unexpected error: {0}")]
    SerdeErr(#[from] SerdeError),
    #[error("Unexpected error: {0}")]
    LocoError(#[from] loco_rs::Error),
    #[error("Unexpected error: {0}")]
    ReqwestErr(#[from] ReqwestError),
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConversionPayload {
    pub data: Vec<EventData>,
    pub test_event_code: Option<String>,
}
impl ConversionPayload {
    fn purchase(user: &UserModel, txn: &TransactionModel) -> Self {
        Self {
            data: vec![EventData::purchase(user, txn)],
            test_event_code: None,
        }
    }
    fn initiate_checkout(user: &UserModel, plan: &PlanModel) -> Self {
        Self {
            data: vec![EventData::initiate_checkout(user, plan)],
            test_event_code: None,
        }
    }
    fn test_code(self, code: &str) -> Self {
        Self {
            test_event_code: Some(code.to_owned()),
            ..self
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventData {
    event_name: Option<String>,
    event_time: i64,
    event_source_url: String,
    action_source: String,
    user_data: Option<UserData>,
    attribution_data: AttributionData,
    custom_data: Option<CustomData>,
    original_event_data: OriginalEventData,
}
impl EventData {
    fn purchase(user: &UserModel, txn: &TransactionModel) -> Self {
        let user_data = Some(UserData::new(user));
        let custom_data = Some(CustomData::purchase(txn));
        Self {
            event_name: Some("Purchase".to_string()),
            user_data,
            custom_data,
            original_event_data: OriginalEventData {
                event_name: Some("Purchase".to_string()),
                event_time: Utc::now().timestamp(),
            },
            ..Self::default()
        }
    }
    fn initiate_checkout(user: &UserModel, plan: &PlanModel) -> Self {
        let user_data = Some(UserData::new(user));
        let custom_data = Some(CustomData::initiate_checkout(plan));
        Self {
            event_name: Some("InitiateCheckout".to_string()),
            user_data,
            custom_data,
            original_event_data: OriginalEventData {
                event_name: Some("InitiateCheckout".to_string()),
                event_time: Utc::now().timestamp(),
            },
            ..Self::default()
        }
    }
}
impl Default for EventData {
    fn default() -> Self {
        let time = Utc::now().timestamp();
        Self {
            event_name: None,
            event_time: time,
            event_source_url: "https://replicapixel.com".to_string(),
            action_source: "website".to_string(),
            user_data: None,
            attribution_data: AttributionData {
                attribution_share: "0.3".to_string(),
            },
            custom_data: None,
            original_event_data: OriginalEventData {
                event_name: None,
                event_time: time,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserData {
    pub em: Vec<Option<String>>,
    #[serde(rename = "fn")]
    pub r#fn: Vec<Option<String>>,
    pub ln: Vec<Option<String>>,
}
impl UserData {
    fn new(user: &UserModel) -> Self {
        let (first_name, last_name) = split_full_name(user);
        Self {
            em: vec![sha256_hash(&user.email)],
            r#fn: vec![first_name],
            ln: vec![last_name],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributionData {
    pub attribution_share: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomData {
    pub currency: String,
    pub value: String,
}
impl CustomData {
    fn purchase(txn: &TransactionModel) -> Self {
        Self {
            currency: txn.currency.to_string().to_uppercase(),
            value: txn.payment_amount.to_string(),
        }
    }
    fn initiate_checkout(plan: &PlanModel) -> Self {
        let value = (plan.price_cents / 100) as f64;
        Self {
            currency: "usd".to_string().to_uppercase(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalEventData {
    pub event_name: Option<String>,
    pub event_time: i64,
}

fn sha256_hash(value: &str) -> Option<String> {
    let trimmed = value.trim().to_lowercase();
    if trimmed.is_empty() {
        return None;
    }
    let mut hasher = Sha256::new();
    hasher.update(trimmed);
    Some(hex::encode(hasher.finalize()))
}

pub fn split_full_name(user: &UserModel) -> (Option<String>, Option<String>) {
    let parts: Vec<&str> = user.name.trim().split_whitespace().collect();

    match parts.len() {
        0 => (None, None),
        1 => (Some(parts[0].to_string()), None),
        _ => {
            let first_name = parts[0].to_string();
            let last_name = parts[1..].join(" ");
            (
                Some(first_name),
                if last_name.is_empty() {
                    None
                } else {
                    Some(last_name)
                },
            )
        }
    }
}
