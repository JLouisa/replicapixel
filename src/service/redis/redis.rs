use async_trait::async_trait;
use derive_more::Constructor;
use loco_rs::{
    app::AppContext,
    cache::{drivers::CacheDriver, CacheError, CacheResult},
};
use redis::{
    aio::ConnectionManagerConfig,
    io::tcp::{socket2::TcpKeepalive, TcpSettings},
    AsyncCommands, Client, IntoConnectionInfo, RedisResult,
};
use serde::Deserialize;
use std::{sync::Arc, time::Duration};
use strum::{AsRefStr, EnumString};
use thiserror::Error;

use crate::{controllers::home::WebImages, views::images::ImageView};

pub type Cache = Arc<loco_rs::cache::Cache>;
pub type RedisDbResult<T> = std::result::Result<T, RedisDbError>;

use redis::aio::ConnectionManager;

const WEB_IMAGES_CACHE_KEY: &str = "web";
const WEB_IMAGES_TTL_SECONDS: u64 = match !cfg!(debug_assertions) {
    true => 3600,
    false => 60,
};

#[derive(Debug, Error)]
pub enum RedisDbError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Invalid database alias")]
    InvalidDatabaseAlias,
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Ping failed: {0}")]
    PingFailed(String),
    #[error("Set value failed")]
    SetValueFailed,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Not found")]
    NotFound,
    #[error("Cache error: {0}")]
    CacheError(#[from] CacheError),
    #[error("Conversion Error: {0}")]
    ConversionError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, EnumString, AsRefStr)]
pub enum RedisKey {
    PasswordReset,
    Verification,
    S3PreUrl,
    Session,
    User,
    Queue,
}

#[derive(Clone, Debug, Deserialize, Constructor)]
pub struct RedisSettings {
    pub redis_url: String,
}

#[derive(Clone)]
pub struct RedisCacheDriver {
    client: ConnectionManager,
    settings: RedisSettings,
}

impl RedisCacheDriver {
    pub async fn new(config: &RedisSettings) -> RedisResult<Self> {
        let manager = Self::connect_with_manager(config).await?;
        Ok(Self {
            client: manager,
            settings: config.clone(),
        })
    }
    async fn connect_with_manager(
        redis_settings: &RedisSettings,
    ) -> RedisResult<ConnectionManager> {
        let keep_alive_settings = TcpKeepalive::new()
            .with_time(Duration::from_secs(60))
            .with_interval(Duration::from_secs(15))
            .with_retries(5);
        let tcp_settings = TcpSettings::default().set_keepalive(keep_alive_settings);
        let config = ConnectionManagerConfig::new().set_tcp_settings(tcp_settings);
        let connection_info = redis_settings.redis_url.as_str().into_connection_info()?;
        let client = Client::open(connection_info)?;
        let manager = ConnectionManager::new_with_config(client, config).await?;
        Ok(manager)
    }
    pub fn redis_settings(&self) -> &RedisSettings {
        &self.settings
    }
}

#[async_trait]
impl CacheDriver for RedisCacheDriver {
    async fn contains_key(&self, key: &str) -> CacheResult<bool> {
        let mut conn = self.client.clone();
        let exists: bool = match conn.exists(key).await {
            Ok(exists) => exists,
            Err(_) => false,
        };
        Ok(exists)
    }
    async fn get(&self, key: &str) -> CacheResult<Option<String>> {
        let mut conn = self.client.clone();
        let result: Option<String> = match conn.get(key).await {
            Ok(result) => result,
            Err(_) => None,
        };
        Ok(result)
    }
    async fn insert(&self, key: &str, value: &str) -> CacheResult<()> {
        let mut conn = self.client.clone();
        let result: () = match conn.set(key, value).await {
            Ok(result) => result,
            Err(_) => (),
        };
        Ok(result)
    }
    async fn insert_with_expiry(
        &self,
        key: &str,
        value: &str,
        duration: Duration,
    ) -> CacheResult<()> {
        let mut conn = self.client.clone();
        let ttl_secs = duration.as_secs() as usize;
        let result: () = match conn.set_ex(key, value, ttl_secs as u64).await {
            Ok(result) => result,
            Err(_) => (),
        };
        Ok(result)
    }
    async fn remove(&self, key: &str) -> CacheResult<()> {
        let mut conn = self.client.clone();
        let result: () = match conn.del(key).await {
            Ok(result) => result,
            Err(_) => (),
        };
        Ok(result)
    }
    async fn clear(&self) -> CacheResult<()> {
        let mut conn = self.client.clone();
        let result: () = match conn.flushdb().await {
            Ok(result) => result,
            Err(_) => (),
        };
        Ok(result)
    }
}

