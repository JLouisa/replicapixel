use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ClientIp(pub Option<String>);

impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let ip = parts
            .headers
            .get("cf-connecting-ip")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim().to_string())
            .or_else(|| {
                parts
                    .headers
                    .get("x-forwarded-for")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.split(',').next())
                    .map(|s| s.trim().to_string())
            })
            .or_else(|| {
                parts
                    .headers
                    .get("x-real-ip")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.trim().to_string())
            })
            .or_else(|| {
                parts
                    .extensions
                    .get::<std::net::SocketAddr>()
                    .map(|addr| addr.ip().to_string())
            });

        Ok(ClientIp(ip))
    }
}

// use axum::{extract::FromRequestParts, http::request::Parts};
// use std::convert::Infallible;

// #[derive(Debug, Clone)]
// pub struct ClientIp(pub String);

// impl<S> FromRequestParts<S> for ClientIp
// where
//     S: Send + Sync,
// {
//     type Rejection = Infallible;

//     async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
//         let ip = parts
//             .headers
//             // 1. Cloudflare
//             .get("cf-connecting-ip")
//             .and_then(|v| v.to_str().ok())
//             .map(|s| s.trim().to_string())
//             // 2. X-Forwarded-For
//             .or_else(|| {
//                 parts
//                     .headers
//                     .get("x-forwarded-for")
//                     .and_then(|v| v.to_str().ok())
//                     .and_then(|s| s.split(',').next())
//                     .map(|s| s.trim().to_string())
//             })
//             // 3. X-Real-IP
//             .or_else(|| {
//                 parts
//                     .headers
//                     .get("x-real-ip")
//                     .and_then(|v| v.to_str().ok())
//                     .map(|s| s.trim().to_string())
//             })
//             .unwrap_or_else(|| "unknown".to_string());

//         Ok(ClientIp(ip))
//     }
// }
