use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    controllers::{dashboard::WebsiteOptions, home::WebGallery},
    models::{packs::PackModelList, PackModel},
};

pub fn get_all_packs(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/sections/partials/pack_inner_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn packs(v: impl ViewRenderer, website_options: &WebsiteOptions) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "packs/pack_base.html",
        data!(
            {
                "website": website_options.website, "cc_cookie": website_options.cc_cookie,
                "web_images": website_options.web_images, "pack": website_options.pack,
                "pack_images": website_options.pack_images,
                "user": website_options.user, "is_pack": website_options.is_pack,
                "options": website_options
            }
        ),
    )
}

pub fn packs_partial(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "packs/pack_partial.html",
        data!({ "options": website_options }),
    )
}

#[derive(Debug, Serialize, Deserialize, Clone, Constructor)]
pub struct PackView {
    pub id: i32,
    pub pid: Uuid,
    pub title: String,
    pub title_url: String,
    pub short_description: String,
    pub full_description: String,
    pub credits: i32,
    pub num_images: i32,
    pub main_image: String,
    pub used: i32,
    pub is_popular: bool,
    pub images: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
}
impl From<PackModel> for PackView {
    fn from(p: PackModel) -> Self {
        Self {
            id: p.id,
            pid: p.pid,
            title: p.title,
            title_url: p.title_url,
            short_description: p.short_description,
            full_description: p.full_description,
            credits: p.credits,
            num_images: p.num_images,
            main_image: p.main_image,
            used: p.used,
            is_popular: p.popular,
            images: p.images,
            features: p.features,
        }
    }
}
impl PackView {
    pub fn create_item_groups(&self) -> WebGallery {
        let list = self.images.clone().unwrap();
        assert!(
            (2..=30).contains(&list.len()),
            "Original vector must have between 2 and 6 items"
        );

        // Repeat original items to get exactly 30 items
        let mut expanded = Vec::with_capacity(30);
        while expanded.len() < 30 {
            for item in &list {
                if expanded.len() >= 30 {
                    break;
                }
                expanded.push(item.clone());
            }
        }

        // Split into 5 groups of 6
        // Chunk into five groups of six
        let group1 = expanded[0..6].to_vec();
        let group2 = expanded[6..12].to_vec();
        let group3 = expanded[12..18].to_vec();
        let group4 = expanded[18..24].to_vec();
        let group5 = expanded[24..30].to_vec();

        WebGallery::new(group1, group2, group3, group4, group5)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsRef, Constructor)]
pub struct PackViewList(pub Vec<PackView>);
impl PackViewList {
    pub fn into_inner(&self) -> Vec<PackView> {
        self.0.clone()
    }
}

impl From<PackModelList> for PackViewList {
    fn from(p: PackModelList) -> Self {
        Self(p.0.into_iter().map(|x| x.into()).collect())
    }
}
