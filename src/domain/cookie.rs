use axum_extra::extract::cookie::{Cookie as AxumCookie, SameSite};
use loco_oauth2::models::users::OAuth2UserTrait;
use loco_rs::app::AppContext;
use sea_orm::ModelTrait;
use time::{Duration, OffsetDateTime};

use thiserror::Error;

use crate::models::{
    users::OAuth2UserProfile, UserModel, _entities::sea_orm_active_enums::Language,
};

#[derive(Debug, Error)]
pub enum CookieError {
    #[error("Failed to generate JWT: {0}")]
    JwtCreationError(String),

    #[error("Failed to generate Token: {0}")]
    TokenCreationError(String),

    #[error("Missing required configuration")]
    MissingConfig,

    #[error("Invalid language preference")]
    InvalidLanguage,

    #[error("Unknown cookie error")]
    Unknown,
}

pub type CookieResult<T> = std::result::Result<T, CookieError>;

pub trait CookieTrait {
    fn logout_cookie() -> AxumCookie<'static>;
    fn create_language_cookie(lang_pref: Language) -> AxumCookie<'static>;
}

pub trait UserCookieTrait<T>: OAuth2UserTrait<T> + ModelTrait {
    fn create_cookie_base(
        &self,
        ctx: &AppContext,
        same_site: SameSite,
    ) -> CookieResult<AxumCookie<'static>>;
    fn create_cookie(&self, ctx: &AppContext) -> CookieResult<AxumCookie<'static>>;
    fn create_cookie_strict(&self, ctx: &AppContext) -> CookieResult<AxumCookie<'static>>;
    fn user(&self) -> UserModel;
}

impl UserCookieTrait<OAuth2UserProfile> for UserModel {
    fn create_cookie_base(
        &self,
        ctx: &AppContext,
        same_site: SameSite,
    ) -> CookieResult<AxumCookie<'static>> {
        let jwt_secret = ctx
            .config
            .get_jwt_config()
            .map_err(|e| CookieError::TokenCreationError(e.to_string()))?;
        let days = 7;
        let jwt_ttl_secs = jwt_secret.expiration * days;

        let expiration_time =
            time::OffsetDateTime::now_utc() + time::Duration::seconds(jwt_ttl_secs as i64);
        let token = self
            .generate_jwt(&jwt_secret.secret, jwt_ttl_secs as u64)
            .or_else(|e| {
                tracing::error!("Failed to generate JWT: {:?}", e);
                Err(CookieError::JwtCreationError(e.to_string()))
            })?;

        let cookie = AxumCookie::build(("auth", token))
            .path("/")
            .http_only(!cfg!(debug_assertions))
            .secure(!cfg!(debug_assertions))
            .same_site(same_site)
            .expires(expiration_time)
            .max_age(time::Duration::seconds(jwt_ttl_secs as i64))
            .build();

        Ok(cookie)
    }
    // Public method for Lax cookie (e.g., for OAuth)
    fn create_cookie(&self, ctx: &AppContext) -> CookieResult<AxumCookie<'static>> {
        self.create_cookie_base(ctx, SameSite::Lax)
    }
    // Public method for Strict cookie (e.g., for standard login)
    fn create_cookie_strict(&self, ctx: &AppContext) -> CookieResult<AxumCookie<'static>> {
        self.create_cookie_base(ctx, SameSite::Strict)
    }
    fn user(&self) -> UserModel {
        self.clone()
    }
}

pub struct AppCookie;
impl CookieTrait for AppCookie {
    fn logout_cookie() -> AxumCookie<'static> {
        AxumCookie::build(("auth", ""))
            .path("/")
            .http_only(true)
            .secure(!cfg!(debug_assertions)) // true in production
            .same_site(SameSite::Lax)
            .expires(OffsetDateTime::now_utc() - Duration::days(1)) // past date
            .max_age(Duration::ZERO)
            .build()
    }

    fn create_language_cookie(lang_pref: Language) -> AxumCookie<'static> {
        AxumCookie::build(("lang", lang_pref.to_string()))
            .path("/")
            .http_only(false) // explicitly state this is readable from client if needed
            .secure(!cfg!(debug_assertions))
            .same_site(SameSite::Lax)
            .max_age(Duration::days(365)) // 1 year
            .expires(OffsetDateTime::now_utc() + Duration::days(365))
            .build()
    }
}
