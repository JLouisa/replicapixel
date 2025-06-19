use crate::{
    controllers::webhooks::routes::Webhooks,
    domain::{url::Url, website::WebsiteBasicInfo},
    models::{
        _entities::sea_orm_active_enums::{ImageFormat, ImageSize},
        images::{ImageNew, ImageNewList},
        TrainingModelModel,
    },
};
use futures::future::join_all;
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, fmt::Debug};
use strum_macros::Display;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FalAiClientError {
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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FalAiSettings {
    fal_key: String,
    fal_queue_url: String,
    generate_image_url: String,
    training_model_url: String,
    webhook_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
#[serde(rename_all = "lowercase")]
enum WebhookType {
    Training,
    Image,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, strum::EnumString, strum::Display)]
pub enum FalAiTrainingModel {
    #[strum(to_string = "fal-ai/flux-lora-fast-training")]
    FluxLoraFastTraining,
    #[strum(to_string = "fal-ai/flux-lora-portrait-trainer")]
    FluxLoraPortraitTrainer,
}
impl Default for FalAiTrainingModel {
    fn default() -> Self {
        Self::FluxLoraPortraitTrainer
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, strum::EnumString, strum::Display)]
pub enum FalAiImageModel {
    #[strum(to_string = "fal-ai/flux-lora")]
    FluxLora,
    #[strum(to_string = "fal-ai/flux-lora/inpainting")]
    FluxLoraInPainting,
    #[strum(to_string = "rundiffusion-fal/juggernaut-flux-lora")]
    JuggernautFluxLora,
    #[strum(to_string = "rundiffusion-fal/rundiffusion-photo-flux")]
    PhotoFlux,
}
impl Default for FalAiImageModel {
    fn default() -> Self {
        Self::JuggernautFluxLora
    }
}
impl FalAiImageModel {
    pub fn to_fields() -> Vec<(String, String)> {
        vec![
            (String::from("high"), String::from("High")),
            (String::from("low"), String::from("Low")),
            // (String::from("inpainting"), String::from("Inpainting")),
            (String::from("photo"), String::from("Photo Realism")),
        ]
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum WebhookPayload {
    Image(FalAiImageModel),
    Training(FalAiTrainingModel),
}
impl Default for WebhookPayload {
    fn default() -> Self {
        Self::Image(FalAiImageModel::default())
    }
}
// 👇 Custom Deserialize implementation
impl<'de> Deserialize<'de> for WebhookPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(WebhookPayload::from_str(&s))
    }
}
impl WebhookPayload {
    pub fn from_str(value: &str) -> Self {
        match value {
            "high" => Self::Image(FalAiImageModel::JuggernautFluxLora),
            "low" => Self::Image(FalAiImageModel::FluxLora),
            // "inpainting" => Self::Image(FalAiImageModel::FluxLoraInPainting),
            "photo" => Self::Image(FalAiImageModel::PhotoFlux),
            "portrait" => Self::Training(FalAiTrainingModel::FluxLoraPortraitTrainer),
            "train-fast" => Self::Training(FalAiTrainingModel::FluxLoraFastTraining),
            _ => Self::default(),
        }
    }

    pub fn webhook_url<'a>(&self, client: &'a FalAiClient) -> &'a str {
        match self {
            Self::Training(FalAiTrainingModel::FluxLoraFastTraining) => {
                &client.flux_lora_fast_training_webhook
            }
            Self::Training(FalAiTrainingModel::FluxLoraPortraitTrainer) => {
                &client.flux_lora_portrait_trainer_webhook
            }
            Self::Image(FalAiImageModel::FluxLora) => &client.flux_lora_webhook,
            Self::Image(FalAiImageModel::PhotoFlux) => &client.photo_flux_webhook,
            Self::Image(FalAiImageModel::JuggernautFluxLora) => {
                &client.juggernaut_flux_lora_webhook
            }
            Self::Image(FalAiImageModel::FluxLoraInPainting) => {
                &client.photo_flux_inpainting_webhook
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FalAiClient {
    client: ReqwestClient,
    fal_key: String,
    pub image_url: String,
    pub training_url: String,
    pub webhook_image: String,
    pub webhook_training: String,
    pub flux_lora_fast_training: String,
    pub flux_lora_portrait_trainer: String,
    pub flux_lora_portrait_trainer_webhook: String,
    pub flux_lora_fast_training_webhook: String,
    pub flux_lora: String,
    pub flux_lora_webhook: String,
    pub juggernaut_flux_lora: String,
    pub juggernaut_flux_lora_webhook: String,
    pub photo_flux: String,
    pub photo_flux_webhook: String,
    pub photo_flux_inpainting: String,
    pub photo_flux_inpainting_webhook: String,
}

impl FalAiClient {
    pub fn new(settings: &FalAiSettings, website: &WebsiteBasicInfo) -> Self {
        let _site = website.site.to_owned();
        let site = String::from("https://replicapixel.com");
        let fal_site = settings.fal_queue_url.to_owned();
        let webhook_image_webhook = format!(
            "{}{}{}{}",
            &settings.webhook_url,
            &site,
            Webhooks::BASE,
            Webhooks::API_FAL_AI_IMAGE
        );
        let webhook_training_webhook = format!(
            "{}{}{}{}",
            &settings.webhook_url,
            &site,
            Webhooks::BASE,
            Webhooks::API_FAL_AI_TRAINING
        );
        Self {
            client: ReqwestClient::new(),
            fal_key: settings.fal_key.to_string(),
            image_url: settings.generate_image_url.to_string(),
            training_url: settings.training_model_url.to_string(),
            webhook_image: webhook_image_webhook.clone(),
            webhook_training: webhook_training_webhook.clone(),
            flux_lora_fast_training: format!(
                "{}/{}",
                &fal_site,
                FalAiTrainingModel::FluxLoraFastTraining.to_string(),
            ),
            flux_lora_portrait_trainer: format!(
                "{}/{}",
                &fal_site,
                FalAiTrainingModel::FluxLoraPortraitTrainer.to_string(),
            ),
            flux_lora_portrait_trainer_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiTrainingModel::FluxLoraPortraitTrainer.to_string(),
                &webhook_training_webhook
            ),
            flux_lora_fast_training_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiTrainingModel::FluxLoraFastTraining.to_string(),
                &webhook_training_webhook
            ),
            flux_lora: format!("{}/{}", &fal_site, FalAiImageModel::FluxLora.to_string(),),
            flux_lora_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiImageModel::FluxLora.to_string(),
                &webhook_image_webhook
            ),
            juggernaut_flux_lora: format!(
                "{}/{}",
                &fal_site,
                FalAiImageModel::JuggernautFluxLora.to_string(),
            ),
            juggernaut_flux_lora_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiImageModel::JuggernautFluxLora.to_string(),
                &webhook_image_webhook
            ),
            photo_flux: format!("{}/{}", &fal_site, FalAiImageModel::PhotoFlux.to_string(),),
            photo_flux_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiImageModel::PhotoFlux.to_string(),
                &webhook_image_webhook
            ),
            photo_flux_inpainting: format!(
                "{}/{}",
                &fal_site,
                FalAiImageModel::FluxLoraInPainting.to_string(),
            ),
            photo_flux_inpainting_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiImageModel::FluxLoraInPainting.to_string(),
                &webhook_image_webhook
            ),
        }
    }

    /// Sends an image generation request via a webhook to the Flux Lora API.
    ///
    /// This function posts a serialized request body (`V`) to the configured Flux Lora webhook
    /// endpoint and attempts to deserialize the response into type `R`.
    ///
    /// # Type Parameters
    /// - `V`: The request type. Must implement `FluxExt` and be serializable with `serde::Serialize`.
    /// - `R`: The expected response type. Must implement `serde::de::DeserializeOwned`.
    ///
    /// # Arguments
    /// - `body`: A reference to the request payload.
    ///
    /// # Returns
    /// - `Ok(R)`: If the request succeeds and the response is deserialized.
    /// - `Err(FalAiClientError)`: If any part of the request or response handling fails.
    pub async fn send_queue_webhook_all<V, R>(&self, body: &V) -> Result<R, FalAiClientError>
    where
        V: FluxExt + Serialize + Debug,
        R: DeserializeOwned,
    {
        let response = self
            .client
            .post(body.model().webhook_url(self))
            .header("Authorization", format!("Key {}", &self.fal_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?
            .json::<R>()
            .await?;
        Ok(response)
    }

    pub async fn send_image_queue_many_async(
        &self,
        list: &ImageNewList,
    ) -> Result<ImageNewList, FalAiClientError> {
        let futures = list.clone().into_inner().into_iter().map(|mut item| {
            let body = item.clone().into();
            let client = self.clone();
            async move {
                match client
                    .send_queue_webhook_all::<FluxLoraImageGenerate, QueueResponse>(&body)
                    .await
                {
                    Ok(response) => {
                        item.fal_ai_request_id = Some(response.request_id);
                        Ok(item)
                    }
                    Err(e) => Err(e),
                }
            }
        });

        let results: Vec<Result<ImageNew, FalAiClientError>> = join_all(futures).await;

        // Collect only successful results
        let mut successful: Vec<ImageNew> = Vec::new();
        for res in results {
            match res {
                Ok(item) => successful.push(item),
                Err(e) => tracing::error!("Failed to send image queue: {:?}", e),
            }
        }
        Ok(ImageNewList::new(successful))
    }

    pub async fn retry(
        &self,
        mut response: ImageNewList,
        list_img: &ImageNewList,
    ) -> Result<ImageNewList, FalAiClientError> {
        if response.as_ref().len() != list_img.as_ref().len() {
            let missing_images: Vec<ImageNew> = list_img
                .as_ref()
                .iter()
                .filter(|item| !response.as_ref().contains(item))
                .cloned()
                .collect();
            let missing_images = ImageNewList::new(missing_images);

            // second request
            let second_try = self.send_image_queue_many_async(&missing_images).await?;

            // Extend response manually
            let mut response_inner = response.into_inner();
            response_inner.extend(second_try.into_inner());
            response = ImageNewList::new(response_inner);
        }

        Ok(response)
    }

    pub async fn request_result_training(
        &self,
        request_id: &str,
    ) -> Result<SuccessfulPayloadTraining, FalAiClientError> {
        let response = self
            .client
            .get(format!("{}/requests/{}", &self.image_url, request_id))
            .header("Authorization", format!("Key {}", &self.fal_key))
            .send()
            .await
            .map_err(|_| {
                loco_rs::Error::Message("Error processing Result Request: 103".to_string())
            })?
            .json::<SuccessfulPayloadTraining>()
            .await?;

        Ok(response)
    }
    pub async fn request_result_image(
        &self,
        request_id: &str,
    ) -> Result<SuccessfulPayload, FalAiClientError> {
        let response = self
            .client
            .get(format!("{}/requests/{}", &self.image_url, request_id))
            .header("Authorization", format!("Key {}", &self.fal_key))
            .send()
            .await
            .map_err(|_| {
                loco_rs::Error::Message("Error processing Result Request: 103".to_string())
            })?
            .json::<SuccessfulPayload>()
            .await?;

        Ok(response)
    }

    pub async fn request_cancel(&self, request_id: &str) -> Result<(), FalAiClientError> {
        let response = self
            .client
            .put(format!(
                "{}/requests/{}/cancel",
                &self.image_url, request_id
            ))
            .header("Authorization", format!("Key {}", &self.fal_key))
            .send()
            .await
            .map_err(|_| {
                loco_rs::Error::Message("Error processing Cancel Request: 104".to_string())
            })?;

        println!("Response: {:?}", response);

        Ok(())
    }
}

// ========================
// Enums
// ========================

/// Represents the status of a response.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StatusResponse {
    Ok,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FluxStatus {
    #[serde(rename = "IN_QUEUE")]
    InQueue,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "COMPLETED")]
    Completed,
}

