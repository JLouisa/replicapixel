use crate::models::_entities::sea_orm_active_enums::Language;
use accept_language::intersection;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::CookieJar;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LangEngine(pub Language);

const SUPPORTED_LANGUAGES: &[&str] = &["en-US", "de-DE", "es-ES", "it-IT", "nl-NL"];

impl<S> FromRequestParts<S> for LangEngine
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // This is safe because the Rejection type is Infallible.
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();

        // 1. The Cookie is the highest priority.
        if let Some(cookie) = jar.get("lang") {
            // Try to parse the cookie's value directly into our Language enum.
            if let Ok(lang) = Language::from_str(cookie.value()) {
                return Ok(Self(lang));
            }
        }

        // 2. If no cookie, check the Accept-Language header.
        if let Some(accept_language_value) = parts.headers.get("accept-language") {
            if let Ok(header_str) = accept_language_value.to_str() {
                let common = intersection(header_str, SUPPORTED_LANGUAGES);
                if let Some(lang) = common.first() {
                    if let Ok(lang) = Language::from_str(lang) {
                        return Ok(Self(lang));
                    }
                }
            }
        }

        // 3. If all else fails, use the hardcoded default.
        Ok(Self(Language::default()))
    }
}
