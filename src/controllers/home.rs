#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::cookie::ExtractConsentState;
use crate::models::packs::PackModelList;
use crate::models::users::UserPid;
use crate::models::{PackModel, UserModel};
use crate::service::redis::redis::{load_cached_web, load_from_file_and_cache};
use crate::views;
use crate::views::auth::UserView;
use crate::views::packs::PackViewList;
use crate::{domain::website::Website, middleware::cookie::CookieConsentLayer};
use axum::{debug_handler, Extension};
use derive_more::Constructor;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use axum::{http::StatusCode, response::IntoResponse};
use std::path::Path;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct HomeRoutes {
        pub base: String,
        pub home_partial: String,
    }
    impl HomeRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Home::BASE),
                home_partial: String::from(Home::HOME_PARTIAL),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Home;
    impl Home {
        pub const BASE: &'static str = "/";
        pub const HOME_PARTIAL: &'static str = "/partial/home";
        pub const ROBOT_TXT: &'static str = "/robots.txt";
        pub const SITEMAP_XML: &'static str = "/sitemap.xml";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add(routes::Home::BASE, get(render_home))
        .add(routes::Home::HOME_PARTIAL, get(render_home_partial))
        .add(routes::Home::ROBOT_TXT, get(robots_txt))
        .add(routes::Home::SITEMAP_XML, get(sitemap_xml))
        .layer(CookieConsentLayer::new())
}

pub async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
    Ok(item)
}
async fn load_packs(db: &DatabaseConnection) -> Result<PackModelList> {
    let list = PackModel::find_first_12_packs(db).await?;
    Ok(PackModelList::new(list))
}

#[debug_handler]
pub async fn robots_txt(State(ctx): State<AppContext>) -> impl IntoResponse {
    let path = Path::new("assets/static/robots.txt");
    let cache_key = "robot";

    let content: String = match ctx.cache.get(cache_key).await {
        Ok(Some(cached)) => cached,
        Ok(None) => match load_from_file_and_cache(&ctx, path, cache_key).await {
            Ok(content) => content,
            Err(e) => {
                tracing::error!("Failed to load robots.txt: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "robots.txt not found").into_response();
            }
        },
        Err(e) => {
            tracing::error!("Failed to read robots.txt from cache: {}", e);
            match load_from_file_and_cache(&ctx, path, cache_key).await {
                Ok(content) => content,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "robots.txt not found")
                        .into_response();
                }
            }
        }
    };

    Response::builder()
        .header("Content-Type", "text/plain")
        .body(content)
        .unwrap()
        .into_response()
}

#[debug_handler]
pub async fn sitemap_xml(State(ctx): State<AppContext>) -> impl IntoResponse {
    let path = Path::new("assets/static/sitemap.xml");
    let cache_key = "sitemap";

    let sitemap: String = match ctx.cache.get(cache_key).await {
        Ok(Some(cached)) => cached,
        Ok(None) => match load_from_file_and_cache(&ctx, path, cache_key).await {
            Ok(content) => content,
            Err(e) => {
                tracing::error!("Failed to load sitemap.xml: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "sitemap.xml not found")
                    .into_response();
            }
        },
        Err(e) => {
            tracing::error!("Failed to read sitemap.xml from cache: {}", e);
            match load_from_file_and_cache(&ctx, path, cache_key).await {
                Ok(content) => content,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "sitemap.xml not found")
                        .into_response();
                }
            }
        }
    };

    Response::builder()
        .header("Content-Type", "application/xml")
        .body(sitemap)
        .unwrap()
        .into_response()
}

#[debug_handler]
pub async fn render_home(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user: Option<UserView> = match auth {
        Ok(auth) => {
            let user_pid = UserPid::new(&auth.claims.pid);
            let user = match load_user(&ctx.db, &user_pid).await {
                Ok(user) => Some(user.into()),
                Err(_) => None,
            };
            user
        }
        Err(_) => None,
    };
    let is_home = true;
    let images = load_cached_web(&ctx).await?;
    views::home::home(v, &website, is_home, &cc_cookie, &images, &user)
}

