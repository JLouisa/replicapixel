use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    http,
    response::Response,
};
use cookie::CookieJar;
use futures_util::future::BoxFuture;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use urlencoding;

pub const CONSENT_COOKIE_NAME_V3: &str = "cc_cookie";

#[derive(Clone, Debug, Serialize)]
pub struct CookieConsent {
    pub cookie_exists: bool,
    pub necessary: bool,
    pub analytics: bool,
    pub marketing: bool,
    // Add other fields if needed (e.g., consentId, timestamp)
}
impl Default for CookieConsent {
    fn default() -> Self {
        CookieConsent {
            cookie_exists: false,
            necessary: true,
            analytics: false,
            marketing: false,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CookieConsentData {
    categories: Option<Vec<String>>,
    revision: Option<u32>,
    data: Option<serde_json::Value>,
    consent_timestamp: Option<String>,
    consent_id: Option<String>,
    services: Option<HashMap<String, Vec<String>>>,
    language_code: Option<String>,
    last_consent_timestamp: Option<String>,
    expiration_time: Option<i64>,
}

#[derive(Clone)]
pub struct CookieConsentLayer;
impl CookieConsentLayer {
    pub fn new() -> Self {
        Self {}
    }
}
impl<S> Layer<S> for CookieConsentLayer {
    type Service = CookieConsentService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CookieConsentService { inner }
    }
}

#[derive(Clone)]
pub struct CookieConsentService<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for CookieConsentService<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let jar = CookieJar::from_headers(req.headers());
        let mut consent_state = CookieConsent::default();

        if let Some(cookie) = jar.get(CONSENT_COOKIE_NAME_V3) {
            consent_state.cookie_exists = true;
            let encoded_value = cookie.value();

            match urlencoding::decode(encoded_value) {
                Ok(decoded_value) => {
                    match serde_json::from_str::<CookieConsentData>(&decoded_value) {
                        Ok(data) => {
                            tracing::debug!("Parsed CookieConsentData: {:?}", data);
                            consent_state.necessary = true;

                            if let Some(categories) = data.categories {
                                consent_state.analytics =
                                    categories.contains(&"analytics".to_string());
                                consent_state.marketing =
                                    categories.contains(&"marketing".to_string());
                            } else {
                                tracing::warn!("Cookie '{}' parsed but 'categories' field missing. Assuming necessary only.", CONSENT_COOKIE_NAME_V3);
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse JSON from cookie '{}' after decoding: {}. Value: '{}'. Assuming necessary only.",
                                CONSENT_COOKIE_NAME_V3, e, decoded_value
                            );
                            consent_state.analytics = false;
                            consent_state.marketing = false;
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to URL-decode cookie '{}': {}. Raw value: '{}'. Treating as no consent.",
                        CONSENT_COOKIE_NAME_V3, e, encoded_value
                    );
                    dbg!(&e);
                    dbg!(&encoded_value);
                    consent_state.cookie_exists = false;
                }
            }
        }
        req.extensions_mut().insert(consent_state);

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move { inner.call(req).await })
    }
}

#[derive(Debug, Clone)]
pub struct ExtractConsentState(pub CookieConsent);

impl<S> FromRequestParts<S> for ExtractConsentState
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = parts
            .extensions
            .get::<CookieConsent>()
            .cloned()
            .unwrap_or_default();
        Ok(ExtractConsentState(state))
    }
}
