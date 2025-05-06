use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, Condition};

// src/models/o_auth2_sessions.rs
pub use super::_entities::o_auth2_sessions::{self, ActiveModel, Entity, Model};
use super::users;
use async_trait::async_trait;
use loco_oauth2::{
    base_oauth2::basic::BasicTokenResponse, base_oauth2::TokenResponse,
    models::oauth2_sessions::OAuth2SessionsTrait,
};

use chrono::Utc;

#[async_trait]
impl OAuth2SessionsTrait<users::Model> for Model {
    /// Check if a session is expired from the database
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// session_id: &str - Session id
    /// # Returns
    /// A boolean
    /// # Errors
    /// Returns a `ModelError` if the session is not found
    async fn is_expired(db: &DatabaseConnection, session_id: &str) -> ModelResult<bool> {
        let oauth2_session = o_auth2_sessions::Entity::find()
            .filter(o_auth2_sessions::Column::SessionId.eq(session_id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        Ok(oauth2_session.expires_at < Utc::now())
    }

    /// Upsert a session with OAuth
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// token: &`BasicTokenResponse` - OAuth token
    /// user: &`users::Model` - User
    /// # Returns
    /// A session
    /// # Errors
    /// Returns a `ModelError` if the session cannot be upserted
    async fn upsert_with_oauth2(
        db: &DatabaseConnection,
        token: &BasicTokenResponse,
        user: &users::Model,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let oauth2_session_id = token.access_token().secret().clone();
        let oauth2_session = match o_auth2_sessions::Entity::find()
            .filter(o_auth2_sessions::Column::UserId.eq(user.id))
            .one(&txn)
            .await?
        {
            Some(oauth2_session) => {
                // Update the session
                let mut oauth2_session: o_auth2_sessions::ActiveModel = oauth2_session.into();
                oauth2_session.session_id = ActiveValue::set(oauth2_session_id);
                oauth2_session.expires_at = ActiveValue::set(chrono::Utc::now().into());
                oauth2_session.updated_at = ActiveValue::set(Utc::now().naive_utc());
                oauth2_session.update(&txn).await?
            }
            None => {
                // Create the session
                o_auth2_sessions::ActiveModel {
                    session_id: ActiveValue::set(oauth2_session_id),
                    expires_at: ActiveValue::set(
                        (chrono::Utc::now() + token.expires_in().unwrap()).into(),
                    ),
                    user_id: ActiveValue::set(user.id),
                    ..Default::default()
                }
                .insert(&txn)
                .await?
            }
        };
        txn.commit().await?;
        Ok(oauth2_session)
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(Utc::now().naive_utc().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn is_find_by_user_id(db: &impl ConnectionTrait, user_id: i32) -> ModelResult<bool> {
        let condition = Condition::all().add(o_auth2_sessions::Column::UserId.eq(user_id));
        let is_oauth = Entity::find().filter(condition).one(db).await?;
        dbg!(&is_oauth);
        match is_oauth {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