#[debug_handler]
pub async fn render_home_partial(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user: Option<UserView> = match auth {
        Ok(auth) => {
            let user_pid = UserPid::new(&auth.claims.pid);
            let user = match load_user(&ctx.db, &user_pid).await {
                Ok(user) => Some(user.into()),
                Err(_) => None,
            };
            user
        }
        Err(_) => None,
    };
    let is_home = true;
    let images = load_cached_web(&ctx).await?;
    views::home::home_partial(v, &website, is_home, &cc_cookie, &images, &user)
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct WebGallery {
    images_r0: Vec<String>,
    images_r1: Vec<String>,
    images_r2: Vec<String>,
    images_r3: Vec<String>,
    images_r4: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
struct WebBeforeAfter {
    before: String,
    after: String,
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct WebImages {
    hero_panel: Vec<String>,
    gallery: WebGallery,
    before_after: WebBeforeAfter,
    studio: String,
    pub packs: PackViewList,
    creators: Vec<String>,
}
impl WebImages {
    pub async fn web_images(db: &DatabaseConnection) -> WebImages {
        let hero_panel = vec![
            String::from("../../../static/images/hero/halloween-hero.webp"),
            String::from("../../../static/images/hero/nature-hero.webp"),
            String::from("../../../static/images/hero/quin-hero.webp"),
            String::from("../../../static/images/hero/lara-hero.webp"),
            String::from("../../../static/images/hero/terminator-hero.webp"),
            String::from("../../../static/images/hero/valentine-hero.webp"),
            String::from("../../../static/images/hero/spiritual-hero.webp"),
        ];
        let web_images0 = vec![
            String::from("../../../static/images/hero/nature-hero.webp"),
            String::from("../../../static/images/gallery/corporate-headshot.webp"),
            String::from("../../../static/images/gallery/mma-fe.webp"),
            String::from("../../../static/images/gallery/wife1.webp"),
            String::from("../../../static/images/gallery/street-fighter.webp"),
            String::from("../../../static/images/gallery/nature3.webp"),
        ];
        let web_images1 = vec![
            String::from("../../../static/images/gallery/nature2.webp"),
            String::from("../../../static/images/hero/quin-hero.webp"),
            String::from("../../../static/images/hero/halloween-hero.webp"),
            String::from("../../../static/images/gallery/cosplay1-small.webp"),
            String::from("../../../static/images/gallery/machina2.webp"),
            String::from("../../../static/images/gallery/cosplay2-small.webp"),
        ];
        let web_images2 = vec![
            String::from("../../../static/images/hero/lara-hero.webp"),
            String::from("../../../static/images/gallery/machina1.webp"),
            String::from("../../../static/images/gallery/angel.webp"),
            String::from("../../../static/images/hero/quin-hero.webp"),
            String::from("../../../static/images/gallery/emo-girl.webp"),
            String::from("../../../static/images/gallery/blackwidow.webp"),
        ];
        let web_images3 = vec![
            String::from("../../../static/images/hero/halloween-hero.webp"),
            String::from("../../../static/images/hero/terminator-hero.webp"),
            String::from("../../../static/images/gallery/nature1.webp"),
            String::from("../../../static/images/gallery/dracula-wife.webp"),
            String::from("../../../static/images/gallery/cosplay3.webp"),
            String::from("../../../static/images/gallery/model-show.webp"),
        ];
        let web_images4 = vec![
            String::from("../../../static/images/hero/spiritual-hero.webp"),
            String::from("../../../static/images/gallery/easter1.webp"),
            String::from("../../../static/images/gallery/model-makeup.webp"),
            String::from("../../../static/images/gallery/white-dress.webp"),
            String::from("../../../static/images/gallery/model-closeup.webp"),
            String::from("../../../static/images/hero/halloween-hero.webp"),
        ];
        let gallery = WebGallery::new(
            web_images0,
            web_images1,
            web_images2,
            web_images3,
            web_images4,
        );
        let before_after = WebBeforeAfter::new(
        String::from(
            "../../../static/images/head-shot/WhatsApp Image 2025-04-27 at 22.23.32_6190d2f4.webp",
        ),
        String::from("../../../static/images/head-shot/a5197708-06f9-4ecc-b29d-e25879d73d9b.webp"),
    );

        let studio = String::from("../../../static/images/studio/studio2.webp");

        let packs = match load_packs(db).await {
            Ok(packs) => packs,
            Err(e) => {
                tracing::error!("Failed to load packs: {}", e);
                PackModelList::new(vec![])
            }
        }
        .into();

        let creators = vec![
            String::from("../../../static/images/creators/image(5).webp"),
            String::from("../../../static/images/creators/image(1).webp"),
            String::from("../../../static/images/creators/image(3).webp"),
            String::from("../../../static/images/creators/image.webp"),
            String::from("../../../static/images/creators/image(4).webp"),
            String::from("../../../static/images/creators/image(2).webp"),
        ];
        let web_images = WebImages::new(hero_panel, gallery, before_after, studio, packs, creators);
        web_images
    }
}
