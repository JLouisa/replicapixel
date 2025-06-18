#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::controllers::auth::HxRedirect;
use crate::controllers::dashboard::CurrentPage;
use crate::controllers::payment::PricingViewList;
use crate::domain::website::{Website, WebsiteOptions};
use crate::middleware::cookie::ExtractConsentState;
use crate::middleware::i18nv2::LangEngine;
use crate::models::_entities::sea_orm_active_enums::Language;
use crate::models::join::user_credits_models::load_user_credit_training;
use crate::models::packs::PackModelList;
use crate::models::users::UserPid;
use crate::models::{PackModel, PlanModel, UserModel};
use crate::service::redis::redis::{
    load_cached_web, load_from_file_and_cache, RedisCacheDriver, RedisKey,
};
use crate::views;
use crate::views::auth::UserView;
use crate::views::packs::PackViewList;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

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
pub async fn load_packs_translated(
    db: &DatabaseConnection,
    lang: &Language,
    cache: &RedisCacheDriver,
) -> Result<PackViewList> {
    let key = RedisKey::Packs(lang.to_owned());
    let packs = match cache.get::<PackViewList>(&key).await {
        Ok(Some(packs)) => packs,
        Ok(None) => {
            let list = PackModel::find_all_translated(db, lang).await?.into();
            let _ = cache.set::<PackViewList>(&key, &list, None).await;
            list
        }
        Err(err) => {
            tracing::error!("Failed to read from cache: {}", err);
            let list = PackModel::find_all_translated(db, lang).await?.into();
            let _ = cache.set::<PackViewList>(&key, &list, None).await;
            list
        }
    };
    Ok(packs)
}
pub async fn load_pricing_translated(
    db: &DatabaseConnection,
    lang: &Language,
    cache: &RedisCacheDriver,
) -> Result<PricingViewList> {
    let key = RedisKey::Pricing(lang.to_owned());
    let pricing = match cache.get::<PricingViewList>(&key).await {
        Ok(Some(pricing)) => pricing,
        Ok(None) => {
            let list = PlanModel::find_all_translated(db, lang).await?.into();
            let _ = cache.set::<PricingViewList>(&key, &list, None).await;
            list
        }
        Err(err) => {
            tracing::error!("Failed to read from cache: {}", err);
            let list = PlanModel::find_all_translated(db, lang).await?.into();
            let _ = cache.set::<PricingViewList>(&key, &list, None).await;
            list
        }
    };
    Ok(pricing)
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
    Extension(cache): Extension<RedisCacheDriver>,
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
    let images = load_cached_web(&ctx, &lang, &cache).await?;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .set_user(user)
        .packs(images.packs())
        .web_gallery(&images.gallery)
        .web_images(&images)
        .is_home()
        .build();

    views::home::home(v, &website_options)
}

#[debug_handler]
pub async fn render_home_partial(
    auth: Result<auth::JWT>,
    ExtractConsentState(cc_cookie): ExtractConsentState,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
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
    let images = load_cached_web(&ctx, &lang, &cache).await?;

    let website_options = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .cc_cookie(&cc_cookie)
        .set_user(user)
        .packs(images.packs())
        .web_gallery(&images.gallery)
        .web_images(&images)
        .is_home()
        .build();

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
        .current_page(CurrentPage::Models)
        .build();

    Ok(views::dashboard::home_training_partial_dashboard(v, &website_options).into_response())
}
#[debug_handler]
pub async fn render_dashboard_extend_packs_partial(
    auth: Result<auth::JWT>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    Extension(cache): Extension<RedisCacheDriver>,
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
    let packs = match load_cached_web(&ctx, &lang, &cache).await {
        Ok(images) => images.packs().clone(),
        Err(_) => load_packs(&ctx.db).await?.into(),
    };

    let website_options: WebsiteOptions = WebsiteOptions::new()
        .website(&website)
        .language(&lang)
        .user(user.into())
        .user_credits(user_credits.into())
        .training_models(training_models.into())
        .packs(&packs)
        .current_page(CurrentPage::Packs)
        .build();

    Ok(views::dashboard::home_packs_partial_dashboard(v, &website_options).into_response())
}
