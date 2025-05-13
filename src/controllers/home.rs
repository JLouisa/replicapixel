#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
// use std::time::Duration;

use crate::middleware::cookie::ExtractConsentState;
use crate::models::packs::PackModelList;
use crate::models::PackModel;
use crate::service::redis::redis::load_cached_web;
use crate::views;
use crate::views::packs::PackViewList;
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
        pub const ROBOT_TXT: &'static str = "/robots.txt";
        pub const SITEMAP: &'static str = "/sitemap.xml";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add(routes::Home::BASE, get(render_home))
        .add(routes::Home::HOME_PARTIAL, get(render_home_partial))
        .add(routes::Home::ROBOT_TXT, get(robots_txt))
        .add(routes::Home::SITEMAP, get(sitemap_xml))
        .layer(CookieConsentLayer::new())
}

async fn load_packs(db: &DatabaseConnection) -> Result<PackModelList> {
    let list = PackModel::find_all_packs(db).await?;
    Ok(PackModelList::new(list))
}

pub async fn robots_txt() -> impl IntoResponse {
    let content = r#"User-agent: *
User-agent: *
Disallow: /api/
Disallow: /partial/
Disallow: /studio/partial/
Disallow: /settings/
Disallow: /_health
Disallow: /_ping

# Allow public-facing pages
Allow: /

# Example for disallowing specific file types if needed (less common for "simple")
# Disallow: /*.pdf$
# Disallow: /*.doc$

# Example for disallowing search result pages (if they don't add SEO value)
# Disallow: /search
# Disallow: /*?s=
# Disallow: /*&query=

# Allow everything else (this is implicit if no other Disallow matches,
# but an empty Disallow: line makes it explicit for this block)
# Disallow:

# Sitemap location
Sitemap: https://www.replicapixel.com/sitemap.xml
"#
    .to_string();

    Response::builder()
        .header("Content-Type", "text/plain")
        .body(content)
        .unwrap()
        .into_response()
}

pub async fn sitemap_xml() -> impl IntoResponse {
    let sitemap = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset 
  xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
>
  <url><loc>https://replicapixel.com/</loc></url>
  <url><loc>https://replicapixel.com/login</loc></url>
  <url><loc>https://replicapixel.com/register</loc></url>
  <url><loc>https://replicapixel.com/forgot</loc></url>

  <!-- Policy pages -->
  <url><loc>https://replicapixel.com/policy/privacy</loc></url>
  <url><loc>https://replicapixel.com/policy/terms-and-conditions</loc></url>
  <url><loc>https://replicapixel.com/policy/model-consent</loc></url>
  <url><loc>https://replicapixel.com/policy/cookie</loc></url>

  <!-- Studio area -->
  <url><loc>https://replicapixel.com/studio</loc></url>
  <url><loc>https://replicapixel.com/studio/models</loc></url>
  <url><loc>https://replicapixel.com/studio/album/deleted</loc></url>
  <url><loc>https://replicapixel.com/studio/album/favorite</loc></url>
  <url><loc>https://replicapixel.com/studio/features</loc></url>
  <url><loc>https://replicapixel.com/studio/notifications</loc></url>
  <url><loc>https://replicapixel.com/studio/packs</loc></url>
  <url><loc>https://replicapixel.com/studio/photo</loc></url>
  <url><loc>https://replicapixel.com/studio/settings</loc></url>
  <url><loc>https://replicapixel.com/studio/models/create</loc></url>
  <url><loc>https://replicapixel.com/studio/billing</loc></url>

  <!-- Starter page -->
  <url><loc>https://replicapixel.com/starter</loc></url>

  <!-- Payment pages -->
  <url><loc>https://replicapixel.com/payment/plan</loc></url>
  <url><loc>https://replicapixel.com/payment/success</loc></url>
  <url><loc>https://replicapixel.com/payment/cancel</loc></url>
</urlset>"#
        .to_string();

    Response::builder()
        .header("Content-Type", "application/xml")
        .body(sitemap)
        .unwrap()
        .into_response()
}

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