impl Default for FluxStatus {
    fn default() -> Self {
        Self::InProgress
    }
}

/// Represents the size of an image.
// #[derive(Debug, Clone, Copy, EnumString, Serialize, Deserialize, EnumIter, PartialEq, Eq)]

// ========================
// Structs: Image and Payloads
// ========================

/// Represents an image with metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub url: String,
    pub content_type: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub file_data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessfulPayloadTraining {
    pub diffusers_lora_file: FileInfo,
    pub config_file: FileInfo,
    pub debug_preprocessed_output: Option<FileInfo>,
}
impl SuccessfulPayloadTraining {
    pub fn lora(&self) -> String {
        self.diffusers_lora_file.url.to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub content_type: Option<String>,
}

/// Represents a successful response payload containing images and a seed.
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulPayload {
    pub images: Vec<Image>,
    pub prompt: String,
    pub seed: u64,
    pub timings: Option<serde_json::Value>,
    pub has_nsfw_concepts: Option<Vec<bool>>,
}
impl SuccessfulPayload {
    pub fn image_url(&self) -> Option<String> {
        self.images
            .first()
            .map(|first_image| first_image.url.clone())
    }
}

/// Represents details of an error.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    loc: Vec<String>,
    msg: String,
    r#type: String,
}

