#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
// use std::time::Duration;

use crate::middleware::cookie::ExtractConsentState;
use crate::models::packs::PackModelList;
use crate::models::PackModel;
use crate::service::redis::redis::load_cached_web;
use crate::views;
use crate::views::dashboard::PackViewList;
use crate::{domain::website::Website, middleware::cookie::CookieConsentLayer};
use axum::{debug_handler, Extension};
use derive_more::Constructor;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

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
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add(routes::Home::BASE, get(render_home))
        .add(routes::Home::HOME_PARTIAL, get(render_home_partial))
        .layer(CookieConsentLayer::new())
}

async fn load_packs(db: &DatabaseConnection) -> Result<PackModelList> {
    let list = PackModel::find_all_packs(db).await?;
    Ok(PackModelList::new(list))
}

// async fn fetch_and_cache_web_images(ctx: &AppContext) -> CacheResult<WebImages> {
//     let images = web_images(&ctx.db).await;
//     match serde_json::to_string(&images) {
//         Ok(serialized) => {
//             if let Err(e) = ctx
//                 .cache
//                 .insert_with_expiry("web", &serialized, Duration::from_secs(60))
//                 .await
//             {
//                 tracing::error!("Failed to write web images to cache: {}", e);
//             }
//         }
//         Err(e) => {
//             tracing::error!("Failed to serialize web images: {}", e);
//         }
//     }
//     Ok(images)
// }
// async fn load_cached_web(ctx: &AppContext) -> CacheResult<WebImages> {
//     match ctx.cache.get("web").await {
//         Ok(Some(cached)) => match serde_json::from_str::<WebImages>(&cached) {
//             Ok(data) => Ok(data),
//             Err(err) => {
//                 tracing::error!("Failed to deserialize cached web images: {}", err);
//                 fetch_and_cache_web_images(ctx).await
//             }
//         },
//         Ok(None) => {
//             tracing::info!("Web images not found in cache, loading from DB.");
//             fetch_and_cache_web_images(ctx).await
//         }
//         Err(err) => {
//             tracing::error!("Failed to read from cache: {}", err);
//             fetch_and_cache_web_images(ctx).await
//         }
//     }
// }

#[debug_handler]
pub async fn render_home(
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let images = load_cached_web(&ctx).await?;
    views::home::home(v, &website, is_home, &cc_cookie, &images)
}

#[debug_handler]
pub async fn render_home_partial(
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let images = load_cached_web(&ctx).await?;
    views::home::home_partial(v, &website, is_home, &cc_cookie, &images)
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
struct WebGallery {
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
            String::from("../../../static/images/hero/nature-hero.webp"),
            String::from("../../../static/images/gallery/wife1.webp"),
            String::from("../../../static/images/hero/nature-hero.webp"),
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
            String::from("../../../static/images/hero/lara-hero.webp"),
            String::from("../../../static/images/hero/quin-hero.webp"),
            String::from("../../../static/images/hero/lara-hero.webp"),
            String::from("../../../static/images/gallery/blackwidow.webp"),
        ];
        let web_images3 = vec![
            String::from("../../../static/images/hero/halloween-hero.webp"),
            String::from("../../../static/images/hero/terminator-hero.webp"),
            String::from("../../../static/images/gallery/nature1.webp"),
            String::from("../../../static/images/hero/terminator-hero.webp"),
            String::from("../../../static/images/gallery/cosplay3.webp"),
            String::from("../../../static/images/hero/terminator-hero.webp"),
        ];
        let web_images4 = vec![
            String::from("../../../static/images/hero/spiritual-hero.webp"),
            String::from("../../../static/images/gallery/easter1.webp"),
            String::from("../../../static/images/hero/spiritual-hero.webp"),
            String::from("../../../static/images/creator1.png"),
            String::from("../../../static/images/hero/spiritual-hero.webp"),
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
            String::from("../../../static/images/creator1.webp"),
            String::from("../../../static/images/creator2.webp"),
            String::from("../../../static/images/creator3.webp"),
            String::from("../../../static/images/creator3.webp"),
            String::from("../../../static/images/creator2.webp"),
            String::from("../../../static/images/creator1.webp"),
        ];
        let web_images = WebImages::new(hero_panel, gallery, before_after, studio, packs, creators);
        web_images
    }
}
