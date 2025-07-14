use crate::{
    controllers::webhooks::routes::Webhooks,
    domain::{url::Url, website::WebsiteBasicInfo},
    models::{
        _entities::sea_orm_active_enums::{AspectRatio, ImageFormat, ImageSize},
        images::{ImageNew, ImageNewList},
        videos::VideoNew,
        TrainingModelModel,
    },
};
use futures::future::join_all;
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, fmt::Debug};
use strum::EnumString;
use strum_macros::Display;

use rand::Rng;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::time::Duration;
use thiserror::Error;
use tokio_retry::strategy::ExponentialBackoff;
use tokio_retry::Retry;

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
    #[error("Fal AI error: {0}")]
    FalApiError(String),
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
pub enum FalAiVideoModel {
    #[strum(to_string = "fal-ai/veo3/fast")]
    Veo3,
}
impl Default for FalAiVideoModel {
    fn default() -> Self {
        Self::Veo3
    }
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
        Self::FluxLoraFastTraining
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
    Video(FalAiVideoModel),
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
            Self::Video(FalAiVideoModel::Veo3) => &client.veo3_webhook,
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
    pub veo3: String,
    pub veo3_webhook: String,
}

impl FalAiClient {
    pub fn new(settings: &FalAiSettings, website: &WebsiteBasicInfo) -> Self {
        let _site = website.site.to_owned();
        let site = String::from("https://replicapixel.com"); // Because of local dev
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
        let webhook_video_webhook = format!(
            "{}{}{}{}",
            &settings.webhook_url,
            &site,
            Webhooks::BASE,
            Webhooks::API_FAL_AI_VIDEO
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
            veo3: format!("{}/{}", &fal_site, FalAiVideoModel::Veo3.to_string(),),
            veo3_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiVideoModel::Veo3.to_string(),
                &webhook_video_webhook
            ),
        }
    }

    pub async fn send_queue_webhook_with_retries<V, R>(
        &self,
        body: &V,
        retries: usize,
    ) -> Result<R, FalAiClientError>
    where
        V: FluxExt + Serialize + Debug,
        R: DeserializeOwned,
    {
        let base_strategy = ExponentialBackoff::from_millis(300)
            .factor(2)
            .max_delay(Duration::from_secs(3))
            .take(retries);

        // Apply jitter to each delay in the strategy
        let jittered_strategy = base_strategy.map(|delay| {
            let jitter_factor = rand::rng().random_range(0.8..1.2); // ±20% jitter
            let jittered_millis = (delay.as_millis() as f64 * jitter_factor) as u64;
            Duration::from_millis(jittered_millis)
        });

        let fal_response = Retry::spawn(jittered_strategy, || async {
            self.send_queue_webhook_all::<V, R>(&body)
                .await
                .map_err(|err| {
                    tracing::warn!(error = ?err, "Fal AI request failed, retrying...");
                    err
                })
        })
        .await?;

        Ok(fal_response)
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
    pub fn successful_video_opt(&self) -> Option<String> {
        let value = self.payload.clone()?;
        let payload: SuccessFalVideoPayload = serde_json::from_value(value).ok()?;
        Some(payload.video_url())
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
impl QueueResponse {
    pub fn test() -> QueueResponse {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            ..Default::default()
        }
    }
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
impl FluxExt for FalVideoSend {
    fn model(&self) -> WebhookPayload {
        self.model.clone()
    }
}

// ==================================================

#[derive(Serialize, Debug)]
pub struct FalVideoSend {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub aspect_ratio: AspectRatio,
    pub duration: DurationSeconds,
    pub enhance_prompt: bool,
    pub seed: Option<i32>,
    pub generate_audio: bool,
    pub model: WebhookPayload,
}
impl From<VideoNew> for FalVideoSend {
    fn from(value: VideoNew) -> Self {
        Self {
            prompt: value.sys_prompt.into_inner(),
            negative_prompt: value.negative_prompt.into_inner(),
            aspect_ratio: value.aspect_ratio,
            duration: value.duration,
            enhance_prompt: value.enhance_prompt,
            seed: value.seed,
            generate_audio: value.generate_audio,
            model: value.model,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SuccessFalVideoPayload {
    pub video: VideoUrl,
}
impl SuccessFalVideoPayload {
    pub fn video_url(&self) -> String {
        self.video.url.to_owned()
    }
}

#[derive(Deserialize, Debug)]
pub struct VideoUrl {
    pub url: String,
}

#[derive(Serialize, Debug, Clone, Deserialize, EnumString)]
pub enum DurationSeconds {
    #[serde(rename = "01s")]
    #[strum(to_string = "01")]
    One,
    #[serde(rename = "02s")]
    #[strum(to_string = "02")]
    Two,
    #[serde(rename = "03s")]
    #[strum(to_string = "03")]
    Three,
    #[serde(rename = "04s")]
    #[strum(to_string = "04")]
    Four,
    #[serde(rename = "05s")]
    #[strum(to_string = "05")]
    Five,
    #[serde(rename = "06s")]
    #[strum(to_string = "06")]
    Six,
    #[serde(rename = "07s")]
    #[strum(to_string = "07")]
    Seven,
    #[serde(rename = "08s")]
    #[strum(to_string = "08")]
    Eight,
    #[serde(rename = "09s")]
    #[strum(to_string = "09")]
    Nine,
    #[serde(rename = "10s")]
    #[strum(to_string = "10")]
    Ten,
    #[serde(rename = "11s")]
    #[strum(to_string = "11")]
    Eleven,
    #[serde(rename = "12s")]
    #[strum(to_string = "12")]
    Twelve,
    #[serde(rename = "13s")]
    #[strum(to_string = "13")]
    Thirteen,
    #[serde(rename = "14s")]
    #[strum(to_string = "14")]
    Fourteen,
    #[serde(rename = "15s")]
    #[strum(to_string = "15")]
    Fifteen,
}

impl DurationSeconds {
    pub fn to_int(&self) -> i32 {
        match self {
            DurationSeconds::One => 1,
            DurationSeconds::Two => 2,
            DurationSeconds::Three => 3,
            DurationSeconds::Four => 4,
            DurationSeconds::Five => 5,
            DurationSeconds::Six => 6,
            DurationSeconds::Seven => 7,
            DurationSeconds::Eight => 8,
            DurationSeconds::Nine => 9,
            DurationSeconds::Ten => 10,
            DurationSeconds::Eleven => 11,
            DurationSeconds::Twelve => 12,
            DurationSeconds::Thirteen => 13,
            DurationSeconds::Fourteen => 14,
            DurationSeconds::Fifteen => 15,
        }
    }
}

impl Default for DurationSeconds {
    fn default() -> Self {
        Self::Eight
    }
}
