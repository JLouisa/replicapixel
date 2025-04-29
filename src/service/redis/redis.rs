use derive_more::Constructor;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, RedisResult};
use serde::Deserialize;
use strum::{AsRefStr, EnumString};
use thiserror::Error;

use crate::views::images::{ImageView, ImageViewList};

pub type Cache = Redis;

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

#[derive(Debug, Clone)]
pub struct Redis {
    client: MultiplexedConnection,
}

impl Redis {
    pub async fn new(config: &RedisSettings) -> RedisResult<Self> {
        let client_instance = Client::open(config.redis_url.as_str())?;
        let connection = client_instance
            .get_multiplexed_tokio_connection() // Use the tokio version
            .await?;
        Ok(Redis { client: connection })
    }
    /// Sets the value of a key.
    pub async fn set(&self, key: &str, value: &str, seconds: Option<usize>) -> RedisResult<()> {
        let mut conn = self.client.clone();
        if let Some(timeout) = seconds {
            let _: () = conn.set_ex(key, value, timeout as u64).await?;
        } else {
            let _: () = conn.set(key, value).await?;
        }
        Ok(())
    }
    /// Gets the value of a key.
    pub async fn get(&self, key: &str) -> RedisResult<String> {
        let mut conn = self.client.clone();
        let value: String = conn.get(key).await?;
        Ok(value)
    }
}

impl Cache {
    pub async fn set_s3_pre_url(&self, key: &ImageView) -> Result<(), RedisDbError> {
        let mut conn = self.client.clone();
        let _: () = conn
            .set_ex(key.pid.to_string(), key.s3_pre_url.to_owned(), 60 * 60 * 23)
            .await?;
        Ok(())
    }
    pub async fn get_s3_pre_url(&self, key: &ImageView) -> Result<String, RedisDbError> {
        let mut conn = self.client.clone();

        let value: Option<String> = conn
            .get(key.pid.to_string())
            .await
            .map_err(RedisDbError::from)?;

        value.ok_or(RedisDbError::NotFound)
    }
    // pub async fn populate_s3_pre_urls(
    //     &self,
    //     items: &mut ImageViewList,
    // ) -> Result<(), RedisDbError> {
    //     let mut conn = self.client.clone();

    //     for item in items.as_mut_vec().iter_mut() {
    //         let url: String = conn.get(item.pid.to_string()).await?;
    //         item.s3_pre_url = Some(url);
    //     }

    //     Ok(())
    // }
}