/// Represents an error response payload containing error details.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    detail: Vec<ErrorDetail>,
}

// ========================
// Struct: Webhook Response
// ========================

/// Represents the response from the Flux API webhook.
#[derive(Debug, Serialize, Deserialize)]
pub struct FluxApiWebhookResponse {
    pub request_id: String,
    pub gateway_request_id: String,
    pub status: StatusResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_error: Option<String>,
}

impl FluxApiWebhookResponse {
    /// Extracts a successful payload from the response.
    pub fn successful_img_opt(&self) -> Option<String> {
        let value = self.payload.clone()?;
        let new_value: SuccessfulPayload = serde_json::from_value(value).ok()?;
        new_value
            .images
            .first()
            .map(|first_image| first_image.url.clone())
    }
    pub fn successful_training_opt(&self) -> Option<String> {
        let value = self.payload.clone()?;
        let payload: SuccessfulPayloadTraining = serde_json::from_value(value).ok()?;
        Some(payload.lora())
    }
    /// Extracts an error payload from the response.
    pub fn error(&self) -> ErrorPayload {
        let payload: ErrorPayload = serde_json::from_value(self.payload.clone().unwrap()).unwrap();
        payload
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluxResponse {
    pub request_id: String,
    pub gateway_request_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluxStatusResponse {
    status: FluxStatus,
    queue_position: i32,
    response_url: String,
}

//? Working ================================

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueueResponse {
    pub status: String,
    pub request_id: String,
    pub response_url: String,
    pub status_url: String,
    pub cancel_url: String,
    pub logs: Option<String>,
    pub metrics: HashMap<String, serde_json::Value>,
    pub queue_position: usize,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Lora {
    pub path: String,
    pub scale: f32,
}

#[derive(Serialize, Debug)]
pub struct UltraFineTuned {
    pub prompt: String,
    pub num_images: u8,
    pub image_size: ImageSize,
    pub steps: u16,
    pub guidance_scale: f32,
    pub enable_safety_checker: bool,
    pub safety_tolerance: i32,
    pub output_format: ImageFormat,
    pub aspect_ratio: String,
    pub finetune_strength: f32,
    pub finetune_id: Lora,
}

/// Represents a request to generate images using Flux Lora.
#[derive(Serialize, Debug)]
pub struct FluxLoraImageGenerate {
    pub prompt: String,
    pub image_size: ImageSize,
    #[serde(rename = "steps")]
    pub num_inference_steps: u16,
    pub guidance_scale: f32,
    pub num_images: u8,
    pub enable_safety_checker: bool,
    pub output_format: ImageFormat,
    pub loras: Vec<Lora>,
    pub photo_lora_scale: f32,
    #[serde(skip_serializing)]
    pub model: WebhookPayload,
}
impl From<ImageNew> for FluxLoraImageGenerate {
    fn from(value: ImageNew) -> Self {
        Self {
            prompt: value.sys_prompt.into_inner(),
            image_size: value.image_size,
            num_inference_steps: value.num_inference_steps as u16,
            loras: value.loras,
            model: value.model,
            ..Default::default()
        }
    }
}
impl Default for FluxLoraImageGenerate {
    fn default() -> Self {
        Self {
            prompt: "".to_string(),
            image_size: ImageSize::default(),
            num_inference_steps: 28,
            guidance_scale: 3.5,
            num_images: 1,
            enable_safety_checker: false,
            output_format: ImageFormat::default(),
            photo_lora_scale: 1.0,
            loras: vec![],
            model: WebhookPayload::Image(FalAiImageModel::default()),
        }
    }
}

/// Represents the schema for training a Lora model.
#[derive(Serialize, Deserialize, Debug)]
pub struct FluxLoraTrainingSchema {
    pub images_data_url: String,
    pub trigger_word: String,
    pub steps: i32,
    pub create_mask: bool,
    pub is_style: bool,
    #[serde(skip_serializing)]
    pub model: WebhookPayload,
}
impl FluxLoraTrainingSchema {
    pub fn from_training(value: &TrainingModelModel, images_data_url: Url) -> Self {
        Self {
            images_data_url: images_data_url.into_inner(),
            trigger_word: value.trigger_word.clone(),
            steps: value.steps.clone(),
            create_mask: value.create_mask.clone(),
            is_style: value.is_style.clone(),
            model: WebhookPayload::Training(FalAiTrainingModel::default()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluxQueueResponse {
    pub request_id: String,
    pub response_url: String,
    pub status_url: String,
    pub cancel_url: String,
}

pub trait FluxExt {
    fn model(&self) -> WebhookPayload;
}
impl FluxExt for FluxLoraImageGenerate {
    fn model(&self) -> WebhookPayload {
        self.model.clone()
    }
}
impl FluxExt for FluxLoraTrainingSchema {
    fn model(&self) -> WebhookPayload {
        self.model.clone()
    }
}
