use axum::http::request::Parts;
use axum::http::Uri;
use axum::{
    extract::{FromRequestParts, Request},
    response::Response,
};
use futures_util::future::BoxFuture;
use loco_rs::prelude::*;
use std::convert::Infallible;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::info;

#[derive(Clone)]
pub struct I18n;
impl I18n {
    pub fn new() -> Self {
        Self {}
    }
}
impl<S> Layer<S> for I18n {
    type Service = I18nService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service { inner }
    }
}

#[derive(Clone, Debug)]
pub struct I18nService<S> {
    inner: S,
}

#[derive(Clone)]
pub struct DetectedLanguage(pub String);
impl DetectedLanguage {
    fn is_valid_lang_code(code: &str) -> bool {
        let supported = ["de-DE", "en-US"];
        supported.contains(&code)
    }
}

impl<S, B> Service<Request<B>> for I18nService<S>
where
    S: Service<Request<B>, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static + std::fmt::Debug,
{
    type Response = S::Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        info!("[I18N MIDDLEWARE] REQ IN: {:#?}", &req);
        // This logic modifies `req` in-place.
        let mut parts = req.uri().clone().into_parts();
        if let Some(path_and_query) = parts.path_and_query.as_ref() {
            let path = path_and_query.path();
            info!("[I18N MIDDLEWARE] IN: {}", path);
            let mut segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            info!("[I18N MIDDLEWARE] IN: {:?}", &segments);
            if let Some(lang_code) = segments.first() {
                if DetectedLanguage::is_valid_lang_code(lang_code) {
                    info!("[I18N MIDDLEWARE] Found lang: {}", lang_code);
                    req.extensions_mut()
                        .insert(DetectedLanguage(lang_code.to_string()));
                    segments.remove(0);
                    let new_path = format!("/{}", segments.join("/"));
                    info!("[I18N MIDDLEWARE] OUT: {}", new_path);

                    let new_path_and_query_str = if let Some(query) = path_and_query.query() {
                        format!("{}?{}", new_path, query)
                    } else {
                        new_path
                    };
                    parts.path_and_query = Some(new_path_and_query_str.parse().unwrap());
                    *req.uri_mut() = Uri::from_parts(parts).unwrap();
                }
            }
        }

        info!("[I18N MIDDLEWARE] REQ OUT: {:#?}", &req);
        let future = self.inner.call(req);

        Box::pin(async move {
            let response: Result<Response, Infallible> = future.await;
            response
        })
    }
}

#[derive(Debug, Clone)]
pub struct LangEngine(pub String);

impl<S> FromRequestParts<S> for LangEngine
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let lang = parts
            .extensions
            .get::<DetectedLanguage>()
            .map(|d| d.0.clone())
            .unwrap_or_else(|| "en-US".to_string());

        Ok(Self(lang))
    }
}

// use axum::http::request::Parts;
// use axum::http::Uri;
// use axum::{
//     extract::{FromRequestParts, Request},
//     response::Response,
// };
// use futures_util::future::BoxFuture;
// use loco_rs::prelude::*;
// use std::convert::Infallible;
// use std::task::{Context, Poll};
// use tower::{Layer, Service};
// use tracing::info;

// use axum::body::Body;

// #[derive(Clone)]
// pub struct I18n;
// impl I18n {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl<S> Layer<S> for I18n {
//     type Service = I18nService<S>;

//     fn layer(&self, inner: S) -> Self::Service {
//         Self::Service { inner }
//     }
// }

// #[derive(Clone)]
// pub struct I18nService<S> {
//     inner: S,
// }

// #[derive(Clone)]
// pub struct DetectedLanguage(pub String);
// impl DetectedLanguage {
//     fn is_valid_lang_code(code: &str) -> bool {
//         let supported = ["en", "fr", "es", "de", "de-DE", "en-US"];
//         supported.contains(&code)
//     }
// }

// impl<S, B> Service<Request<B>> for I18nService<S>
// where
//     S: Service<Request<B>, Response = Response<Body>, Error = Infallible> + Clone + Send + 'static,
//     S::Future: Send + 'static,
//     B: Send + 'static,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }

//     fn call(&mut self, mut req: Request<B>) -> Self::Future {
//         // Your logging and rewriting logic from the previous step is PERFECT.
//         // It does not need to change.
//         info!("[I18N MIDDLEWARE] IN: {}", req.uri().path());

//         let mut parts = req.uri().clone().into_parts();
//         if let Some(path_and_query) = parts.path_and_query.as_ref() {
//             let path = path_and_query.path();
//             let mut segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

//             if let Some(lang_code) = segments.first() {
//                 if DetectedLanguage::is_valid_lang_code(lang_code) {
//                     info!("[I18N MIDDLEWARE] Found lang: {}", lang_code);
//                     req.extensions_mut()
//                         .insert(DetectedLanguage(lang_code.to_string()));
//                     segments.remove(0);
//                     let new_path = format!("/{}", segments.join("/"));
//                     info!("[I18N MIDDLEWARE] OUT: {}", new_path);

//                     let new_path_and_query_str = if let Some(query) = path_and_query.query() {
//                         format!("{}?{}", new_path, query)
//                     } else {
//                         new_path
//                     };
//                     parts.path_and_query = Some(new_path_and_query_str.parse().unwrap());
//                     *req.uri_mut() = Uri::from_parts(parts).unwrap();
//                 }
//             }
//         }

//         let clone = self.inner.clone();
//         let mut inner = std::mem::replace(&mut self.inner, clone);

//         Box::pin(async move {
//             let (parts, body) = req.into_parts();
//             info!("Request: {:?} {:?}", parts.method, parts.uri.path());
//             let req = Request::from_parts(parts, body);
//             inner.call(req).await
//         })
//     }
// }

// // fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
// //     let original_path = req.uri().path().to_string();
// //     let segments = original_path.split('/').filter(|s| !s.is_empty());
// //     let mut segments_vec: Vec<&str> = segments.collect();

// //     if let Some(first_segment) = segments_vec.first() {
// //         if DetectedLanguage::is_valid_lang_code(first_segment) {
// //             // Store detected language
// //             req.extensions_mut()
// //                 .insert(DetectedLanguage(first_segment.to_string()));

// //             // Rebuild URI without the language code
// //             segments_vec.remove(0);
// //             let new_path = format!("/{}", segments_vec.join("/"));

// //             // Rebuild URI parts
// //             let mut uri_parts = req.uri().clone().into_parts();
// //             uri_parts.path_and_query = Some(new_path.parse().unwrap());

// //             *req.uri_mut() = Uri::from_parts(uri_parts).unwrap();
// //         }
// //     }

// //     let clone = self.inner.clone();
// //     let mut inner = std::mem::replace(&mut self.inner, clone);
// //     Box::pin(async move { inner.call(req).await })
// // }

// #[derive(Debug, Clone)]
// pub struct LangEngine(pub String);

// impl<S> FromRequestParts<S> for LangEngine
// where
//     S: Send + Sync,
// {
//     type Rejection = Infallible;

//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         let lang = parts
//             .extensions
//             .get::<DetectedLanguage>()
//             .map(|d| d.0.clone())
//             .unwrap_or_else(|| "de-DE".to_string());

//         Ok(Self(lang))
//     }
// }
