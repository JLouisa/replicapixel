#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::cookie::ExtractConsentState;
use crate::models::packs::PackModelList;
use crate::models::PackModel;
use crate::service::redis::redis::{RedisCacheDriver, RedisDbError};
use crate::views;
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

#[debug_handler]
pub async fn render_home(
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let images = match cache.get_web_images().await {
        Ok(images_from_cache) => {
            // Cache Hit
            tracing::info!("Using web images from cache.");
            images_from_cache // Use cached version
        }
        Err(RedisDbError::NotFound) => {
            // Clean Cache Clean Miss
            tracing::warn!("Web images not found in cache. Generating and attempting to cache.");
            let generated_images = web_images(); // Generate fresh data

            // Attempt to cache the new data
            match cache.set_web_images(&generated_images).await {
                Ok(_) => {
                    // Log success (already logged in set_web_images with debug level)
                    tracing::info!("Successfully cached newly generated web images.");
                }
                Err(e) => {
                    // Log failure to cache, but proceed with generated data
                    tracing::error!("Failed to cache newly generated web images: {}", e);
                }
            }
            generated_images
        }
        Err(e) => {
            // This is an unexpected cache error (Redis connection, deserialization failed, etc.)
            tracing::error!(
                "Error retrieving web images from cache: {}. Using generated fallback.",
                e
            );
            web_images()
        }
    };
    let packs = load_packs(&ctx.db).await?;
    views::home::home(v, &website, is_home, &cc_cookie, &images, &packs.into())
}

#[debug_handler]
pub async fn render_home_partial(
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let is_home = true;
    let images = match cache.get_web_images().await {
        Ok(images_from_cache) => {
            // Cache Hit
            tracing::info!("Using web images from cache.");
            images_from_cache // Use cached version
        }
        Err(RedisDbError::NotFound) => {
            // Clean Cache Clean Miss
            tracing::warn!("Web images not found in cache. Generating and attempting to cache.");
            let generated_images = web_images(); // Generate fresh data

            // Attempt to cache the new data
            match cache.set_web_images(&generated_images).await {
                Ok(_) => {
                    // Log success (already logged in set_web_images with debug level)
                    tracing::info!("Successfully cached newly generated web images.");
                }
                Err(e) => {
                    // Log failure to cache, but proceed with generated data
                    tracing::error!("Failed to cache newly generated web images: {}", e);
                }
            }
            generated_images
        }
        Err(e) => {
            // This is an unexpected cache error (Redis connection, deserialization failed, etc.)
            tracing::error!(
                "Error retrieving web images from cache: {}. Using generated fallback.",
                e
            );
            web_images()
        }
    };
    let packs = load_packs(&ctx.db).await?;
    views::home::home_partial(v, &website, is_home, &cc_cookie, &images, &packs.into())
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
}

fn web_images() -> WebImages {
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
        String::from("../../../static/images/gallery/corporate-headshot.jpg"),
        String::from("../../../static/images/hero/nature-hero.webp"),
        String::from("../../../static/images/gallery/wife1.jpg"),
        String::from("../../../static/images/hero/nature-hero.webp"),
        String::from("../../../static/images/gallery/nature3.jpg"),
    ];
    let web_images1 = vec![
        String::from("../../../static/images/gallery/nature2.jpg"),
        String::from("../../../static/images/hero/quin-hero.webp"),
        String::from("../../../static/images/hero/halloween-hero.webp"),
        String::from("../../../static/images/gallery/cosplay1-small.jpg"),
        String::from("../../../static/images/gallery/machina2.jpg"),
        String::from("../../../static/images/gallery/cosplay2-small.jpg"),
    ];
    let web_images2 = vec![
        String::from("../../../static/images/hero/lara-hero.webp"),
        String::from("../../../static/images/gallery/machina1.jpg"),
        String::from("../../../static/images/hero/lara-hero.webp"),
        String::from("../../../static/images/hero/quin-hero.webp"),
        String::from("../../../static/images/hero/lara-hero.webp"),
        String::from("../../../static/images/gallery/blackwidow.jpg"),
    ];
    let web_images3 = vec![
        String::from("../../../static/images/hero/halloween-hero.webp"),
        String::from("../../../static/images/hero/terminator-hero.webp"),
        String::from("../../../static/images/gallery/nature1.jpg"),
        String::from("../../../static/images/hero/terminator-hero.webp"),
        String::from("../../../static/images/gallery/cosplay3.jpg"),
        String::from("../../../static/images/hero/terminator-hero.webp"),
    ];
    let web_images4 = vec![
        String::from("../../../static/images/hero/spiritual-hero.webp"),
        String::from("../../../static/images/gallery/easter1.jpg"),
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
            "../../../static/images/head-shot/WhatsApp Image 2025-04-27 at 22.23.32_6190d2f4.jpg",
        ),
        String::from("../../../static/images/head-shot/a5197708-06f9-4ecc-b29d-e25879d73d9b.jpg"),
    );
    let web_images = WebImages::new(hero_panel, gallery, before_after);
    web_images
}
