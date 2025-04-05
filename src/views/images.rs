use futures::future::try_join_all;
use loco_rs::prelude::*;

use serde::Serialize;
use uuid::Uuid;

use crate::models::images::{ImageNew, ImagesModelList, UserPrompt};
use crate::models::{ImageModel, UserCreditModel};
use crate::service::aws::s3::{AwsError, AwsS3};
use crate::{
    domain::image::Image,
    models::{_entities::images, images::ImageNewList, user_credits::UserCreditsClient},
};

pub fn one(
    v: &impl ViewRenderer,
    credits: &CreditsViewModel,
    image_list: &Vec<ImageViewModel>,
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
    images: &Vec<ImageViewModel>,
    check_route: &str,
    credits: &CreditsViewModel,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_partial.html",
        data!({"credits": credits,"images": images, "check_route": check_route}),
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

#[derive(Serialize, Clone)]
pub struct ImageViewModel {
    pub pid: Uuid,
    pub training_model_id: i32,
    pub user_prompt: String,
    pub image_size: String,
    pub image_url: Option<String>,
    pub image_url_s3: Option<String>,
    pub content_type: String,
    pub image_alt: String,
    pub image_status: String,
    pub pre_url: Option<String>,
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
        let futures = list.into_iter().map(|image| {
            let user_id = *user_id;
            let s3_client = s3_client.clone();
            async move {
                if image.image_url.is_some() && image.image_url_s3.is_none() {
                    image.set_pre_url(&user_id, &s3_client).await
                } else {
                    Ok(image)
                }
            }
        });
        try_join_all(futures).await
    }
}
impl From<ImageModel> for ImageViewModel {
    fn from(img: ImageModel) -> Self {
        Self {
            pid: img.pid,
            training_model_id: img.training_model_id,
            user_prompt: img.user_prompt,
            image_size: img.image_size.to_string(),
            image_url: img.image_url,
            image_url_s3: img.image_url_s3,
            content_type: img.content_type.to_string(),
            image_alt: img.alt,
            image_status: img.status.to_string(),
            pre_url: None,
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
            image_url: img.image_url.to_owned(),
            image_url_s3: img.image_url_s3.to_owned(),
            content_type: img.content_type.to_string(),
            image_alt: img.alt.to_owned(),
            image_status: img.status.to_string(),
            pre_url: None,
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
            image_url: img.image_url,
            image_url_s3: img.image_url_s3,
            content_type: img.content_type.to_string(),
            image_alt: img.alt.into_inner(),
            image_status: img.status.to_string(),
            pre_url: None,
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
            image_url: img.image_url.to_owned(),
            image_url_s3: img.image_url_s3.to_owned(),
            content_type: img.content_type.to_string(),
            image_alt: img.alt.as_ref().to_owned(),
            image_status: img.status.to_string(),
            pre_url: None,
        }
    }
}
impl From<ImagesModelList> for Vec<ImageViewModel> {
    fn from(list: ImagesModelList) -> Vec<ImageViewModel> {
        list.into_inner().iter().map(ImageViewModel::from).collect()
    }
}
