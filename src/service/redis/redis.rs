use chrono::Utc;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, RedisResult};
use strum::{AsRefStr, EnumString};
use thiserror::Error;

use crate::{models::ImageModel, views::images::ImageViewModel};

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

#[derive(Clone, Debug)]
pub struct RedisSettings {
    pub redis_url: String,
}

#[derive(Clone, Debug)]
pub struct RedisClientOptions {
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
    pub async fn set_s3_pre_url(&self, key: &ImageViewModel) -> Result<(), RedisDbError> {
        let mut conn = self.client.clone();
        let _: () = conn
            .set_ex(key.pid.to_string(), key.s3_pre_url.to_owned(), 60 * 60 * 24)
            .await?;
        Ok(())
    }
    pub async fn get_s3_pre_url(&self, key: &ImageViewModel) -> Result<String, RedisDbError> {
        let mut conn = self.client.clone();
        let value: String = conn.get(key.pid.to_string()).await?;
        Ok(value)
    }
}

// use redis::{Client, Commands, RedisResult};
// use std::time::Duration;
// use strum::{AsRefStr, EnumString};
// use thiserror::Error;

// pub type Cache = Redis;

// #[derive(Debug, Error)]
// pub enum RedisDbError {
//     #[error("Redis error: {0}")]
//     RedisError(#[from] redis::RedisError),
//     #[error("Connection error: {0}")]
//     ConnectionError(String),
//     #[error("Invalid database alias")]
//     InvalidDatabaseAlias,
//     #[error("Connection failed")]
//     ConnectionFailed,
//     #[error("Ping failed: {0}")]
//     PingFailed(String),
//     #[error("Set value failed")]
//     SetValueFailed,
//     #[error("Authentication failed")]
//     AuthenticationFailed,
// }

// #[derive(Debug, Clone, EnumString, AsRefStr)]
// pub enum RedisKey {
//     PasswordReset,
//     Verification,
//     S3PreUrl,
//     Session,
//     User,
//     Queue,
// }

// #[derive(Clone, Debug)]
// pub struct RedisClientOptions {
//     pub redis_url: String,
// }

// #[derive(Debug)]
// pub struct Redis {
//     client: Client,
// }

// impl Redis {
//     pub fn new(options: RedisClientOptions) -> RedisResult<Self> {
//         let client = Client::open(options.redis_url)?;
//         Ok(Redis { client })
//     }
//     fn get_connection(&self) -> RedisResult<redis::Connection> {
//         self.client.get_connection()
//     }
//     pub fn set(&self, key: &str, value: &str, timeout_seconds: usize) -> RedisResult<()> {
//         let mut conn = self.get_connection()?;
//         let _: () = conn.set_ex(key, value, timeout_seconds as u64)?;
//         Ok(())
//     }
//     pub fn get(&self, key: &str) -> RedisResult<String> {
//         let mut conn = self.get_connection()?;
//         let value: String = conn.get(key)?;
//         Ok(value)
//     }
// }

// // Example Usage Function
// fn redis_start() {
//     println!("Attempting to connect to Redis...");
//     let redis_config = RedisClientOptions {
//         redis_url: "redis://localhost:6379".to_string(),
//     };

//     // Use if let for cleaner success/error handling
//     match Redis::new(redis_config) {
//         Ok(redis) => {
//             println!("Redis client created successfully.");

//             let key = "my_simple_key";
//             let value = "Hello Redis!";
//             let timeout = 60; // seconds

//             // Set Key
//             println!(
//                 "Setting key '{}' with value '{}' ({}s timeout)...",
//                 key, value, timeout
//             );
//             match redis.set(key, value, timeout) {
//                 Ok(_) => println!(" -> OK: Key set successfully."),
//                 // Use eprintln! for errors
//                 Err(e) => eprintln!(" -> ERROR: Failed to set key: {}", e),
//             }

//             // Get Key
//             println!("Getting key '{}'...", key);
//             match redis.get(key) {
//                 Ok(retrieved_value) => println!(" -> OK: Got value: '{}'", retrieved_value),
//                 Err(e) => eprintln!(" -> ERROR: Failed to get key: {}", e),
//             }

//             // Get a non-existent key (example of error)
//             let missing_key = "does_not_exist";
//             println!("Getting missing key '{}'...", missing_key);
//             match redis.get(missing_key) {
//                 Ok(retrieved_value) => {
//                     println!(" -> UNEXPECTED OK: Got value: '{}'", retrieved_value)
//                 }
//                 Err(e) => eprintln!(" -> EXPECTED ERROR: Failed to get key: {}", e), // This error is expected
//             }

//             println!("Example finished. Redis client will be dropped now.");
//             // No need to call close_connection explicitly
//         }
//         Err(e) => {
//             eprintln!("ERROR: Failed to create Redis client: {}", e);
//         }
//     }
// }

// // Main function to run the example
// fn main() {
//     redis_start();
// }
