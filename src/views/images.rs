use loco_rs::prelude::*;

use derive_more::{AsRef, Constructor, From};
use futures::future::{join_all, try_join_all};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::website::Website;
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::images::{ImageNew, ImagesModelList};
use crate::models::{ImageModel, UserCreditModel};
use crate::models::{_entities::images, images::ImageNewList};
use crate::service::aws::s3::{AwsError, AwsS3, S3Key};
use crate::service::redis::redis::{RedisCacheDriver, RedisDbError};

pub fn img_infinite_loading(
    v: &impl ViewRenderer,
    website: &Website,
    images: &ImageViewList,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_partial.html",
        data!({"website": website, "images": images }),
    )
}

pub fn one(
    v: &impl ViewRenderer,
    website: &Website,
    credits: &CreditsViewModel,
    image_list: &ImageViewList,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_loading_partial.html",
        data!({ "website": website, "credits": credits, "images": image_list }),
    )
}

pub fn img_completed(
    v: &impl ViewRenderer,
    website: &Website,
    images: &ImageViewList,
    credits: &CreditsViewModel,
    is_image_gen: Option<bool>,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_partial.html",
        data!({ "website": website, "credits": credits, "images": images, "is_image_gen": is_image_gen }),
    )
}

/// When there is an issue with rendering the view.
pub fn favorite(v: &impl ViewRenderer, website: &Website, image: &ImageView) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/partials/favorite_button.html",
        data!({ "website": website, "image": image }),
    )
}

/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<images::Model>) -> Result<Response> {
    format::render().view(v, "images/list.html", data!({"items": items}))
}

/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &images::Model) -> Result<Response> {
    format::render().view(v, "images/show.html", data!({"item": item}))
}

/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "images/create.html", data!({}))
}

/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &images::Model) -> Result<Response> {
    format::render().view(v, "images/edit.html", data!({"item": item}))
}

