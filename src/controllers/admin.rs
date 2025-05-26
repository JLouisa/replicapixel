#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    models::PackActiveModel,
    views::{self},
};
use axum::{debug_handler, response::Redirect, Extension};
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::{
    domain::settings::OtherSettings,
    models::{packs::PackModelList, users::UserPid, PackModel, UserModel},
};

pub mod routes {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Admin;
    impl Admin {
        pub const BASE: &'static str = "/admin";
        pub const ADMIN_PACKS: &'static str = "/packs";
        pub const ADMIN_PACKS_IMG: &'static str = "/packs/img";
        pub const ADMIN_PACK_ADD: &'static str = "/pack/add";
        pub const ADMIN_PACK_ADD_IMG: &'static str = "/pack/add/img";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("admin")
        .add(routes::Admin::ADMIN_PACKS, get(admin_packs))
        .add(routes::Admin::ADMIN_PACKS_IMG, get(admin_packs_img))
        .add(routes::Admin::ADMIN_PACK_ADD, post(add_pack))
        .add(routes::Admin::ADMIN_PACK_ADD_IMG, post(admin_packs_img))
}

pub async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
    Ok(item)
}
async fn load_packs(db: &DatabaseConnection) -> Result<PackModelList> {
    let list = PackModel::find_first_12_packs(db).await?;
    Ok(PackModelList::new(list))
}

#[derive(Debug, Deserialize)]
pub struct CreatePackPayload {
    #[serde(default = "Uuid::new_v4", skip_deserializing)]
    pub pid: Uuid,
    pub title: String,
    pub title_url: String,
    pub short_description: String,
    pub full_description: String,
    pub pack_prompts: String,
    pub credits: i32,
    pub num_images: i32,
    #[serde(default = "default_num_inference_steps")]
    pub num_inference_steps: i32,
    #[serde(default = "default_stars")]
    pub stars: i32,
    #[serde(default)]
    pub popular: bool,
    pub main_image: String,
    #[serde(default, deserialize_with = "deserialize_comma_separated_string_array")]
    pub images: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_comma_separated_string_array")]
    pub features: Vec<String>,
}
impl CreatePackPayload {
    pub async fn save(&self, db: &DatabaseConnection) -> ModelResult<PackModel> {
        let pack = PackActiveModel::save(db, self).await?;
        Ok(pack)
    }
    /// Sanitizes the `title_url` field in-place.
    /// If `title_url` is empty or becomes empty after sanitization,
    /// it attempts to generate it from the `title` field.
    pub fn sanitize_title_url_in_place(&mut self) {
        let mut sanitized = self
            .title_url
            .trim()
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>();

        // If title_url was empty or only special chars, try to use title
        if sanitized.is_empty() && !self.title.is_empty() {
            sanitized = self
                .title
                .trim()
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                .collect::<String>();
        }
        self.title_url = sanitized;
    }
}
fn default_num_inference_steps() -> i32 {
    50
}
fn default_stars() -> i32 {
    5
}
fn deserialize_comma_separated_string_array<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(s.split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty()) // Remove empty strings resulting from trailing commas or multiple commas
            .collect())
    }
}

#[debug_handler]
pub async fn admin_packs(
    auth: auth::JWT,
    Extension(other): Extension<OtherSettings>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    if auth.claims.pid != other.admin {
        return Ok(Redirect::to("/login").into_response());
    }
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let packs = load_packs(&ctx.db).await?;
    let view_output = views::admin::packs(v, &packs.into(), &user.into(), false)?;
    Ok(view_output.into_response())
}
#[debug_handler]
pub async fn admin_packs_img(
    auth: auth::JWT,
    Extension(other): Extension<OtherSettings>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    if auth.claims.pid != other.admin {
        return Ok(Redirect::to("/login").into_response());
    }
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let packs = load_packs(&ctx.db).await?;
    let view_output = views::admin::packs(v, &packs.into(), &user.into(), true)?;
    Ok(view_output.into_response())
}

#[debug_handler]
pub async fn add_pack(
    auth: auth::JWT,
    Extension(other): Extension<OtherSettings>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(mut form): Json<CreatePackPayload>,
) -> Result<impl IntoResponse> {
    if auth.claims.pid != other.admin {
        return Ok(Redirect::to("/login").into_response());
    }
    dbg!(&form);
    form.sanitize_title_url_in_place();
    form.save(&ctx.db).await?;
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let packs = load_packs(&ctx.db).await?;
    let view_output = views::admin::packs_form_partial(v, &packs.into(), &user.into(), false)?;
    Ok(view_output.into_response())
}
