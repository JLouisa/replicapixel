// use redis::aio::MultiplexedConnection;
// use redis::Client;
// use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
// use secrecy::ExposeSecret;
// use serde_aux::field_attributes::deserialize_number_from_string;
// use std::fmt;
// use std::fmt::Debug;
// use strum::EnumString;
// use strum_macros::{AsRefStr, Display};
// use thiserror::Error;
// use tokio::sync::RwLock;

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

// #[derive(serde::Deserialize, Debug, Clone)]
// pub struct RedisSettings {
//     pub redis_url: String,
//     pub username: Option<String>,
//     pub password: Option<String>,
//     pub database_name: String,
// }

// #[derive(Debug, Display, Clone, EnumString, AsRefStr)]
// pub enum RedisKey {
//     PasswordReset,
//     Verification,
//     Session,
//     User,
//     S3PreUrl,
//     Queue,
// }

// #[derive(Debug, Display, AsRefStr)]
// pub enum RedisCmd {
//     AUTH,
//     PING,
//     PONG,
//     HMSET,
//     RPOP,
//     RPUSH,
//     LPUSH,
//     LPOP,
// }

// #[derive(Debug)]
// pub struct Redis {
//     pub client: RwLock<MultiplexedConnection>,
// }

// impl Redis {
//     pub async fn new(config: &RedisSettings) -> Result<Self, RedisDbError> {
//         // Check if both username and password are provided, else leave auth_part empty
//         let auth_part =
//             if let (Some(username), Some(password)) = (&config.username, &config.password) {
//                 format!("{}:{}@", username, password)
//             } else {
//                 String::new()
//             };

//         let con: Client = Client::open(config.redis_url.to_owned()).map_err(RedisDbError::from)?;
//         let mut client: MultiplexedConnection = con
//             .get_multiplexed_tokio_connection()
//             .await
//             .map_err(RedisDbError::from)?;

//         // // Authenticate if a password is provided
//         // if let (Some(username), Some(password)) = (&config.username, &config.password) {
//         //     redis::cmd(RedisCmd::AUTH.as_ref())
//         //         .arg(username)
//         //         .arg(password)
//         //         .query_async::<_, ()>(&mut client)
//         //         .await
//         //         .map_err(|_| RedisDbError::AuthenticationFailed)?;
//         // }

//         Ok(Redis {
//             client: RwLock::new(client),
//         })
//     }

//     // Redis health check method
//     pub async fn check_health(&self) -> Result<(), RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         let pong: String = redis::cmd(RedisCmd::PING.as_ref())
//             .query_async(&mut *write_guard)
//             .await
//             .map_err(|e| RedisDbError::PingFailed(e.to_string()))?;

//         println!("{}", &pong);

//         if pong == RedisCmd::PONG.as_ref() {
//             Ok(())
//         } else {
//             Err(RedisDbError::PingFailed("Didn't receive Pong".to_string()))
//         }
//     }

//     // Set a value by key
//     pub async fn set_value(&mut self, key: &RedisKey, value: &str) -> Result<(), RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         write_guard.set(key.to_string(), value).await?;
//         drop(write_guard);

//         Ok(())
//     }

//     // Get a value by key
//     pub async fn get_value(&mut self, key: &RedisKey) -> Result<String, RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         let result: String = write_guard.get(key.to_string()).await?;
//         drop(write_guard);

//         Ok(result)
//     }

//     // Increment a value by key
//     pub async fn incr(&mut self, key: &RedisKey, increment: isize) -> Result<isize, RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         let result: isize = write_guard.incr(key.to_string(), increment).await?;
//         drop(write_guard);

//         Ok(result)
//     }

//     // Get a value by key
//     pub async fn delete_key(&mut self, key: &RedisKey) -> Result<bool, RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         let result: isize = write_guard.del(key.to_string()).await?;
//         drop(write_guard);

//         Ok(result > 0)
//     }

//     // Add a value to a set
//     pub async fn sadd(&mut self, key: &RedisKey, value: &str) -> Result<(), RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         write_guard.sadd(key.to_string(), value).await?;
//         drop(write_guard);

