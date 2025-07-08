use loco_rs::prelude::*;
use serde::Serialize;

use crate::{
    controllers::admin::routes::AdminRoutes,
    models::{
        packs_translations::{PackTranslatedView, TranslateGroupView},
        PackModel,
        _entities::sea_orm_active_enums::Language,
    },
};

use super::{auth::UserView, packs::PackViewList};

pub fn packs(
    v: impl ViewRenderer,
    packs: &PackViewList,
    user: &UserView,
    is_img: bool,
    admin_routes: &AdminRoutes,
    pack_translates: &TranslateGroupView,
) -> Result<impl IntoResponse> {
    let mut list_packs = packs.clone().into_inner();
    list_packs.reverse();
    format::render().view(
        &v,
        "admin/packs.html",
        data!(
            {
                "user": user, "packs": list_packs, "is_img": is_img,
                "admin_routes": admin_routes, "pack_translates": pack_translates
            }
        ),
    )
}

pub fn packs_form_partial(
    v: impl ViewRenderer,
    packs: &PackViewList,
    user: &UserView,
    is_img: bool,
    admin_routes: &AdminRoutes,
) -> Result<impl IntoResponse> {
    let key = if is_img {
        "admin/pack_form_img_partial.html"
    } else {
        "admin/pack_form_partial.html"
    };
    format::render().view(
        &v,
        key,
        data!(
            {
                "user": user, "packs": packs, "is_img": is_img,
                "admin_routes": admin_routes

            }
        ),
    )
}

pub fn packs_form_edit_partial_translated(
    v: impl ViewRenderer,
    translated_info: &PackTranslatedView,
    user: &UserView,
    admin_routes: &AdminRoutes,
    language: &Language,
    pack: &PackAdmin,
    is_successfully_updated: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "admin/pack_translate_form_partial.html",
        data!(
            {
                "user": user, "translated_info": translated_info, "edit_link": admin_routes.admin_packs_edit,
                "admin_routes": admin_routes, "translated_language": language, "pack": pack, "is_successfully_updated": is_successfully_updated
            }
        ),
    )
}

pub fn packs_form_edit_partial(
    v: impl ViewRenderer,
    pack: &PackAdmin,
    user: &UserView,
    admin_routes: &AdminRoutes,
    pack_translates: &TranslateGroupView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "admin/pack_forms.html",
        data!(
            {
                "user": user, "pack": pack, "edit_link": admin_routes.admin_packs_edit,
                "admin_routes": admin_routes, "pack_translates": pack_translates
            }
        ),
    )
}

#[derive(Debug, Clone, Serialize)]
pub struct PackAdmin {
    pub id: i32,
    pub pid: Uuid,
    pub title: String,
    pub title_url: String,
    pub short_description: String,
    pub full_description: String,
    pub pack_prompts: String,
    pub credits: i32,
    pub num_images: i32,
    pub num_inference_steps: i32,
    pub stars: i32,
    pub popular: bool,
    pub main_image: String,
    pub images: String,
    pub features: String,
}
impl From<PackModel> for PackAdmin {
    fn from(packs: PackModel) -> Self {
        Self {
            id: packs.id,
            pid: packs.pid,
            title: packs.title,
            title_url: packs.title_url,
            short_description: packs.short_description,
            full_description: packs.full_description,
            pack_prompts: packs.pack_prompts,
            credits: packs.credits,
            num_images: packs.num_images,
            num_inference_steps: packs.num_inference_steps,
            stars: packs.stars,
            popular: packs.popular,
            main_image: packs.main_image,
            images: Self::convert_vec_to_str(&packs.images),
            features: Self::convert_vec_to_str(&packs.features),
        }
    }
}
impl PackAdmin {
    pub fn convert_vec_to_str(list: &Option<Vec<String>>) -> String {
        match list {
            Some(vec) => vec.join(","),
            None => String::new(),
        }
    }
}
