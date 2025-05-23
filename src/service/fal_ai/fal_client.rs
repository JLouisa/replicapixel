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
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
pub enum FalAiModel {
    #[strum(to_string = "fal-ai/flux-lora")]
    FluxLora,
    #[strum(to_string = "fal-ai/flux-lora-fast-training")]
    FluxLoraFastTraining,
    #[strum(to_string = "rundiffusion-fal/juggernaut-flux-lora")]
    JuggernautFluxLora,
    #[strum(to_string = "rundiffusion-fal/rundiffusion-photo-flux")]
    PhotoFlux,
    #[strum(to_string = "fal-ai/flux-lora/inpainting")]
    FluxLoraInPainting,
}
impl Default for FalAiModel {
    fn default() -> Self {
        Self::JuggernautFluxLora
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
                FalAiModel::FluxLoraFastTraining.to_string(),
            ),
            flux_lora_fast_training_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiModel::FluxLoraFastTraining.to_string(),
                &webhook_training_webhook
            ),
            flux_lora: format!("{}/{}", &fal_site, FalAiModel::FluxLora.to_string(),),
            flux_lora_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiModel::FluxLora.to_string(),
                &webhook_image_webhook
            ),
            juggernaut_flux_lora: format!(
                "{}/{}",
                &fal_site,
                FalAiModel::JuggernautFluxLora.to_string(),
            ),
            juggernaut_flux_lora_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiModel::JuggernautFluxLora.to_string(),
                &webhook_image_webhook
            ),
            photo_flux: format!("{}/{}", &fal_site, FalAiModel::PhotoFlux.to_string(),),
            photo_flux_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiModel::PhotoFlux.to_string(),
                &webhook_image_webhook
            ),
            photo_flux_inpainting: format!(
                "{}/{}",
                &fal_site,
                FalAiModel::FluxLoraInPainting.to_string(),
            ),
            photo_flux_inpainting_webhook: format!(
                "{}/{}{}",
                &fal_site,
                FalAiModel::FluxLoraInPainting.to_string(),
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
    pub async fn send_image_queue_webhook_all<V, R>(&self, body: &V) -> Result<R, FalAiClientError>
    where
        V: FluxExt + Serialize + Debug,
        R: DeserializeOwned,
    {
        let response = self
            .client
            .post(&self.juggernaut_flux_lora_webhook)
            .header("Authorization", format!("Key {}", &self.fal_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to process image schema: {:?}", e);
                loco_rs::Error::Message(format!("Error processing image schema: {:?}", e))
            })?
            .json::<R>()
            .await?;

        Ok(response)
    }

    pub async fn send_training_queue_webhook(
        &self,
        prompt: &FluxLoraTrainingSchema,
    ) -> Result<QueueResponse, FalAiClientError> {
        let response = self
            .client
            .post(&self.flux_lora_fast_training_webhook)
            .header("Authorization", format!("Key {}", &self.fal_key))
            .header("Content-Type", "application/json")
            .json(&prompt)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to process training schema: {:?}", e);
                loco_rs::Error::Message("Error processing training model schema: 100".to_string())
            })?
            .json::<QueueResponse>()
            .await?;

        Ok(response)
    }

    pub async fn send_image_queue_webhook(
        &self,
        body: &FluxLoraImageGenerate,
    ) -> Result<QueueResponse, FalAiClientError> {
        let response = self
            .client
            .post(&self.flux_lora_webhook)
            .header("Authorization", format!("Key {}", &self.fal_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to process image schema: {:?}", e);
                loco_rs::Error::Message(format!("Error processing image schema: {:?}", e))
            })?
            .json::<QueueResponse>()
            .await?;

        Ok(response)
    }

    pub async fn send_image_queue_many(
        &self,
        list: ImageNewList,
    ) -> Result<ImageNewList, FalAiClientError> {
        for item in &mut list.clone().into_inner() {
            let body = item.clone().into();
            let response = self
                .send_image_queue_webhook_all::<FluxLoraImageGenerate, QueueResponse>(&body)
                .await?;
            item.fal_ai_request_id = Some(response.request_id);
        }
        Ok(list)
    }

    pub async fn send_image_queue_many_async(
        &self,
        list: ImageNewList,
    ) -> Result<ImageNewList, FalAiClientError> {
        let futures = list.into_inner().into_iter().map(|mut item| {
            let body = item.clone().into();
            let client = self.clone();
            async move {
                match client
                    .send_image_queue_webhook_all::<FluxLoraImageGenerate, QueueResponse>(&body)
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
        dbg!(&successful);
        Ok(ImageNewList::new(successful))
    }

    pub async fn send_training_queue_test(
        &self,
        prompt: &FluxLoraTrainingSchema,
    ) -> Result<FluxQueueResponse, FalAiClientError> {
        let response = self
            .client
            .post(&self.training_url)
            .header("Authorization", format!("Key {}", &self.fal_key))
            .header("Content-Type", "application/json")
            .json(&prompt)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to process training schema: {:?}", e);
                loco_rs::Error::Message("Error processing training model schema: 100".to_string())
            })?;

        let text = response.text().await.map_err(|e| {
            tracing::error!("Failed to read response body: {:?}", e);
            loco_rs::Error::Message("Error reading response body".to_string())
        })?;

        tracing::error!("Raw response from FAL AI API: {}", text); // 🔥 This logs what’s actually returned

        let parsed_response: FluxQueueResponse = serde_json::from_str(&text).map_err(|e| {
            tracing::error!("Failed to parse response body: {:?}", e);
            loco_rs::Error::Message("Error decoding response body".to_string())
        })?;
        // .json::<FluxQueueResponse>()
        // .await?;

        Ok(parsed_response)
    }

    // pub async fn send_image_queue_test(
    //     &self,
    //     prompt: &FluxLoraImageGenerate,
    // ) -> Result<FluxQueueResponse, FalAiClientError> {
    //     // dbg!("FluxLoraImageGenerate", &prompt);
    //     let response = self
    //         .client
    //         .post(&self.image_url)
    //         .header("Authorization", format!("Key {}", &self.fal_key))
    //         .header("Content-Type", "application/json")
    //         .json(&prompt)
    //         .send()
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to process training schema: {:?}", e);
    //             loco_rs::Error::Message("Error processing training model schema: 100".to_string())
    //         })?;

    //     dbg!("FluxQueueResponse", &response);
    //     let text = response.text().await.map_err(|e| {
    //         tracing::error!("Failed to read response body: {:?}", e);
    //         loco_rs::Error::Message("Error reading response body".to_string())
    //     })?;

    //     tracing::error!("Raw response from FAL AI API: {}", text); // 🔥 This logs what’s actually returned

    //     let parsed_response: FluxQueueResponse = serde_json::from_str(&text).map_err(|e| {
    //         tracing::error!("Failed to parse response body: {:?}", e);
    //         loco_rs::Error::Message("Error decoding response body".to_string())
    //     })?;
    //     // .json::<FluxQueueResponse>()
    //     // .await?;

    //     Ok(parsed_response)
    // }

    pub async fn request_status(
        &self,
        request_id: &str,
    ) -> Result<FluxStatusResponse, FalAiClientError> {
        let response = self
            .client
            .get(format!(
                "{}/requests/{}/status",
                &self.image_url, request_id
            ))
            .header("Authorization", format!("Key {}", &self.fal_key))
            .send()
            .await
            .map_err(|_| {
                loco_rs::Error::Message("Error processing Status Request: 102".to_string())
            })?
            .json::<FluxStatusResponse>()
            .await?;

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

    pub async fn retry(
        &self,
        mut response: ImageNewList,
        list_img: ImageNewList,
    ) -> Result<ImageNewList, FalAiClientError> {
        if response.as_ref().len() != list_img.as_ref().len() {
            let missing_images: Vec<ImageNew> = list_img
                .into_inner()
                .iter()
                .filter(|item| !response.as_ref().contains(item))
                .cloned()
                .collect();
            let missing_images = ImageNewList::new(missing_images);

            // second request
            let second_try = self.send_image_queue_many(missing_images).await?;

            // Extend response manually
            let mut response_inner = response.into_inner();
            response_inner.extend(second_try.into_inner());
            response = ImageNewList::new(response_inner);
        }

        Ok(response)
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
    pub content_type: String,
    pub file_name: String,
    pub file_size: u64,
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
    pub content_type: String,
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
    pub fn successful_img(&self) -> SuccessfulPayload {
        let payload: SuccessfulPayload =
            serde_json::from_value(self.payload.clone().unwrap()).unwrap();
        payload
    }

    pub fn successful_training(&self) -> SuccessfulPayloadTraining {
        let payload: SuccessfulPayloadTraining =
            serde_json::from_value(self.payload.clone().unwrap()).unwrap();
        payload
    }

    pub fn successful_training_opt(&self) -> Option<String> {
        let payload: SuccessfulPayloadTraining = match self.payload.clone() {
            None => return None,
            Some(value) => serde_json::from_value(value).unwrap(),
        };
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

#[derive(Debug, Serialize, Deserialize)]
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
}
impl From<ImageNew> for FluxLoraImageGenerate {
    fn from(value: ImageNew) -> Self {
        Self {
            prompt: value.sys_prompt.into_inner(),
            image_size: value.image_size,
            num_inference_steps: value.num_inference_steps as u16,
            loras: value.loras,
            ..Default::default() // guidance_scale: 1.0,
                                 // num_images: 1,
                                 // output_format: ImageFormat::Jpeg,
                                 // enable_safety_checker: false,
        }
    }
}

impl Default for FluxLoraImageGenerate {
    fn default() -> Self {
        Self {
            prompt: "".to_string(),
            image_size: ImageSize::Square,
            num_inference_steps: 28,
            guidance_scale: 3.5,
            num_images: 1,
            enable_safety_checker: false,
            output_format: ImageFormat::Jpeg,
            photo_lora_scale: 1.0,
            loras: vec![],
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
}
impl FluxLoraTrainingSchema {
    pub fn from_training(value: TrainingModelModel, images_data_url: Url) -> Self {
        Self {
            images_data_url: images_data_url.into_inner(),
            trigger_word: value.trigger_word,
            steps: value.steps,
            create_mask: value.create_mask,
            is_style: value.is_style,
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

pub trait FluxExt {}
impl FluxExt for FluxLoraImageGenerate {}
impl FluxExt for FluxLoraTrainingSchema {}
