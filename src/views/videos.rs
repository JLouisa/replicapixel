use loco_rs::prelude::*;

use derive_more::{AsRef, Constructor};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::website::WebsiteOptions;
use crate::models::_entities::sea_orm_active_enums::{AspectRatio, Status};
use crate::models::images::{AltText, UserPrompt};
use crate::models::videos::NegativePrompt;
use crate::models::VideoModel;
use crate::models::_entities::images;

pub fn img_infinite_loading(
    v: &impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/photo_router.html",
        data!({ "options": website_options }),
    )
}

pub fn one(v: &impl ViewRenderer, website_options: &WebsiteOptions) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/video/video_router.html",
        data!({  "options": website_options, "video": &website_options.video }),
    )
}

pub fn video_generated(
    v: &impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/photo_router.html",
        data!({ "options": website_options }),
    )
}

/// When there is an issue with rendering the view.
pub fn favorite(v: &impl ViewRenderer, website_options: &WebsiteOptions) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/cards/partials/favorite_button.html",
        data!({ "options": website_options }),
    )
}

/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<images::Model>) -> Result<Response> {
    format::render().view(v, "images/list.html", data!({"items": items}))
}

/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, website_options: &WebsiteOptions) -> Result<Response> {
    format::render().view(v, "images/show.html", data!({"options": website_options}))
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
#[derive(Clone, Debug, Serialize, Constructor)]
pub struct VideoView {
    pid: Uuid,
    title: String,
    user_prompt: UserPrompt,
    negative_prompt: NegativePrompt,
    alt: AltText,
    duration: String,
    generate_audio: bool,
    status: Status,
    aspect_ratio: AspectRatio,
    is_favorite: bool,
    is_deleted: bool,
    pub created_at: DateTimeWithTimeZone,
    video_url: Option<String>,
    thumbnail_url: Option<String>,
    video_pre_url: Option<String>,
}
impl VideoView {
    pub fn set_video_url(self, video_url: Option<String>) -> Self {
        Self { video_url, ..self }
    }
    pub fn set_video_pre_url(self, video_pre_url: Option<String>) -> Self {
        Self {
            video_pre_url,
            ..self
        }
    }
    pub fn set_thumbnail_url(self, thumbnail_url: Option<String>) -> Self {
        Self {
            thumbnail_url,
            ..self
        }
    }
    pub fn uuid(&self) -> Uuid {
        self.pid
    }
}
impl From<VideoModel> for VideoView {
    fn from(item: VideoModel) -> Self {
        Self {
            duration: item.parse_duration(),
            pid: item.pid,
            title: item.title,
            user_prompt: UserPrompt::new(item.user_prompt),
            negative_prompt: NegativePrompt::new(item.negative_prompt),
            alt: AltText::new(item.alt),
            generate_audio: item.generate_audio,
            status: item.status,
            aspect_ratio: item.aspect_ratio,
            is_favorite: item.is_favorite,
            is_deleted: item.deleted_at.is_some(),
            created_at: item.created_at,
            video_url: item.video_url_fal,
            thumbnail_url: None,
            video_pre_url: None,
        }
    }
}
impl From<&VideoModel> for VideoView {
    fn from(item: &VideoModel) -> Self {
        Self {
            duration: item.parse_duration(),
            pid: item.pid,
            title: item.title.clone(),
            user_prompt: UserPrompt::new(item.user_prompt.clone()),
            negative_prompt: NegativePrompt::new(item.negative_prompt.clone()),
            alt: AltText::new(item.alt.clone()),
            generate_audio: item.generate_audio,
            status: item.status,
            aspect_ratio: item.aspect_ratio.clone(),
            is_favorite: item.is_favorite,
            is_deleted: item.deleted_at.is_some(),
            created_at: item.created_at,
            video_url: item.video_url_fal.clone(),
            thumbnail_url: None,
            video_pre_url: None,
        }
    }
}

#[derive(Debug, Serialize, Clone, Constructor, AsRef)]
pub struct VideoViewList(Vec<VideoView>);
impl VideoViewList {
    pub fn one(video_view: VideoView) -> Self {
        Self(vec![video_view])
    }
    pub fn into_inner(self) -> Vec<VideoView> {
        self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}
