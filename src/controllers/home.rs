#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::controllers::auth::HxRedirect;
use crate::controllers::dashboard::{CurrentPage, WebsiteOptions};
use crate::controllers::payment::PricingViewList;
use crate::domain::website::Website;
use crate::middleware::cookie::ExtractConsentState;
use crate::middleware::i18nv2::LangEngine;
use crate::models::_entities::sea_orm_active_enums::Language;
use crate::models::join::user_credits_models::load_user_credit_training;
use crate::models::packs::PackModelList;
use crate::models::users::UserPid;
use crate::models::{PackModel, PlanModel, UserModel};
use crate::service::redis::redis::{load_cached_web, load_from_file_and_cache};
use crate::views;
use crate::views::auth::UserView;
use crate::views::packs::PackViewList;
use axum::{debug_handler, Extension};
use derive_more::Constructor;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

// use crate::controllers::auth::routes as AuthRoutes;
use axum::{http::StatusCode, response::IntoResponse};
use std::path::Path;

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct HomeRoutes {
        pub base: String,
        pub home_partial: String,
        pub dashboard_extend: String,
        pub dashboard_packs_extend: String,
    }
    impl HomeRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Home::BASE),
                home_partial: String::from(Home::HOME_PARTIAL),
                dashboard_extend: String::from(Home::DASHBOARD_EXTEND),
                dashboard_packs_extend: String::from(Home::DASHBOARD_PACKS_EXTEND),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Home;
    impl Home {
        pub const BASE: &'static str = "/";
        pub const ROBOT_TXT: &'static str = "/robots.txt";
        pub const SITEMAP_XML: &'static str = "/sitemap.xml";
        pub const HOME_PARTIAL: &'static str = "/partial/home";
        pub const DASHBOARD_EXTEND: &'static str = "/partial/dashboard";
        pub const DASHBOARD_PACKS_EXTEND: &'static str = "/partial/dashboard/packs";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add(routes::Home::BASE, get(render_home))
        .add(routes::Home::ROBOT_TXT, get(robots_txt))
        .add(routes::Home::SITEMAP_XML, get(sitemap_xml))
        .add(routes::Home::HOME_PARTIAL, get(render_home_partial))
        .add(
            routes::Home::DASHBOARD_EXTEND,
            get(render_dashboard_extend_partial),
        )
        .add(
            routes::Home::DASHBOARD_PACKS_EXTEND,
            get(render_dashboard_extend_packs_partial),
        )
}

pub async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
    Ok(item)
}
async fn load_packs(db: &DatabaseConnection) -> Result<PackModelList> {
    let list = PackModel::find_all_packs(db).await?;
    Ok(PackModelList::new(list))
}
async fn load_packs_translated(db: &DatabaseConnection, lang: &Language) -> Result<PackViewList> {
    let list = PackModel::find_all_translated(db, lang).await?;
    Ok(list.into())
}
async fn load_pricing_translated(
    db: &DatabaseConnection,
    lang: &Language,
) -> Result<PricingViewList> {
    let list = PlanModel::find_all_translated(db, lang).await?;
    Ok(list.into())
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
    LangEngine(lang): LangEngine,
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
    let images = load_cached_web(&ctx, &lang).await?;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .set_user(user)
        .web_images(&images)
        .is_home();

    views::home::home(v, &website_options)
}

#[debug_handler]
pub async fn render_home_partial(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
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
    let images = load_cached_web(&ctx, &lang).await?;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .set_user(user)
        .web_images(&images)
        .is_home();

    views::home::home_partial(v, &website_options)
}

