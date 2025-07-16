use bytes::Bytes;
use derive_more::Constructor;
use loco_rs::prelude::*;
use sea_orm::TransactionTrait;
use std::process::Stdio;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::service::fal_ai::fal_client::FalAiClientError;
use crate::{
    domain::{
        dal::load_user_credits_by_user_id, domain_services::image_generation::MediaGenerationError,
    },
    models::{
        images::UserPrompt, UserCreditModel, UserModel,
        _entities::sea_orm_active_enums::AspectRatio, videos::VideoNew, VideoModel,
    },
    service::fal_ai::fal_client::{FalAiClient, FalVideoSend, QueueResponse, WebhookPayload},
};

#[derive(Debug, Error)]
pub enum VideoProcessingError {
    #[error("FFmpeg library error: {0}")]
    FfmpegError(String),
    #[error("Task panicked or was cancelled")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("The provided video file does not contain a video stream")]
    VideoStreamNotFound,
    #[error("Failed to find a suitable frame in the video")]
    FrameNotFound,
    #[error("Failed to media from Fal API: {0}")]
    FalAiClientErr(#[from] FalAiClientError),
    #[error("Storage error: {0}")]
    StorageError(#[from] loco_rs::storage::StorageError),
}

#[derive(Debug, Clone, Constructor)]
pub struct VideoAndImageBytes {
    pub video_bytes: Bytes,
    pub image_bytes: Vec<u8>,
}
pub trait VideoGenerationTrait {
    fn formatted_prompt(&self) -> UserPrompt;
    fn aspect_ratio(&self) -> AspectRatio;
    fn cost(&self) -> i32;
    fn quality_model(&self) -> WebhookPayload;
    fn process(self, user: &UserModel) -> VideoNew;
}

pub struct VideoGenerationService;

impl VideoGenerationService {
    pub async fn generate(
        ctx: &AppContext,
        fal_ai_client: &FalAiClient,
        request: impl VideoGenerationTrait,
        user: &UserModel,
    ) -> Result<(UserCreditModel, VideoModel), MediaGenerationError> {
        // Prepare Video request
        let video_request = request.process(user);

        // Cost and Video amount from request
        let txn = ctx.db.begin().await?;
        let user_credits = load_user_credits_by_user_id(&txn, user.id).await?;
        if user_credits.credit_amount < video_request.cost() {
            txn.rollback().await?;
            return Err(MediaGenerationError::InsufficientCredits);
        }

        // Retry sending to Fal AI
        let fal_video_send: FalVideoSend = video_request.clone().into();
        let fal_response = fal_ai_client
            .send_queue_webhook_with_retries::<FalVideoSend, QueueResponse>(&fal_video_send, 3)
            .await?;

        // Update user's credits in DB
        let video = video_request.save(&txn, &fal_response).await?;
        let updated_user_credits = user_credits.deduct_credits(&txn, &video).await?;

        txn.commit().await?;
        Ok((updated_user_credits, video))
    }
    pub async fn test(
        ctx: &AppContext,
        request: impl VideoGenerationTrait,
        user: &UserModel,
    ) -> Result<(UserCreditModel, VideoModel), MediaGenerationError> {
        // Prepare Video request
        let video_request = request.process(user);

        // Cost and Video amount from request
        let txn = ctx.db.begin().await?;
        let user_credits = load_user_credits_by_user_id(&txn, user.id).await?;

        // Retry sending to Fal AI
        let fal_response = QueueResponse::test();

        // Update user's credits in DB
        let video = video_request.save(&txn, &fal_response).await?;

        txn.commit().await?;
        Ok((user_credits, video))
    }

    pub async fn download_video(
        ctx: &AppContext,
        video_model: &VideoModel,
    ) -> Result<Bytes, FalAiClientError> {
        let url = video_model
            .video_url_fal
            .clone()
            .ok_or_else(|| FalAiClientError::RequestFailed("Video url not found".into()))?;
        let client = reqwest::Client::new();
        let video_bytes = client.get(&url).send().await?.bytes().await?;
        let str = video_model.storage_key();
        let path = std::path::Path::new(&str);
        ctx.storage.upload(&path, &video_bytes).await?;
        Ok(video_bytes)
    }

    pub async fn process_video_in_memory(
        ctx: &AppContext,
        video_model: &VideoModel,
    ) -> Result<VideoAndImageBytes, VideoProcessingError> {
        // 1. Download video directly to memory
        let video_bytes = Self::download_video(&ctx, &video_model).await?;

        // 2. Extract thumbnail
        let image_thumb = Self::extract_thumbnail_local_v2(video_model).await?;

        // 3. Return video and thumbnail
        let video_and_image = VideoAndImageBytes::new(video_bytes.into(), image_thumb);
        Ok(video_and_image)
    }

    async fn extract_thumbnail_local_v2(
        video_model: &VideoModel,
    ) -> Result<Vec<u8>, VideoProcessingError> {
        let path = format!("storage/{}", video_model.storage_key());
        let child = Command::new("ffmpeg")
            .args([
                "-i",
                &path,
                "-ss",
                "00:00:01.000",
                "-frames:v",
                "1",
                "-f",
                "mjpeg",
                "-v",
                "error",
                "pipe:1",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                VideoProcessingError::FfmpegError(format!("Failed to spawn ffmpeg: {}", e))
            })?;

        let output_result = child.wait_with_output().await;
        let output = output_result.map_err(|e| {
            VideoProcessingError::FfmpegError(format!("Failed to wait for ffmpeg: {}", e))
        })?;

        // Check FFmpeg's exit status
        if !output.status.success() {
            let stderr_string = String::from_utf8_lossy(&output.stderr);
            if !stderr_string.trim().is_empty() {
                return Err(VideoProcessingError::FfmpegError(format!(
                    "ffmpeg failed with status {}. Stderr: {}",
                    output.status, stderr_string
                )));
            } else {
                if output.stdout.is_empty() {
                    return Err(VideoProcessingError::FfmpegError(format!(
                    "ffmpeg exited with status {} and produced no output. This may indicate a pipe or memory issue.",
                    output.status
                )));
                }
            }
        }

        // If we have no thumbnail, it's an error, even if the exit code was 0.
        if output.stdout.is_empty() {
            let stderr_string = String::from_utf8_lossy(&output.stderr);
            return Err(VideoProcessingError::FfmpegError(format!(
                "ffmpeg succeeded but produced no thumbnail. Stderr: {}",
                stderr_string
            )));
        }

        Ok(output.stdout)
    }

    pub async fn extract_thumbnail_local(
        ctx: &AppContext,
        video_model: &VideoModel,
    ) -> Result<Vec<u8>, VideoProcessingError> {
        let path_str = video_model.storage_key();
        let path = format!("storage/{}", &path_str);
        let mut child = Command::new("ffmpeg")
            .args([
                "-i",
                &path,
                "-ss",
                "00:00:01.000",
                "-frames:v",
                "1",
                "-f",
                "mjpeg",
                "-v",
                "error",
                "pipe:1",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                VideoProcessingError::FfmpegError(format!("Failed to spawn ffmpeg: {}", e))
            })?;

        let stdin = child.stdin.take();

        let path = std::path::Path::new(&path_str);
        let video_file = ctx.storage.download::<Vec<u8>>(path).await?;

        let (write_result, output_result) = tokio::join!(
            // Future 1: Write all data to stdin
            async move {
                if let Some(mut stdin) = stdin {
                    if let Err(e) = stdin.write_all(&video_file).await {
                        // We only care about errors that are NOT BrokenPipe.
                        if e.kind() != std::io::ErrorKind::BrokenPipe {
                            return Err(e);
                        }
                    }
                    // The write is done or was gracefully interrupted.
                    Ok(())
                } else {
                    // This case should be rare, but we handle it.
                    Ok(())
                }
            },
            // Future 2: Wait for the process to finish and collect output
            child.wait_with_output()
        );

        // --- Step 2: Check for errors in both futures ---
        write_result.map_err(|e| {
            VideoProcessingError::FfmpegError(format!("Fatal stdin write error: {}", e))
        })?;

        // Check if waiting for the process failed
        let output = output_result.map_err(|e| {
            VideoProcessingError::FfmpegError(format!("Failed to wait for ffmpeg: {}", e))
        })?;

        // --- Step 3: Check FFmpeg's exit status ---
        if !output.status.success() {
            let stderr_string = String::from_utf8_lossy(&output.stderr);
            // This check is important. If stderr is empty and we got a weird exit code,
            // it's likely a pipe issue we didn't handle. But if there's stderr, that's the real error.
            if !stderr_string.trim().is_empty() {
                return Err(VideoProcessingError::FfmpegError(format!(
                    "ffmpeg failed with status {}. Stderr: {}",
                    output.status, stderr_string
                )));
            } else {
                // If stderr is empty, the exit code is likely from a signal like SIGPIPE.
                // We can consider this a success if we got output, or a failure if not.
                if output.stdout.is_empty() {
                    return Err(VideoProcessingError::FfmpegError(format!(
                    "ffmpeg exited with status {} and produced no output. This may indicate a pipe or memory issue.",
                    output.status
                )));
                }
            }
        }

        // If we have no thumbnail, it's an error, even if the exit code was 0.
        if output.stdout.is_empty() {
            let stderr_string = String::from_utf8_lossy(&output.stderr);
            return Err(VideoProcessingError::FfmpegError(format!(
                "ffmpeg succeeded but produced no thumbnail. Stderr: {}",
                stderr_string
            )));
        }

        Ok(output.stdout)
    }

    pub fn extract_thumbnail_in_memory(video_data: &[u8]) -> Result<Vec<u8>, VideoProcessingError> {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut child = Command::new("ffmpeg")
            .args([
                "-i",
                "pipe:0",
                "-ss",
                "00:00:01.000",
                "-frames:v",
                "1",
                "-f",
                "mjpeg",
                "-v",
                "error",
                "pipe:1",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                VideoProcessingError::FfmpegError(format!("Failed to spawn ffmpeg: {}", e))
            })?;

        // Write all video data to ffmpeg's stdin
        if let Some(stdin) = child.stdin.as_mut() {
            let _ = stdin
                .write_all(video_data)
                .map_err(|e| VideoProcessingError::FfmpegError(e.to_string()))?;
            // Important: close stdin so ffmpeg knows no more input is coming
        } else {
            return Err(VideoProcessingError::FfmpegError(
                "Failed to open ffmpeg stdin".to_string(),
            ));
        }

        // Wait for ffmpeg to exit and capture output
        let output = child
            .wait_with_output()
            .map_err(|e| VideoProcessingError::FfmpegError(e.to_string()))?;

        if !output.status.success() {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            return Err(VideoProcessingError::FfmpegError(format!(
                "ffmpeg failed with status {}. Stderr: {}",
                output.status, stderr_str
            )));
        }

        if output.stdout.is_empty() {
            return Err(VideoProcessingError::FfmpegError(
                "ffmpeg succeeded but produced no output".to_string(),
            ));
        }

        Ok(output.stdout)
    }
}