// ============== View Models for the View Templates ==============
#[derive(Serialize)]
pub struct CreditsViewModel {
    pub credit_amount: i32,
}
impl From<UserCreditModel> for CreditsViewModel {
    fn from(credits: UserCreditModel) -> Self {
        Self {
            credit_amount: credits.credit_amount,
        }
    }
}
impl From<&UserCreditModel> for CreditsViewModel {
    fn from(credits: &UserCreditModel) -> Self {
        Self {
            credit_amount: credits.credit_amount,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageView {
    pub pid: Uuid,
    pub training_model_id: Option<i32>,
    pub user_prompt: String,
    pub image_size: String,
    pub image_s3_key: String,
    pub image_url_fal: Option<String>,
    pub content_type: String,
    pub image_alt: String,
    pub image_status: String,
    pub is_favorite: bool,
    pub pre_url: Option<String>,
    pub s3_pre_url: Option<String>,
}
impl ImageView {
    pub async fn set_pre_url(self, s3_client: &AwsS3) -> Result<Self, AwsError> {
        let pre_url = s3_client.auto_upload_img_presigned_url(&self).await?;
        let mut new_self = self;
        new_self.pre_url = Some(pre_url.into_inner());
        Ok(new_self)
    }
    pub async fn set_pre_url_mut(&mut self, s3_client: &AwsS3) -> Result<(), AwsError> {
        let pre_url = s3_client.auto_upload_img_presigned_url(self).await?;
        self.pre_url = Some(pre_url.into_inner());
        Ok(())
    }

    pub async fn set_pre_url_many(
        list: Vec<Self>,
        s3_client: &AwsS3,
    ) -> Result<Vec<Self>, AwsError> {
        let futures = list.into_iter().map(|image| async move {
            if image.image_url_fal.is_some() && image.image_status == Status::Processing.to_string()
            {
                image.set_pre_url(s3_client).await
            } else {
                Ok(image)
            }
        });
        try_join_all(futures).await
    }
    pub async fn get_pre_url(mut self, s3_client: &AwsS3, cache: &RedisCacheDriver) -> Self {
        // Early exit if there isn't an image URL or S3 key
        if self.image_url_fal.is_none() || self.image_s3_key.is_empty() {
            return self;
        }

        let s3_key = S3Key::new(self.image_s3_key.clone());

        // Try getting from cache
        match cache.get_s3_pre_url(&self).await {
            Ok(pre_url) => {
                tracing::info!("Using cached pre_url: {}", self.pid);
                self.s3_pre_url = Some(pre_url);
            }
            Err(_) => {
                // Fallback: generate new URL and set it in cache
                match s3_client.get_object_pre(&s3_key, None).await {
                    Ok(url) => {
                        tracing::info!("Generating new pre_url: {}", self.pid);
                        self.s3_pre_url = Some(url.into_inner());
                        let _ = cache.set_s3_pre_url(&self).await;
                    }
                    Err(err) => {
                        tracing::warn!("Failed to generate pre_url for {}: {:?}", self.pid, err);
                    }
                }
            }
        }

        self
    }

    pub async fn get_pre_url_many(
        list: Vec<Self>,
        s3_client: &AwsS3,
        cache: &RedisCacheDriver,
    ) -> Vec<Self> {
        let futures = list.into_iter().map(|image| async move {
            if image.image_url_fal.is_some() && image.image_status == Status::Completed.to_string()
            {
                image.get_pre_url(s3_client, cache).await
            } else {
                image
            }
        });

        join_all(futures).await
    }
}
impl ImageView {
    pub async fn get_pre_url_mut(
        &mut self,
        s3_client: &AwsS3,
        cache: &RedisCacheDriver,
    ) -> &mut Self {
        if self.image_url_fal.is_none() || self.image_s3_key.is_empty() {
            tracing::trace!(
                "Skipping pre_url for {}: missing url_fal or s3_key",
                self.pid
            );
            return self;
        }
        // Optional: Add status check if pre-signed URLs only make sense for completed images
        if self.image_status != Status::Completed.to_string() {
            tracing::trace!("Skipping pre_url for {}: status is not Completed", self.pid);
            return self;
        }

        let s3_key = S3Key::new(self.image_s3_key.clone());

        // --- Refined Cache Get Logic ---
        match cache.get_s3_pre_url(&self).await {
            Ok(pre_url) => {
                tracing::info!("Using cached pre_url for: {}", self.pid);
                self.s3_pre_url = Some(pre_url);
            }
            Err(RedisDbError::NotFound) => {
                tracing::info!("Cache miss for pre_url: {}. Generating new one.", self.pid);

                // Proceed to generate a new URL
                match s3_client.get_object_pre(&s3_key, None).await {
                    Ok(url) => {
                        tracing::debug!("Successfully generated new pre_url for: {}", self.pid);
                        let url_str = url.into_inner();
                        self.s3_pre_url = Some(url_str.clone());

                        // Attempt to cache the newly generated URL
                        match cache.set_s3_pre_url(&self).await {
                            Ok(()) => {
                                tracing::debug!("Successfully cached new pre_url for {}", self.pid);
                            }
                            Err(cache_set_err) => {
                                tracing::error!(
                                    "Failed to cache newly generated pre_url for {}: {:?}",
                                    self.pid,
                                    cache_set_err
                                );
                            }
                        }
                    }
                    Err(s3_err) => {
                        tracing::warn!("Failed to generate pre_url for {}: {:?}", self.pid, s3_err);
                    }
                }
            }
            // --- Other Cache Error (Connection, Protocol, Timeout, etc.) ---
            Err(other_cache_err) => {
                tracing::error!(
                    "Failed to retrieve pre_url from cache for {} due to cache error: {:?}. Skipping S3 generation for this request.",
                    self.pid,
                    other_cache_err
                );
            }
        }

        self // Return the (potentially) modified self
    }
}

impl From<ImageModel> for ImageView {
    fn from(img: ImageModel) -> Self {
        Self {
            pid: img.pid,
            training_model_id: img.training_model_id,
            user_prompt: img.user_prompt,
            image_size: img.image_size.to_string(),
            image_url_fal: img.image_url_fal,
            image_s3_key: img.image_s3_key,
            content_type: img.content_type.to_string(),
            image_alt: img.alt,
            image_status: img.status.to_string(),
            is_favorite: img.is_favorite,
            pre_url: None,
            s3_pre_url: None,
        }
    }
}
impl From<&ImageModel> for ImageView {
    fn from(img: &ImageModel) -> Self {
        Self {
            pid: img.pid.to_owned(),
            training_model_id: img.training_model_id,
            user_prompt: img.user_prompt.to_owned(),
            image_size: img.image_size.clone().to_string(),
            image_url_fal: img.image_url_fal.to_owned(),
            image_s3_key: img.image_s3_key.to_owned(),
            content_type: img.content_type.to_string(),
            image_alt: img.alt.to_owned(),
            image_status: img.status.to_string(),
            is_favorite: img.is_favorite,
            pre_url: None,
            s3_pre_url: None,
        }
    }
}
impl From<ImageNew> for ImageView {
    fn from(img: ImageNew) -> Self {
        Self {
            pid: img.pid,
            training_model_id: img.training_model_id,
            user_prompt: img.user_prompt.into_inner(),
            image_size: img.image_size.to_string(),
            image_url_fal: img.image_url_fal,
            image_s3_key: img.image_s3_key.into_inner(),
            content_type: img.content_type.to_string(),
            image_alt: img.alt.into_inner(),
            image_status: img.status.to_string(),
            is_favorite: img.is_favorite,
            pre_url: None,
            s3_pre_url: None,
        }
    }
}
impl From<&ImageNew> for ImageView {
    fn from(img: &ImageNew) -> Self {
        Self {
            pid: img.pid.to_owned(),
            training_model_id: img.training_model_id,
            user_prompt: img.user_prompt.as_ref().to_owned(),
            image_size: img.image_size.clone().to_string(),
            image_url_fal: img.image_url_fal.to_owned(),
            image_s3_key: img.image_s3_key.as_ref().to_string(),
            content_type: img.content_type.to_string(),
            image_alt: img.alt.as_ref().to_owned(),
            image_status: img.status.to_string(),
            is_favorite: img.is_favorite,
            pre_url: None,
            s3_pre_url: None,
        }
    }
}

#[derive(Debug, Serialize, Clone, Constructor, From, AsRef)]
pub struct ImageViewList(Vec<ImageView>);
impl ImageViewList {
    pub fn into_inner(self) -> Vec<ImageView> {
        self.0
    }
    pub fn as_mut_vec(&mut self) -> &mut Vec<ImageView> {
        &mut self.0
    }
    pub async fn populate_s3_pre_urls(
        mut self,
        s3_client: &AwsS3,
        cache: &RedisCacheDriver,
    ) -> Self {
        match self.get_pre_url_many(s3_client, cache).await {
            Ok(()) => {}
            Err(get_pre_url_many_err) => {
                tracing::error!(
                    "Failed to generate pre_url for all images due to cache error: {:?}. Skipping S3 generation for this request.",
                    get_pre_url_many_err
                );
            }
        };
        self
    }
    pub async fn get_pre_url_many(
        &mut self,
        s3_client: &AwsS3,
        cache: &RedisCacheDriver,
    ) -> Result<(), RedisDbError> {
        let futures = self.as_mut_vec().iter_mut().map(|image| async move {
            if image.image_url_fal.is_some() && image.image_status == Status::Completed.to_string()
            {
                let updated_image = image.get_pre_url_mut(s3_client, cache).await;
                return updated_image;
            }
            if image.image_url_fal.is_some() && image.image_status == Status::Processing.to_string()
            {
                let _ = image.set_pre_url_mut(s3_client).await;
                dbg!(&image);
            }
            image
        });
        join_all(futures).await;
        Ok(())
    }
}
impl From<ImagesModelList> for ImageViewList {
    fn from(list: ImagesModelList) -> ImageViewList {
        Self(list.into_inner().iter().map(ImageView::from).collect())
    }
}
impl From<ImageNewList> for ImageViewList {
    fn from(list: ImageNewList) -> ImageViewList {
        Self(list.into_inner().iter().map(ImageView::from).collect())
    }
}
