use derive_more::Constructor;
use loco_rs::{prelude::*, storage::StorageError};
use serde::{Deserialize, Serialize};

use crate::{
    domain::domain_services::{
        image_generation::MediaGenerationError, video_generation::VideoGenerationService,
    },
    models::VideoModel,
};

pub struct DownloadWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize, Constructor)]
pub struct DownloadWorkerArgs {
    pub video_model_pid: Uuid,
}

#[async_trait]
impl BackgroundWorker<DownloadWorkerArgs> for DownloadWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: DownloadWorkerArgs) -> Result<()> {
        use tokio::try_join;
        // Fetch video model
        let video_model = VideoModel::find_by_pid(&self.ctx.db, args.video_model_pid).await?;

        // Generate video/image in memory
        let video_and_image =
            VideoGenerationService::process_video_in_memory(&self.ctx, &video_model)
                .await
                .map_err(MediaGenerationError::from)?;

        // Upload the video to S3 (store ID is 'aws')
        let aws = self
            .ctx
            .storage
            .as_store("aws")
            .ok_or_else(|| StorageError::StoreNotFound("aws store not found".to_string()))?;

        // Upload the video to S3
        let image = video_and_image.image_bytes.into();
        try_join!(
            aws.upload(
                std::path::Path::new(&video_model.video_s3_key),
                &video_and_image.video_bytes,
            ),
            aws.upload(std::path::Path::new(&video_model.thumbnail_s3_key), &image,),
        )?;

        // Mark the video model as completed
        let str = video_model
            .upload_s3_completed(&self.ctx.db)
            .await?
            .storage_key();

        // Delete the video from local storage
        let path = std::path::Path::new(&str);
        self.ctx.storage.delete(&path).await?;

        Ok(())
    }
}
