use derive_more::{AsRef, Constructor, From};
use futures::future::{join_all, try_join_all};
use loco_rs::prelude::*;

use serde::Serialize;
use uuid::Uuid;

use crate::domain::url::Url;
use crate::domain::website::Website;
use crate::models::_entities::sea_orm_active_enums::Status;
use crate::models::images::{ImageNew, ImagesModelList};
use crate::models::{ImageModel, UserCreditModel};
use crate::models::{_entities::images, images::ImageNewList};
use crate::service::aws::s3::{AwsError, AwsS3, S3Key};
use crate::service::redis::redis::Cache;

pub fn img_infinite_loading(
    v: &impl ViewRenderer,
    website: &Website,
    images: &ImageViewList,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_partial.html",
        data!(
            {
                "images": images,
                "check_route": website.main_routes.check,
                "delete_route":  website.main_routes.image,
            }
        ),
    )
}

pub fn one(
    v: &impl ViewRenderer,
    credits: &CreditsViewModel,
    image_list: &ImageViewList,
    check_route: &str,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_loading_partial.html",
        data!({"credits": credits, "image_list": image_list, "check_route": check_route}),
    )
}

pub fn img_completed(
    v: &impl ViewRenderer,
    images: &ImageViewList,
    website: &Website,
    credits: &CreditsViewModel,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_partial.html",
        data!(
            {
                "credits": credits,"images": images,
                "check_route": website.main_routes.check,
                "delete_route":  website.main_routes.image,
            }
        ),
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
pub struct ImageViewModel {
    pub pid: Uuid,
    pub training_model_id: i32,
    pub user_prompt: String,
    pub image_size: String,
    pub image_s3_key: String,
    pub image_url_fal: Option<String>,
    pub content_type: String,
    pub image_alt: String,
    pub image_status: String,
    pub pre_url: Option<String>,
    pub s3_pre_url: Option<String>,
}
impl ImageViewModel {
    pub async fn set_pre_url(self, user_id: &Uuid, s3_client: &AwsS3) -> Result<Self, AwsError> {
        let pre_url = s3_client
            .auto_upload_img_presigned_url(user_id, &self)
            .await?;
        let mut new_self = self;
        new_self.pre_url = Some(pre_url.into_inner());
        Ok(new_self)
    }
    pub async fn set_pre_url_many(
        list: Vec<Self>,
        user_id: &Uuid,
        s3_client: &AwsS3,
    ) -> Result<Vec<Self>, AwsError> {
        let futures = list.into_iter().map(|image| async move {
            if image.image_url_fal.is_some() && image.image_status == Status::Processing.to_string()
            {
                image.set_pre_url(user_id, s3_client).await
            } else {
                Ok(image)
            }
        });
        try_join_all(futures).await
    }
    pub async fn get_pre_url(mut self, s3_client: &AwsS3, cache: &Cache) -> Self {
        // Early exit if fields are missing
        let Some(_image_url_fal) = self.image_url_fal.clone() else {
            return self;
        };
        let s3_key = S3Key::new(self.image_s3_key.clone());

        // Check if the pre_url is already cached
        match cache.get_s3_pre_url(&self).await {
            Ok(pre_url) => {
                tracing::info!("Using cached pre_url: {}", &self.pid);
                let url = Url::new(pre_url);
                self.s3_pre_url = Some(url.into_inner());
            }
            Err(_) => match s3_client.get_object_pre(&s3_key, None).await {
                Ok(url) => {
                    tracing::info!("Generating new pre_url: {}", &self.pid);
                    self.s3_pre_url = Some(url.into_inner());
                    let _ = cache.set_s3_pre_url(&self).await;
                }
                Err(_) => return self,
            },
        }

        self
    }

    pub async fn get_pre_url_many(list: Vec<Self>, s3_client: &AwsS3, cache: &Cache) -> Vec<Self> {
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
impl From<ImageModel> for ImageViewModel {
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
            pre_url: None,
            s3_pre_url: None,
        }
    }
}
impl From<&ImageModel> for ImageViewModel {
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
            pre_url: None,
            s3_pre_url: None,
        }
    }
}

impl From<ImageNewList> for Vec<ImageViewModel> {
    fn from(list: ImageNewList) -> Vec<ImageViewModel> {
        list.into_inner().iter().map(ImageViewModel::from).collect()
    }
}
impl From<ImageNew> for ImageViewModel {
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
            pre_url: None,
            s3_pre_url: None,
        }
    }
}
impl From<&ImageNew> for ImageViewModel {
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
            pre_url: None,
            s3_pre_url: None,
        }
    }
}
impl From<ImagesModelList> for Vec<ImageViewModel> {
    fn from(list: ImagesModelList) -> Vec<ImageViewModel> {
        list.into_inner().iter().map(ImageViewModel::from).collect()
    }
}

#[derive(Debug, Serialize, Clone, Constructor, From, AsRef)]
pub struct ImageViewList(Vec<ImageViewModel>);
impl ImageViewList {
    fn into_inner(self) -> Vec<ImageViewModel> {
        self.0
    }
}