#[debug_handler]
pub async fn render_dashboard_extend_partial(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .current_page(CurrentPage::Models);

    Ok(views::dashboard::home_training_partial_dashboard(v, &website_options).into_response())
}
#[debug_handler]
pub async fn render_dashboard_extend_packs_partial(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
    LangEngine(lang): LangEngine,
) -> Result<impl IntoResponse> {
    let user_pid = match auth {
        Ok(auth) => UserPid::new(&auth.claims.pid),
        Err(_) => {
            return Ok(HxRedirect::login().into_response());
        }
    };
    let (user, user_credits, training_models) =
        load_user_credit_training(&ctx.db, &user_pid).await?;
    let packs = match load_cached_web(&ctx, &lang).await {
        Ok(images) => images.packs,
        Err(_) => load_packs(&ctx.db).await?.into(),
    };

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .packs(packs.into())
        .current_page(CurrentPage::Packs);

    Ok(views::dashboard::home_packs_partial_dashboard(v, &website_options).into_response())
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
    plans: PricingViewList,
}
impl WebImages {
    pub async fn web_images(db: &DatabaseConnection, lang: &Language) -> WebImages {
        let hero_panel = vec![
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/sexy+halloween/a22ec84c-dcd7-4cbd-b872-1963aa140355.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/nature/f40a699f-8064-4015-80d2-ffb68228ac2e.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/cosplay/f193a28b-83e3-4a1c-b13e-3637acb85c84.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/cosplay/a97bb59a-be4f-4b3f-92b5-e8c25a03e361.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/cosplay/53d42133-d8be-47a8-863b-1a489b2a736e.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/sexy-valentine/fcf51df7-27d6-48ad-a34e-a96a78ddeb02.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/spiritual/3b7e781a-6b40-4ef8-8d58-b52bcabddc87.webp"),
        ];
        let web_images0 = vec![
            String::from("../../../static/images/hero/nature-hero.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/corporate-headshot.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/mma-fe.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/wife1.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/street-fighter.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/nature3.webp"),
        ];
        let web_images1 = vec![
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/nature2.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/cosplay/f193a28b-83e3-4a1c-b13e-3637acb85c84.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/sexy+halloween/e5557da7-416a-466c-a5a7-bf7232232ee3.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/cosplay1-small.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/machina2.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/cosplay2-small.webp"),
        ];
        let web_images2 = vec![
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/cosplay/a97bb59a-be4f-4b3f-92b5-e8c25a03e361.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/machina1.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/angel.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/cosplay/f193a28b-83e3-4a1c-b13e-3637acb85c84.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/emo-girl.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/blackwidow.webp"),
        ];
        let web_images3 = vec![
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/sexy+halloween/a22ec84c-dcd7-4cbd-b872-1963aa140355.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/cosplay/53d42133-d8be-47a8-863b-1a489b2a736e.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/nature1.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/dracula-wife.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/cosplay3.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/model-show.webp"),
        ];
        let web_images4 = vec![
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/spiritual/e1ee3b51-53a0-4254-9a09-8d734ea7195a.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/easter1.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/model-makeup.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/white-dress.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/gallery/model-closeup.webp"),
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/packs/sexy+halloween/f861732f-79ed-4c0d-904d-c43b714807c8.webp"),
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
                "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/home-before.webp",
            ),
            String::from(
                "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/home-after.webp",
            ),
        );

        let studio = String::from(
            "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/studio.webp",
        );

        let packs = match load_packs_translated(db, lang).await {
            Ok(packs) => packs,
            Err(e) => {
                tracing::error!("Failed to load packs: {}", e);
                PackViewList::default()
            }
        }
        .into();

        let creators = vec![
            String::from("https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/got.webp"),
            String::from(
                "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/dynasty.webp",
            ),
            String::from(
                "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/cosplay-widow.webp",
            ),
            String::from(
                "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/elf-queen.webp",
            ),
            String::from(
                "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/dynasty2.webp",
            ),
            String::from(
                "https://replicapixel-web.s3.eu-central-1.amazonaws.com/others/cosplay-lara.webp",
            ),
        ];

        let plans = match load_pricing_translated(db, lang).await {
            Ok(pack) => pack,
            Err(e) => {
                tracing::error!("Failed to load plans: {}", e);
                PricingViewList::default()
            }
        };

        let web_images = WebImages::new(
            hero_panel,
            gallery,
            before_after,
            studio,
            packs,
            creators,
            plans,
        );
        web_images
    }
}
