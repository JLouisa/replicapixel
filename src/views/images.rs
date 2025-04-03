use loco_rs::prelude::*;

use serde::Serialize;
use uuid::Uuid;

use crate::models::images::ImageNew;
use crate::models::{ImageModel, UserCreditModel};
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

pub fn img_completed(v: &impl ViewRenderer, images: &Vec<Image>) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_partial.html",
        data!({"images": images}),
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
    pub current_amount: i32,
}
impl From<&UserCreditModel> for CreditsViewModel {
    fn from(credits: &UserCreditModel) -> Self {
        Self {
            current_amount: credits.credit_amount,
        }
    }
}

#[derive(Serialize)]
pub struct ImageViewModel {
    pub pid: Uuid,
    pub user_prompt: String,
    pub image_size: String,
    pub image_url: Option<String>,
    pub image_url_s3: Option<String>,
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
            user_prompt: img.user_prompt.into_inner(),
            image_size: img.image_size.to_string(),
            image_url: img.image_url,
            image_url_s3: img.image_url_s3,
        }
    }
}
impl From<&ImageNew> for ImageViewModel {
    fn from(img: &ImageNew) -> Self {
        Self {
            pid: img.pid.to_owned(),
            user_prompt: img.user_prompt.clone().into_inner(),
            image_size: img.image_size.clone().to_string(),
            image_url: img.image_url.to_owned(),
            image_url_s3: img.image_url_s3.to_owned(),
        }
    }
}
