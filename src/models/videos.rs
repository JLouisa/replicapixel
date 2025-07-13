use crate::{
    controllers::video::VIDEO_COST_PER_SECOND,
    domain::{domain_services::video_generation::VideoGenerationTrait, url::Url},
    models::{
        _entities::{
            sea_orm_active_enums::{AspectRatio, Status},
            videos,
        },
        images::{AltText, SysPrompt, UserPrompt},
        user_credits::CostCreditsTrait,
        UserModel,
    },
    service::{
        aws::s3::{AwsS3, S3Key},
        fal_ai::fal_client::{DurationSeconds, FalAiVideoModel, QueueResponse, WebhookPayload},
        redis::redis::{RedisCacheDriver, RedisKey},
    },
    views::videos::{VideoView, VideoViewList},
};

pub use super::_entities::videos::{ActiveModel, Entity, Model};
use derive_more::{AsRef, Constructor};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{entity::prelude::*, ActiveValue, Condition, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use validator::Validate;
pub type Videos = Entity;
use futures::future::join_all;
use itertools::izip;
use std::collections::HashMap;
use tokio::join;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl CostCreditsTrait for Model {
    fn cost(&self) -> i32 {
        self.video_cost
    }
}

impl CostCreditsTrait for VideoNew {
    fn cost(&self) -> i32 {
        self.video_cost
    }
}

impl VideoGenerationTrait for VideoGenRequestParams {
    fn cost(&self) -> i32 {
        self.duration.to_int() * VIDEO_COST_PER_SECOND
    }
    fn formatted_prompt(&self) -> UserPrompt {
        UserPrompt::new(String::from(self.prompt.to_owned()))
    }
    fn aspect_ratio(&self) -> AspectRatio {
        self.aspect_ratio.to_owned()
    }
    fn quality_model(&self) -> WebhookPayload {
        WebhookPayload::Video(FalAiVideoModel::Veo3)
    }
    fn process(self, user: &UserModel) -> VideoNew {
        VideoNew {
            pid: self.pid,
            user_id: user.id,
            title: self.name.to_owned(),
            user_prompt: self.formatted_prompt(),
            sys_prompt: SysPrompt::new(self.formatted_prompt().as_ref()),
            negative_prompt: NegativePrompt::new(self.negative_prompt.to_owned()),
            alt: AltText::new(self.name.to_owned()),
            aspect_ratio: self.aspect_ratio.clone(),
            duration: self.duration.clone(),
            enhance_prompt: self.enhance_prompt,
            generate_audio: self.generate_audio,
            seed: None,
            video_cost: self.cost(),
            status: Status::Pending,
            is_favorite: false,
            video_s3_key: AwsS3::init_video_s3_key(&user.pid, &self.pid),
            thumbnail_s3_key: AwsS3::init_thumbnail_s3_key(&user.pid, &self.pid),
            video_url_fal: None,
            model: self.quality_model(),
        }
    }
}

impl Model {
    // async fn resolve_video_url(&self, driver: &RedisCacheDriver, aws: &AwsS3) -> Option<String> {
    //     match driver.get::<String>(&self.redis_key()).await {
    //         Ok(Some(url)) => Some(url),
    //         Ok(None) => match aws.get_video_pre(&self).await {
    //             Ok(url) => {
    //                 let url_str = url.to_string();
    //                 if let Err(err) = driver
    //                     .set::<String>(&self.redis_key(), &url_str, None)
    //                     .await
    //                 {
    //                     tracing::warn!(error = ?err, "Failed to cache pre-signed video URL in Redis");
    //                 }
    //                 Some(url_str)
    //             }
    //             Err(err) => {
    //                 tracing::error!(error = ?err, "Failed to get S3 pre-signed URL");
    //                 self.video_url_fal.to_owned()
    //             }
    //         },
    //         Err(err) => {
    //             tracing::error!(error = ?err, "Failed to read from Redis");
    //             self.video_url_fal.to_owned()
    //         }
    //     }
    // }
    // async fn resolve_thumbnail_url(
    //     &self,
    //     driver: &RedisCacheDriver,
    //     aws: &AwsS3,
    // ) -> Option<String> {
    //     match driver.get::<String>(&self.redis_key()).await {
    //         Ok(Some(url)) => Some(url),
    //         Ok(None) => match aws.get_video_pre(&self).await {
    //             Ok(url) => {
    //                 let url_str = url.to_string();
    //                 if let Err(err) = driver
    //                     .set::<String>(&self.redis_key(), &url_str, None)
    //                     .await
    //                 {
    //                     tracing::warn!(error = ?err, "Failed to cache pre-signed video URL in Redis");
    //                 }
    //                 Some(url_str)
    //             }
    //             Err(err) => {
    //                 tracing::error!(error = ?err, "Failed to get S3 pre-signed URL");
    //                 self.video_url_fal.to_owned()
    //             }
    //         },
    //         Err(err) => {
    //             tracing::error!(error = ?err, "Failed to read from Redis");
    //             self.video_url_fal.to_owned()
    //         }
    //     }
    // }
    pub fn parse_duration(&self) -> String {
        if self.duration < 10 {
            format!("0:0{}", self.duration)
        } else {
            format!("0:{}", self.duration)
        }
    }
    async fn resolve_all_url(&self, driver: &RedisCacheDriver, aws: &AwsS3) -> Vec<Option<String>> {
        let redis_result = match driver.mget(&self.redis_key_all()).await {
            Ok(values) => values,
            Err(err) => {
                tracing::error!(error = ?err, "Failed to MGET from Redis");
                vec![None; 2]
            }
        };

        let mut list: Vec<Option<String>> = Vec::with_capacity(2);
        if redis_result[0].is_none() {
            match aws.get_video_pre(&self).await {
                Ok(url) => {
                    let url_str = url.to_string();
                    if let Err(err) = driver
                        .set::<String>(&self.redis_key(), &url_str, None)
                        .await
                    {
                        tracing::warn!(error = ?err, "Failed to cache pre-signed video URL in Redis");
                    }
                    list.push(Some(url_str));
                }
                Err(err) => {
                    tracing::error!(error = ?err, "Failed to get S3 pre-signed URL");
                    list.push(self.video_url_fal.to_owned());
                }
            }
        }

        if redis_result[1].is_none() {
            match aws.get_thumbnail_pre(&self).await {
                Ok(url) => {
                    let url_str = url.to_string();
                    if let Err(err) = driver
                        .set::<String>(&self.redis_key(), &url_str, None)
                        .await
                    {
                        tracing::warn!(error = ?err, "Failed to cache pre-signed video URL in Redis");
                    }
                    list.push(Some(url_str));
                }
                Err(err) => {
                    tracing::error!(error = ?err, "Failed to get S3 pre-signed URL");
                    list.push(None);
                }
            }
        }

        list
    }

    pub async fn into_view(self, driver: &RedisCacheDriver, aws: &AwsS3) -> VideoView {
        let pre = self.resolve_all_url(driver, aws).await;
        let video_url = pre[0].clone();
        let thumbnail_url = pre[1].clone();

        VideoView::new(
            self.pid,
            self.title,
            UserPrompt::new(self.user_prompt),
            NegativePrompt::new(self.negative_prompt),
            AltText::new(self.alt),
            self.duration.to_string(),
            self.generate_audio,
            self.status,
            self.aspect_ratio,
            self.is_favorite,
            self.deleted_at.is_some(),
            self.created_at,
            video_url,
            thumbnail_url,
            None,
        )
    }
}

#[derive(Clone, Debug, Constructor, Default, AsRef)]
pub struct VideoModelList(Vec<Model>);
impl VideoModelList {
    pub fn into_inner(self) -> Vec<Model> {
        self.0
    }
    pub async fn into_view(self, driver: &RedisCacheDriver, aws: &AwsS3) -> VideoViewList {
        if self.as_ref().is_empty() {
            return VideoViewList::empty();
        }
        self.get_video_urls_perfected(driver, aws).await
    }

    pub async fn get_video_urls_perfected(
        self,
        driver: &RedisCacheDriver,
        aws: &AwsS3,
    ) -> VideoViewList {
        let mut completed_models = Vec::new();
        let mut processing_futures = Vec::new();
        let mut other_views = Vec::new(); // Store views directly

        // 1. Partition models, creating futures for work that can start immediately.
        for model in self.into_inner() {
            match model.status {
                Status::Processing => {
                    processing_futures.push(async move { aws.video_save_pre_url(model).await });
                }
                Status::Completed => {
                    completed_models.push(model);
                }
                _ => {
                    other_views.push(VideoView::from(model));
                }
            }
        }

        // 2. Create a single future that represents ALL work for completed models.
        let completed_future = Self::process_completed_models(completed_models, driver, aws);

        // Run all futures in parallel
        let (completed_results, processing_results) =
            join!(completed_future, join_all(processing_futures));

        // 4. Combine and sort results.
        let mut all_results = Vec::with_capacity(
            completed_results.len() + processing_results.len() + other_views.len(),
        );
        all_results.extend(completed_results);
        all_results.extend(processing_results);
        all_results.extend(other_views);

        all_results.sort_by_key(|v| v.created_at);

        VideoViewList::new(all_results)
    }

    async fn process_completed_models(
        models: Vec<Model>,
        driver: &RedisCacheDriver,
        aws: &AwsS3,
    ) -> Vec<VideoView> {
        if models.is_empty() {
            return Vec::new();
        }

        // The `.await` for MGET is now inside this self-contained future.
        let main_keys: Vec<RedisKey> = models.iter().map(|m| m.redis_key()).collect();
        let thumb_keys: Vec<RedisKey> = models.iter().map(|m| m.redis_thumbnail_key()).collect();
        let all_keys: Vec<RedisKey> = main_keys.into_iter().chain(thumb_keys).collect();

        let all_redis_values: Vec<Option<String>> =
            driver.mget(&all_keys).await.unwrap_or_else(|err| {
                tracing::error!(error = ?err, "Failed to MGET from Redis");
                vec![None; all_keys.len()]
            });

        // Use `split_at` for a more efficient and concise split.
        let (main_values, thumb_values) = all_redis_values.split_at(models.len());

        // Create a future for each model.
        let model_futures = izip!(
            models.into_iter(),          // Move models into the iterator
            main_values.iter().cloned(), // .cloned() is fine here
            thumb_values.iter().cloned()
        )
        .map(|(model, main_val, thumb_val)| async move {
            let main_url_future = Self::get_final_main_url(&model, main_val, driver, aws);
            let thumb_url_future = Self::get_final_thumb_url(&model, thumb_val, driver, aws);

            let (main_url_result, thumb_url_result) = join!(main_url_future, thumb_url_future);

            VideoView::from(model)
                .set_video_url(main_url_result)
                .set_thumbnail_url(thumb_url_result)
        });

        join_all(model_futures).await
    }

    // pub async fn get_video_urls_v3(self, driver: &RedisCacheDriver, aws: &AwsS3) -> VideoViewList {
    //     let mut completed_models = Vec::new();
    //     let mut processing_futures = Vec::new();
    //     let mut other = Vec::new();

    //     for model in self.into_inner() {
    //         match model.status {
    //             Status::Processing => {
    //                 processing_futures.push(async move { aws.video_save_pre_url(model).await });
    //             }
    //             Status::Completed => {
    //                 completed_models.push(model);
    //             }
    //             _ => {
    //                 other.push(VideoView::from(model));
    //             }
    //         }
    //     }

    //     let completed_futures = if !completed_models.is_empty() {
    //         let main_keys: Vec<RedisKey> = completed_models.iter().map(|m| m.redis_key()).collect();
    //         let thumb_keys: Vec<RedisKey> = completed_models
    //             .iter()
    //             .map(|m| m.redis_thumbnail_key())
    //             .collect();
    //         let all_keys: Vec<RedisKey> = main_keys.into_iter().chain(thumb_keys).collect();

    //         let all_redis_values: Vec<Option<String>> =
    //             driver.mget(&all_keys).await.unwrap_or_else(|err| {
    //                 tracing::error!(error = ?err, "Failed to MGET from Redis");
    //                 vec![None; all_keys.len()]
    //             });
    //         let main_values: Vec<Option<String>> = all_redis_values
    //             .iter()
    //             .take(completed_models.len())
    //             .cloned()
    //             .collect();

    //         let thumb_values: Vec<Option<String>> = all_redis_values
    //             .into_iter()
    //             .skip(completed_models.len())
    //             .collect();

    //         izip!(completed_models, main_values, thumb_values)
    //             .map(|(model, main_val, thumb_val)| async move {
    //                 let main_url_future =
    //                     Self::get_final_main_url(&model, main_val.clone(), driver, aws);
    //                 let thumb_url_future =
    //                     Self::get_final_thumb_url(&model, thumb_val.clone(), driver, aws);

    //                 let (main_url_result, thumb_url_result) =
    //                     join!(main_url_future, thumb_url_future);

    //                 VideoView::from(model)
    //                     .set_video_url(main_url_result)
    //                     .set_thumbnail_url(thumb_url_result)
    //             })
    //             .collect::<Vec<_>>()
    //     } else {
    //         Vec::new()
    //     };

    //     // Run all futures in parallel
    //     let (completed_results, processing_results) =
    //         join!(join_all(completed_futures), join_all(processing_futures));

    //     // Combine all
    //     let mut all_results = Vec::new();
    //     all_results.extend(completed_results);
    //     all_results.extend(processing_results);
    //     all_results.extend(other);

    //     // Sort by created_at
    //     all_results.sort_by_key(|v| v.created_at);

    //     VideoViewList::new(all_results)
    // }

    async fn get_final_main_url(
        model: &Model,
        cached_url: Option<String>,
        driver: &RedisCacheDriver,
        aws: &AwsS3,
    ) -> Option<String> {
        if let Some(url) = cached_url {
            return Some(url); // Cache Hit
        }
        // Cache Miss: fetch from S3
        match aws.get_video_pre(model).await {
            Ok(url) => {
                let url_str = url.to_string();
                // Spawn caching as a background task; don't make the user wait for it.
                let key = model.redis_key();
                let driver_clone = driver.clone(); // Assuming driver can be cloned
                tokio::spawn(async move {
                    if let Err(e) = driver_clone.set_s3_video_pre_url(&key, url.as_ref()).await {
                        tracing::warn!(error = ?e, "Failed to cache main URL");
                    }
                });
                Some(url_str)
            }
            Err(err) => {
                tracing::error!(error = ?err, "Failed to get S3 pre-signed URL for video");
                model.video_url_fal.clone()
            }
        }
    }

    async fn get_final_thumb_url(
        model: &Model,
        cached_url: Option<String>,
        driver: &RedisCacheDriver,
        aws: &AwsS3,
    ) -> Option<String> {
        if let Some(url) = cached_url {
            return Some(url); // Cache Hit
        }
        // Cache Miss: fetch from S3
        match aws.get_thumbnail_pre(model).await {
            Ok(url) => {
                let url_str = url.to_string();
                let key = model.redis_thumbnail_key();
                let driver_clone = driver.clone();
                tokio::spawn(async move {
                    if let Err(e) = driver_clone.set_s3_video_pre_url(&key, url.as_ref()).await {
                        tracing::warn!(error = ?e, "Failed to cache thumbnail URL");
                    }
                });
                Some(url_str)
            }
            Err(err) => {
                tracing::error!(error = ?err, "Failed to get S3 pre-signed URL for thumbnail");
                None
            }
        }
    }

    // pub async fn get_video_urls_v2(self, driver: &RedisCacheDriver, aws: &AwsS3) -> VideoViewList {
    //     #[derive(Debug, Clone)]
    //     pub struct VideoUrlResult {
    //         pub main_url: Option<String>,
    //         pub thumbnail_url: Option<String>,
    //     }

    //     let models: Vec<&Model> = self
    //         .as_ref()
    //         .iter()
    //         .filter(|f| f.status == Status::Completed)
    //         .collect();

    //     let main_keys: Vec<RedisKey> = models.iter().map(|m| m.redis_key()).collect();
    //     let thumb_keys: Vec<RedisKey> = models.iter().map(|m| m.redis_thumbnail_key()).collect();
    //     let all_keys: Vec<RedisKey> = main_keys.into_iter().chain(thumb_keys).collect();

    //     let all_redis_values: Vec<Option<String>> = match driver.mget(&all_keys).await {
    //         Ok(values) => values,
    //         Err(err) => {
    //             tracing::error!(error = ?err, "Failed to MGET from Redis");
    //             vec![None; all_keys.len()]
    //         }
    //     };
    //     let (main_values, thumb_values) = all_redis_values.split_at(models.len());

    //     let mut results: HashMap<Uuid, VideoUrlResult> = HashMap::with_capacity(models.len());

    //     for (model, main_url_opt, thumb_url_opt) in izip!(models, main_values, thumb_values) {
    //         let final_thumb_url = match thumb_url_opt {
    //             Some(url) => Some(url.clone()), // Cache hit
    //             None => {
    //                 // Cache miss, fetch from AWS
    //                 match aws.get_thumbnail_pre(model).await {
    //                     Ok(url) => {
    //                         if let Err(err) = driver
    //                             .set::<String>(
    //                                 &&model.redis_thumbnail_key(),
    //                                 &url.to_string(),
    //                                 None,
    //                             )
    //                             .await
    //                         {
    //                             tracing::warn!(error = ?err, "Failed to cache pre-signed URL");
    //                         }
    //                         Some(url.to_string())
    //                     }
    //                     Err(err) => {
    //                         tracing::error!(error = ?err, "Failed to get S3 pre-signed URL");
    //                         None
    //                     }
    //                 }
    //             }
    //         };

    //         // Next, handle the main video URL, with fallback logic
    //         let final_main_url = match main_url_opt {
    //             Some(url) => Some(url.clone()), // Cache hit
    //             None => {
    //                 // Cache miss, fetch from AWS
    //                 match aws.get_video_pre(model).await {
    //                     Ok(url) => {
    //                         // Attempt to cache the new URL, but don't fail if caching does
    //                         if let Err(err) = driver
    //                             .set::<String>(&model.redis_key(), &url.to_string(), None)
    //                             .await
    //                         {
    //                             tracing::warn!(error = ?err, "Failed to cache pre-signed URL");
    //                         }
    //                         Some(url.to_string())
    //                     }
    //                     Err(err) => {
    //                         tracing::error!(error = ?err, "Failed to get S3 pre-signed URL");
    //                         // Use the final fallback URL from the model
    //                         model.video_url_fal.clone()
    //                     }
    //                 }
    //             }
    //         };

    //         results.insert(
    //             model.pid,
    //             VideoUrlResult {
    //                 main_url: final_main_url,
    //                 thumbnail_url: final_thumb_url,
    //             },
    //         );
    //     }

    //     let video_list = self.into_inner().into_iter().map(async |m| {
    //         if m.status == Status::Processing {
    //             let view = aws.video_save_pre_url(m).await;
    //             view
    //         } else {
    //             let url = results.get(&m.pid).unwrap();
    //             let video_url = match url.main_url.clone() {
    //                 Some(url) => Some(url),
    //                 None => m.video_url_fal.clone(),
    //             };
    //             let thumbnail_url = url.thumbnail_url.clone();
    //             VideoView::from(m)
    //                 .set_video_url(video_url)
    //                 .set_thumbnail_url(thumbnail_url)
    //         }
    //     });

    //     let video_list: Vec<VideoView> = join_all(video_list).await;
    //     VideoViewList::new(video_list)
    // }
    // pub async fn resolve_video_urls(self, driver: &RedisCacheDriver, aws: &AwsS3) -> VideoViewList {
    //     let models: Vec<&Model> = self
    //         .as_ref()
    //         .iter()
    //         .filter(|f| f.status == Status::Completed)
    //         .collect();
    //     let keys: Vec<RedisKey> = models.iter().map(|m| m.redis_key()).collect();
    //     let thumb_keys: Vec<RedisKey> = models.iter().map(|m| m.redis_thumbnail_key()).collect();

    //     let redis_results = match driver.mget(&keys).await {
    //         Ok(values) => values,
    //         Err(err) => {
    //             tracing::error!(error = ?err, "Failed to MGET from Redis");
    //             vec![None; models.len()]
    //         }
    //     };
    //     let redis_results_thumb = match driver.mget(&thumb_keys).await {
    //         Ok(values) => values,
    //         Err(err) => {
    //             tracing::error!(error = ?err, "Failed to MGET from Redis");
    //             vec![None; models.len()]
    //         }
    //     };

    //     let mut results: HashMap<Uuid, Option<String>> = HashMap::new();
    //     let mut missing_models = Vec::new();

    //     for (model, redis_value) in models.iter().zip(redis_results.iter()) {
    //         match redis_value {
    //             Some(url) => {
    //                 results.insert(model.pid, Some(url.clone()));
    //             }
    //             None => {
    //                 missing_models.push(model);
    //             }
    //         }
    //     }

    //     for model in missing_models {
    //         match aws.get_video_pre(&model).await {
    //             Ok(url) => {
    //                 if let Err(err) = driver.set::<Url>(&model.redis_key(), &url, None).await {
    //                     tracing::warn!(error = ?err, "Failed to cache pre-signed URL");
    //                 }
    //                 results.insert(model.pid, Some(url.to_string()));
    //             }
    //             Err(err) => {
    //                 tracing::error!(error = ?err, "Failed to get S3 pre-signed URL");
    //                 results.insert(model.pid, model.video_url_fal.clone());
    //             }
    //         }
    //     }

    //     let video_list = self.into_inner().into_iter().map(async |m| {
    //         if m.status == Status::Processing {
    //             let view = aws.video_save_pre_url(m).await;
    //             view
    //         } else {
    //             let url = results
    //                 .get(&m.pid)
    //                 .and_then(|v| v.clone())
    //                 .or_else(|| m.video_url_fal.clone());
    //             VideoView::from(m).set_video_url(url)
    //         }
    //     });

    //     let video_list: Vec<VideoView> = join_all(video_list).await;
    //     VideoViewList::new(video_list)
    // }
}

#[derive(Clone, Debug, Serialize, Deserialize, Constructor, AsRef)]
pub struct NegativePrompt(Option<String>);
impl NegativePrompt {
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

#[derive(Clone, Validate, Debug, Deserialize)]
pub struct VideoGenRequestParams {
    #[serde(default = "Uuid::new_v4", skip_deserializing)]
    pub pid: Uuid,
    pub name: String,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub aspect_ratio: AspectRatio,
    pub duration: DurationSeconds,
    pub enhance_prompt: bool,
    pub generate_audio: bool,
}

#[derive(Clone, Validate, Serialize, Debug)]
pub struct VideoNew {
    pub pid: Uuid,
    pub user_id: i32,
    pub title: String,
    pub user_prompt: UserPrompt,
    pub sys_prompt: SysPrompt,
    pub negative_prompt: NegativePrompt,
    pub alt: AltText,
    pub aspect_ratio: AspectRatio,
    pub duration: DurationSeconds,
    pub enhance_prompt: bool,
    pub generate_audio: bool,
    pub seed: Option<i32>,
    pub video_cost: i32,
    pub status: Status,
    pub is_favorite: bool,
    pub video_url_fal: Option<String>,
    pub video_s3_key: S3Key,
    pub thumbnail_s3_key: S3Key,
    pub model: WebhookPayload,
}
impl VideoNew {
    pub fn cost(&self) -> i32 {
        self.video_cost
    }
    pub async fn save(
        self,
        db: &impl ConnectionTrait,
        response: &QueueResponse,
    ) -> ModelResult<Model> {
        let item = ActiveModel {
            pid: ActiveValue::Set(self.pid.clone()),
            user_id: ActiveValue::Set(self.user_id),
            title: ActiveValue::Set(self.title.clone()),
            user_prompt: ActiveValue::Set(self.user_prompt.into_inner()),
            sys_prompt: ActiveValue::Set(self.sys_prompt.into_inner()),
            negative_prompt: ActiveValue::Set(self.negative_prompt.into_inner()),
            alt: ActiveValue::Set(self.alt.into_inner()),
            duration: ActiveValue::Set(self.duration.to_int()),
            enhance_prompt: ActiveValue::Set(self.enhance_prompt),
            generate_audio: ActiveValue::Set(self.generate_audio),
            seed: ActiveValue::Set(self.seed),
            status: ActiveValue::Set(self.status),
            aspect_ratio: ActiveValue::Set(self.aspect_ratio),
            video_cost: ActiveValue::Set(self.video_cost),
            fal_ai_request_id: ActiveValue::Set(Some(response.request_id.clone())),
            video_s3_key: ActiveValue::Set(self.video_s3_key.into_inner()),
            thumbnail_s3_key: ActiveValue::Set(self.thumbnail_s3_key.into_inner()),
            video_url_fal: ActiveValue::Set(self.video_url_fal),
            is_favorite: ActiveValue::Set(self.is_favorite),
            ..Default::default()
        };
        let item = item.insert(db).await?;
        Ok(item)
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Self> {
        let condition = Condition::all().add(videos::Column::Pid.eq(pid));
        let video = Entity::find()
            .filter(condition)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound);
        video
    }
    pub async fn find_by_request_id(
        db: &DatabaseConnection,
        request_id: &str,
    ) -> ModelResult<Self> {
        let condition = Condition::all().add(videos::Column::FalAiRequestId.eq(request_id));
        let video = Entity::find()
            .filter(condition)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound);
        video
    }
    pub async fn update_fal_video_url_success(
        self,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        self.update_fal_video_url(db, None, Status::Completed).await
    }
    pub async fn update_fal_video_url_failed(
        self,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        self.update_fal_video_url(db, None, Status::Failed).await
    }
    pub async fn update_fal_video_url_processing(
        self,
        db: &impl ConnectionTrait,
        url: Option<String>,
    ) -> ModelResult<Model> {
        self.update_fal_video_url(db, url, Status::Processing).await
    }
    async fn update_fal_video_url(
        self,
        db: &impl ConnectionTrait,
        url: Option<String>,
        status: Status,
    ) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.video_url_fal = ActiveValue::set(url);
        new.status = ActiveValue::set(status);
        let video = new.update(db).await?;
        Ok(video)
    }
    pub async fn favorite_video_toggle(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let time = chrono::Utc::now().into();
        let new_bool = !self.is_favorite;
        let mut new = ActiveModel::from(self);
        new.is_favorite = ActiveValue::set(new_bool);
        new.updated_at = ActiveValue::set(time);
        let video = new.update(db).await?;
        Ok(video)
    }
    pub async fn delete_video(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let time = chrono::Utc::now().into();
        let mut new = ActiveModel::from(self);
        new.deleted_at = ActiveValue::set(Some(time));
        new.updated_at = ActiveValue::set(time);
        let video = new.update(db).await?;
        Ok(video)
    }
    pub async fn restore_video(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.updated_at = ActiveValue::set(chrono::Utc::now().into());
        new.deleted_at = ActiveValue::set(None);
        let video = new.update(db).await?;
        Ok(video)
    }
    pub async fn upload_s3_completed(self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        let mut new = ActiveModel::from(self);
        new.status = ActiveValue::set(Status::Completed);
        let video = new.update(db).await?;
        Ok(video)
    }
    pub async fn find_x_videos_by_user_id(
        db: &DatabaseConnection,
        id: i32,
        is_favorite: bool,
        is_deleted: bool,
        num: u64,
    ) -> ModelResult<Vec<Self>> {
        let mut condition = Condition::all().add(videos::Column::UserId.eq(id));

        if is_deleted {
            condition = condition.add(videos::Column::DeletedAt.is_not_null());
        } else {
            condition = condition.add(videos::Column::DeletedAt.is_null());
            if is_favorite {
                condition = condition.add(videos::Column::IsFavorite.eq(true));
            }
        }

        let order_column = if is_deleted {
            videos::Column::DeletedAt
        } else {
            videos::Column::UpdatedAt
        };

        let results = Entity::find()
            .filter(condition)
            .limit(num)
            .order_by_desc(order_column)
            .all(db)
            .await?;

        Ok(results)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn save(db: &DatabaseConnection, item: &Model) -> ModelResult<Self> {
        let item = ActiveModel {
            pid: ActiveValue::set(item.pid.clone()),
            title: ActiveValue::set(item.title.clone()),
            user_id: ActiveValue::set(item.user_id.clone()),
            user_prompt: ActiveValue::set(item.user_prompt.clone()),
            sys_prompt: ActiveValue::set(item.sys_prompt.clone()),
            negative_prompt: ActiveValue::set(item.negative_prompt.clone()),
            alt: ActiveValue::set(item.alt.clone()),
            duration: ActiveValue::set(item.duration),
            enhance_prompt: ActiveValue::set(item.enhance_prompt),
            generate_audio: ActiveValue::set(item.generate_audio),
            seed: ActiveValue::set(item.seed),
            status: ActiveValue::set(item.status),
            aspect_ratio: ActiveValue::set(item.aspect_ratio.clone()),
            video_cost: ActiveValue::set(item.video_cost),
            fal_ai_request_id: ActiveValue::set(item.fal_ai_request_id.clone()),
            is_favorite: ActiveValue::set(item.is_favorite),
            video_s3_key: ActiveValue::set(item.video_s3_key.clone()),
            video_url_fal: ActiveValue::set(item.video_url_fal.clone()),
            thumbnail_s3_key: ActiveValue::set(item.thumbnail_s3_key.clone()),
            ..Default::default()
        };

        let item = item.insert(db).await?;

        Ok(item.into())
    }
    pub async fn upload_s3_completed(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.status = ActiveValue::Set(Status::Completed);
        Ok(self.update(db).await?)
    }
}
// implement your custom finders, selectors oriented logic here
impl Entity {}
