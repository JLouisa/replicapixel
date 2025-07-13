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
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use strum::AsRefStr;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    domain::website::WebImages,
    models::{users::UserPid, UserModel, VideoModel, _entities::sea_orm_active_enums::Language},
    views::{images::ImageView, videos::VideoView},
};
use serde::de::DeserializeOwned;
use std::path::Path;
use tokio::fs;

pub type Cache = Arc<loco_rs::cache::Cache>;
pub type RedisDbResult<T> = std::result::Result<T, RedisDbError>;

use redis::aio::ConnectionManager;

const WEB_IMAGES_TTL_SECONDS: u64 = match !cfg!(debug_assertions) {
    true => 3600,
    false => 60,
};
const REDIS_TTL_SECONDS: u64 = match !cfg!(debug_assertions) {
    true => 3600 * 24,
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

#[derive(Debug, Clone, AsRefStr)]
pub enum RedisKey {
    User(UserPid),
    UserSetting(UserPid),
    S3PreUrl(Uuid),
    VideoPreUrl(String),
    Website(Language),
    Packs(Language),
    Pricing(Language),
    WebImages,
}
impl ImageView {
    pub fn redis_key(&self) -> RedisKey {
        RedisKey::S3PreUrl(self.pid.clone())
    }
}
impl VideoView {
    pub fn redis_key(&self) -> RedisKey {
        RedisKey::VideoPreUrl(self.uuid().to_string())
    }
}
impl VideoModel {
    pub fn redis_key(&self) -> RedisKey {
        RedisKey::VideoPreUrl(format!("video:{}", self.pid))
    }
    pub fn redis_thumbnail_key(&self) -> RedisKey {
        RedisKey::VideoPreUrl(format!("thumbnail:{}", self.pid))
    }
    pub fn redis_key_all(&self) -> Vec<RedisKey> {
        vec![self.redis_key(), self.redis_thumbnail_key()]
    }
}
impl UserModel {
    pub fn redis_user_settings_key(&self) -> RedisKey {
        let user_pid = UserPid::new(self.pid.clone());
        RedisKey::UserSetting(user_pid)
    }
    pub fn redis_user_key(&self) -> RedisKey {
        let user_pid = UserPid::new(self.pid.clone());
        RedisKey::User(user_pid)
    }
}
impl RedisKey {
    pub fn to_key(&self) -> String {
        match self {
            Self::User(uuid) => format!("user:{}", uuid.as_ref()),
            Self::UserSetting(uuid) => format!("user:setting:{}", uuid.as_ref()),
            Self::S3PreUrl(uuid) => format!("s3:preurl:{}", uuid),
            Self::VideoPreUrl(str) => format!("video:preurl:{}", str),
            Self::Website(lang) => format!("website:{}", lang),
            Self::Packs(lang) => format!("packs:{}", lang),
            Self::Pricing(lang) => format!("pricing:{}", lang),
            Self::WebImages => String::from("web:images"),
        }
    }
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
    pub async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &RedisKey,
        value: &T,
        time: Option<u64>,
    ) -> RedisDbResult<()> {
        let mut conn = self.client.clone();
        let time = time.unwrap_or(REDIS_TTL_SECONDS);
        let item = serde_json::to_string(value)?;
        let result: () = match conn.set_ex(key.to_key(), item, time).await {
            Ok(result) => result,
            Err(_) => (),
        };
        Ok(result)
    }
    pub async fn get<T: DeserializeOwned + Send>(
        &self,
        key: &RedisKey,
    ) -> RedisDbResult<Option<T>> {
        let mut conn = self.client.clone();
        let result: Option<String> = conn.get(key.to_key()).await?;
        match result {
            Some(json_str) => Ok(Some(serde_json::from_str::<T>(&json_str)?)),
            None => Ok(None),
        }
    }
    // pub async fn mget(&self, redis_keys: &Vec<RedisKey>) -> RedisDbResult<Vec<Option<String>>> {
    //     let keys: Vec<String> = redis_keys.iter().map(|k| k.to_key()).collect();
    //     let mut conn = self.client.clone();
    //     let raw_values: Vec<Option<String>> = conn.mget(keys).await?;
    //     Ok(raw_values)
    // }
    pub async fn mget(&self, redis_keys: &Vec<RedisKey>) -> RedisDbResult<Vec<Option<String>>> {
        let keys: Vec<String> = redis_keys.iter().map(|k| k.to_key()).collect();
        let mut conn = self.client.clone();

        // Try decoding into a Value first
        let raw: redis::Value = redis::cmd("MGET").arg(keys).query_async(&mut conn).await?;

        // Now convert safely to Vec<Option<String>>
        let result: Vec<Option<String>> = redis::from_redis_value(&raw)?;
        Ok(result)
    }

    pub async fn set_s3_pre_url(&self, key: &ImageView) -> RedisDbResult<()> {
        let mut conn = self.client.clone();
        let time = 60 * 60 * 23;
        let _: () = conn
            .set_ex(key.pid.to_string(), key.s3_pre_url.to_owned(), time)
            .await?;
        Ok(())
    }
    pub async fn set_s3_video_pre_url(
        &self,
        redis_keys: &RedisKey,
        s3_pre_url: &str,
    ) -> RedisDbResult<()> {
        let mut conn = self.client.clone();
        let time = 60 * 60 * 23;
        let _: () = conn.set_ex(redis_keys.to_key(), s3_pre_url, time).await?;
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
    pub async fn get_s3_pre_url_new(&self, key: &ImageView) -> RedisDbResult<String> {
        let value = self.get(&key.redis_key()).await?;
        value.ok_or(RedisDbError::NotFound)
    }
    pub async fn get_web_images(&self) -> RedisDbResult<WebImages> {
        let mut conn = self.client.clone();

        let key = RedisKey::WebImages;
        let value: Option<String> = conn.get(key.to_key()).await.map_err(RedisDbError::from)?;
        let value = match value {
            Some(web) => web,
            None => return Err(RedisDbError::NotFound),
        };
        let web = serde_json::from_str(&value)?;
        Ok(web)
    }
    pub async fn set_web_images(&self, web: &WebImages) -> RedisDbResult<()> {
        let mut conn = self.client.clone();

        let key = RedisKey::WebImages;
        let value = serde_json::to_string(web)?;
        let _: () = conn
            .set_ex(key.to_key(), value, WEB_IMAGES_TTL_SECONDS)
            .await?;
        Ok(())
    }
}

// async fn fetch_and_cache_video(
//     ctx: &AppContext,
//     cache: &RedisCacheDriver,

// ) -> CacheResult<WebImages> {
//     let video = Webvideo::web_video(&ctx.db, lang, &cache).await;
//     match serde_json::to_string(&video) {
//         Ok(serialized) => {
//             if let Err(e) = ctx
//                 .cache
//                 .insert_with_expiry(
//                     &key.to_key(),
//                     &serialized,
//                     Duration::from_secs(WEB_video_TTL_SECONDS),
//                 )
//                 .await
//             {
//                 tracing::error!("Failed to write web video to cache: {}", e);
//             }
//         }
//         Err(e) => {
//             tracing::error!("Failed to serialize web video: {}", e);
//         }
//     }
//     Ok(video)
// }

async fn fetch_and_cache_web_images(
    ctx: &AppContext,
    lang: &Language,
    key: &RedisKey,
    cache: &RedisCacheDriver,
) -> CacheResult<WebImages> {
    let images = WebImages::web_images(&ctx.db, lang, &cache).await;
    match serde_json::to_string(&images) {
        Ok(serialized) => {
            if let Err(e) = ctx
                .cache
                .insert_with_expiry(
                    &key.to_key(),
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
pub async fn load_cached_web(
    ctx: &AppContext,
    lang: &Language,
    cache: &RedisCacheDriver,
) -> CacheResult<WebImages> {
    let key = RedisKey::Website(lang.to_owned());
    match ctx.cache.get(&key.to_key()).await {
        Ok(Some(cached)) => match serde_json::from_str::<WebImages>(&cached) {
            Ok(data) => Ok(data),
            Err(err) => {
                tracing::error!("Failed to deserialize cached web images: {}", err);
                fetch_and_cache_web_images(ctx, lang, &key, &cache).await
            }
        },
        Ok(None) => {
            // tracing::info!("Web images not found in cache, loading from DB.");
            fetch_and_cache_web_images(ctx, lang, &key, &cache).await
        }
        Err(err) => {
            tracing::error!("Failed to read from cache: {}", err);
            fetch_and_cache_web_images(ctx, lang, &key, &cache).await
        }
    }
}

pub async fn load_from_file_and_cache(
    ctx: &AppContext,
    path: &Path,
    key: &str,
) -> Result<String, std::io::Error> {
    let file = match fs::read_to_string(path).await {
        Ok(serialized) => {
            if let Err(e) = ctx
                .cache
                .insert_with_expiry(key, &serialized, Duration::from_secs(60))
                .await
            {
                tracing::error!("Failed to write web images to cache: {}", e);
            }
            serialized
        }
        Err(e) => return Err(e),
    };
    Ok(file)
}