//         Ok(())
//     }

//     // Set a field in a hash
//     pub async fn hset(
//         &mut self,
//         key: &RedisKey,
//         field: &str,
//         value: &str,
//     ) -> Result<(), RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         write_guard.hset(key.to_string(), field, value).await?;
//         drop(write_guard);

//         Ok(())
//     }

//     // Set multiple fields in a hash
//     pub async fn hset_multiple<
//         'a,
//         K: ToRedisArgs + Send + Sync + 'a,
//         F: ToRedisArgs + Send + Sync + 'a,
//         V: ToRedisArgs + Send + Sync + 'a,
//         RV,
//     >(
//         &'a mut self,
//         key: K,
//         items: &'a [(F, V)],
//     ) -> Result<(), RedisDbError>
//     where
//         RV: FromRedisValue,
//     {
//         let mut write_guard = self.client.write().await;
//         let mut cmd = redis::cmd(RedisCmd::HMSET.as_ref());

//         cmd.arg(key);
//         for (field, value) in items {
//             cmd.arg(field).arg(value);
//         }
//         cmd.query_async(&mut *write_guard).await?;

//         drop(write_guard);
//         Ok(())
//     }

//     // Get a value from a hash field
//     pub async fn hget(
//         &mut self,
//         key: &RedisKey,
//         field: &str,
//     ) -> Result<Option<String>, RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         let result: Option<String> = write_guard.hget(key.to_string(), field).await?;
//         drop(write_guard);

//         Ok(result)
//     }

//     // Get all members of a set
//     pub async fn smembers(&mut self, key: &RedisKey) -> Result<Vec<String>, RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         let members: Vec<String> = write_guard.smembers(key.to_string()).await?;
//         drop(write_guard);

//         Ok(members)
//     }

//     // Remove a value from a set
//     pub async fn srem(&mut self, key: &RedisKey, value: &str) -> Result<(), RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         write_guard.srem(key.to_string(), value).await?;
//         drop(write_guard);

//         Ok(())
//     }

//     // Push to the stack
//     pub async fn low_lpush(&mut self, key: &RedisKey, value: &str) -> Result<(), RedisDbError> {
//         let mut write_guard = self.client.write().await;

//         redis::cmd(RedisCmd::LPUSH.as_ref())
//             .arg(key.to_string())
//             .arg(value)
//             .query_async(&mut *write_guard)
//             .await?;
//         drop(write_guard);

//         Ok(())
//     }

//     // Pop from the stack
//     pub async fn low_rpop(&mut self, key: &RedisKey) -> Result<String, RedisDbError> {
//         let mut write_guard = self.client.write().await;

//         let result = redis::cmd(RedisCmd::RPOP.as_ref())
//             .arg(key.to_string())
//             .query_async(&mut *write_guard)
//             .await?;
//         drop(write_guard);

//         Ok(result)
//     }

//     // Enqueue a value to the back of the queue
//     pub async fn low_enqueue(&mut self, key: &RedisKey, value: &str) -> Result<(), RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         redis::cmd(RedisCmd::RPUSH.as_ref())
//             .arg(key.to_string())
//             .arg(value)
//             .query_async(&mut *write_guard)
//             .await?;

//         drop(write_guard);
//         Ok(())
//     }

//     // Dequeue a value from the front of the queue
//     pub async fn low_dequeue(&mut self, key: &RedisKey) -> Result<String, RedisDbError> {
//         let mut write_guard = self.client.write().await;
//         let result = redis::cmd(RedisCmd::LPOP.as_ref())
//             .arg(key.to_string())
//             .query_async(&mut *write_guard)
//             .await?;
//         drop(write_guard);
//         Ok(result)
//     }
// }

// //? Run cargo test redis_setup
// // #[cfg(test)]
// // mod redis_setup {
// //     use crate::{
// //         data::redis::redis::{RedisDbAsync, RedisKey},
// //         startup::get_redis_url,
// //         tests::helpers::spawn_app,
// //     };