impl RedisCacheDriver {
    pub async fn ping_redis(&self) -> RedisDbResult<()> {
        let mut conn = self.client.clone();
        let cmd = redis::cmd("PING");
        let result: Result<String, _> = cmd.query_async(&mut conn).await;
        match result {
            Ok(response) => {
                if response == "PONG" {
                    Ok(())
                } else {
                    Err(RedisDbError::PingFailed(format!(
                        "Unexpected PING responseReceived: '{}'. Expected 'PONG'",
                        response
                    )))
                }
            }
            Err(e) => Err(RedisDbError::RedisError(e)),
        }
    }
    pub async fn set_s3_pre_url(&self, key: &ImageView) -> RedisDbResult<()> {
        let mut conn = self.client.clone();
        let time = 60 * 60 * 23;
        let _: () = conn
            .set_ex(key.pid.to_string(), key.s3_pre_url.to_owned(), time)
            .await?;
        Ok(())
    }
    pub async fn get_s3_pre_url(&self, key: &ImageView) -> RedisDbResult<String> {
        let mut conn = self.client.clone();

        let value: Option<String> = conn
            .get(key.pid.to_string())
            .await
            .map_err(RedisDbError::from)?;
        value.ok_or(RedisDbError::NotFound)
    }
    pub async fn get_web_images(&self) -> RedisDbResult<WebImages> {
        let mut conn = self.client.clone();

        let value: Option<String> = conn
            .get(WEB_IMAGES_CACHE_KEY.to_owned())
            .await
            .map_err(RedisDbError::from)?;
        let value = match value {
            Some(web) => web,
            None => return Err(RedisDbError::NotFound),
        };
        let web = serde_json::from_str(&value)?;
        Ok(web)
    }
    pub async fn set_web_images(&self, web: &WebImages) -> RedisDbResult<()> {
        let mut conn = self.client.clone();

        let value = serde_json::to_string(web)?;
        let _: () = conn
            .set_ex(
                WEB_IMAGES_CACHE_KEY.to_owned(),
                value,
                WEB_IMAGES_TTL_SECONDS,
            )
            .await?;
        Ok(())
    }
}

async fn fetch_and_cache_web_images(ctx: &AppContext) -> CacheResult<WebImages> {
    let images = WebImages::web_images(&ctx.db).await;
    match serde_json::to_string(&images) {
        Ok(serialized) => {
            if let Err(e) = ctx
                .cache
                .insert_with_expiry(
                    "web",
                    &serialized,
                    Duration::from_secs(WEB_IMAGES_TTL_SECONDS),
                )
                .await
            {
                tracing::error!("Failed to write web images to cache: {}", e);
            }
        }
        Err(e) => {
            tracing::error!("Failed to serialize web images: {}", e);
        }
    }
    Ok(images)
}
pub async fn load_cached_web(ctx: &AppContext) -> CacheResult<WebImages> {
    match ctx.cache.get("web").await {
        Ok(Some(cached)) => match serde_json::from_str::<WebImages>(&cached) {
            Ok(data) => Ok(data),
            Err(err) => {
                tracing::error!("Failed to deserialize cached web images: {}", err);
                fetch_and_cache_web_images(ctx).await
            }
        },
        Ok(None) => {
            tracing::info!("Web images not found in cache, loading from DB.");
            fetch_and_cache_web_images(ctx).await
        }
        Err(err) => {
            tracing::error!("Failed to read from cache: {}", err);
            fetch_and_cache_web_images(ctx).await
        }
    }
}