// //     #[tokio::test]
// //     async fn redis_operations_set_get() {
// //         // Spawn the app for the test
// //         let app = spawn_app().await;

// //         let redis_url = get_redis_url(&app.redis_settings);
// //         println!("Redis URL: {}", &redis_url);

// //         // Setup Redis Connection
// //         let mut client = RedisDbAsync::new(&redis_url, &app.redis_settings)
// //             .await
// //             .unwrap();

// //         let test_key = RedisKey::Test("test2");

// //         // Set and get a string value
// //         client
// //             .set_value(&test_key, "testing2")
// //             .await
// //             .expect("Failed to set string value");

// //         let value: String = client
// //             .get_value(&test_key)
// //             .await
// //             .expect("Failed to get string value");

// //         assert_eq!(value, "testing2", "String value does not match");
// //     }

// //     #[tokio::test]
// //     async fn redis_operations_counter() {
// //         // Spawn the app for the test
// //         let app = spawn_app().await;

// //         let redis_url = get_redis_url(&app.redis_settings);

// //         // Setup Redis Connection
// //         let mut client = RedisDbAsync::new(&redis_url, &app.redis_settings)
// //             .await
// //             .unwrap();

// //         let counter_key = RedisKey::Test("counter");

// //         // Set and increment an integer value
// //         client
// //             .set_value(&counter_key, "100")
// //             .await
// //             .expect("Failed to set integer value");
// //         client
// //             .incr(&counter_key, 1)
// //             .await
// //             .expect("Failed to increment value");
// //         let counter = client
// //             .get_value(&counter_key)
// //             .await
// //             .expect("Failed to get incremented value");

// //         assert_eq!(counter, "101", "Counter value does not match");
// //     }

// //     #[tokio::test]
// //     async fn redis_operations_sadd_smembers() {
// //         // Spawn the app for the test
// //         let app = spawn_app().await;

// //         let redis_url = get_redis_url(&app.redis_settings);

// //         // Setup Redis Connection
// //         let mut client = RedisDbAsync::new(&redis_url, &app.redis_settings)
// //             .await
// //             .unwrap();

// //         let myset_key = RedisKey::Test("myset");

// //         // Add elements to a set and verify
// //         client
// //             .sadd(&myset_key, "member1")
// //             .await
// //             .expect("Failed to add to set");
// //         client
// //             .sadd(&myset_key, "member2")
// //             .await
// //             .expect("Failed to add to set");

// //         // Get and verify members
// //         let members: Vec<String> = client
// //             .smembers(&myset_key)
// //             .await
// //             .expect("Failed to get members");

// //         assert!(
// //             members.contains(&"member1".to_string()) && members.contains(&"member2".to_string()),
// //             "Set members do not match"
// //         );
// //     }

// //     #[tokio::test]
// //     async fn redis_operations_hset_hget() {
// //         // Spawn the app for the test
// //         let app = spawn_app().await;

// //         let redis_url = get_redis_url(&app.redis_settings);

// //         // Setup Redis Connection
// //         let mut client = RedisDbAsync::new(&redis_url, &app.redis_settings)
// //             .await
// //             .unwrap();

// //         let user_1000_key = RedisKey::Test("user:1000");

// //         // Set fields in a hash and verify
// //         client
// //             .hset(&user_1000_key, "name", "Alice")
// //             .await
// //             .expect("Failed to set hash value");
// //         client
// //             .hset(&user_1000_key, "job", "Engineer")
// //             .await
// //             .expect("Failed to set hash value");
// //         let name: Option<String> = client
// //             .hget(&user_1000_key, "name")
// //             .await
// //             .expect("Failed to get hash value");
// //         let job: Option<String> = client
// //             .hget(&user_1000_key, "job")
// //             .await
// //             .expect("Failed to get hash value");

// //         assert_eq!(
// //             name,
// //             Some("Alice".to_string()),
// //             "Hash name value does not match"
// //         );

// //         assert_eq!(
// //             job,
// //             Some("Engineer".to_string()),
// //             "Hash job value does not match"
// //         );
// //     }
// // }
