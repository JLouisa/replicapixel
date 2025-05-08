use async_trait::async_trait;
use chrono::{offset::Local, Duration};
use derive_more::{AsRef, Constructor, From};
use google_oauth::GooglePayload;
use loco_rs::{auth::jwt, controller::ErrorDetail, hash, prelude::*};
use passwords::PasswordGenerator;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
use validator::ValidateEmail;

use super::{
    o_auth2_sessions, UserSettingsActiveModel,
    _entities::sea_orm_active_enums::{Language, ThemePreference},
};
use crate::service::stripe::stripe::StripeClient;
use loco_oauth2::models::users::OAuth2UserTrait;

pub use super::_entities::users::{self, ActiveModel, Entity, Model};
use super::{user_credits::UserCreditsInit, UserCreditActiveModel, UserModel};

use axum::http::StatusCode;

pub const MAGIC_LINK_LENGTH: i8 = 32;
pub const MAGIC_LINK_EXPIRATION_MIN: i8 = 5;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub remember: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterParams {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(must_match(other = "confirm_password", message = "Passwords do not match"))]
    pub password: String,
    pub confirm_password: String,
    pub email_notifications: bool,
    pub marketing: bool,
    #[serde(default)]
    pub theme_preference: ThemePreference,
    #[serde(default)]
    pub language: Language,
    #[serde(skip_deserializing, default)]
    pub picture: Option<String>,
}
impl RegisterParams {
    pub fn validate_email(&self) -> Vec<String> {
        let mut validate = Vec::new();

        // Trim the email
        let email = self.email.trim();

        // Check if the email is empty or whitespace
        if email.is_empty() {
            validate.push(String::from("Email cannot be empty"));
        }

        // Check if the email is too long
        if email.len() > 256 {
            validate.push(String::from("Email is too long"));
        }

        // Check if the email is valid
        if !ValidateEmail::validate_email(&email) {
            validate.push(String::from("Email is invalid"));
        }

        validate
    }

    pub fn validate_name(&self) -> Vec<String> {
        let mut validate = Vec::new();

        // Trim the name and check if it is empty or consists only of whitespace
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            validate.push(String::from("Name cannot be empty"));
        }

        // Check if the name is too short (less than 2 graphemes)
        let grapheme_count = trimmed_name.graphemes(true).count();
        if grapheme_count < 2 {
            validate.push(String::from("Name must be at least 2 characters long"));
        }

        // Check if the name is too long (more than 256 graphemes)
        if grapheme_count > 256 {
            validate.push(String::from("Name cannot be longer than 256 characters"));
        }

        // Check if the name contains any forbidden characters
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        if trimmed_name
            .chars()
            .any(|c| forbidden_characters.contains(&c))
        {
            validate.push(String::from("Name cannot contain any special characters"));
        }

        validate
    }

    pub fn validate_password(&self) -> Vec<String> {
        let mut validate = Vec::new();

        // Trim the password and check if it is empty or consists only of whitespace
        let trimmed_password = self.password.clone(); //-->  do no trim
        if trimmed_password.is_empty() {
            validate.push(String::from("Password cannot be empty"));
        }

        // Check if the password is too short or too long
        let password_length = trimmed_password.len();
        if password_length < 8 {
            validate.push(String::from("Password must be at least 8 characters long"));
        }
        if password_length > 30 {
            validate.push(String::from("Password cannot be longer than 30 characters"));
        }

        // let has_uppercase = trimmed_password.chars().any(|c| c.is_ascii_uppercase());
        // let has_lowercase = trimmed_password.chars().any(|c| c.is_ascii_lowercase());
        // let has_digit = trimmed_password.chars().any(|c| c.is_ascii_digit());
        // let has_special = trimmed_password.chars().any(|c| !c.is_ascii_alphanumeric());

        // if !has_uppercase {
        //     validate.push(String::from(
        //         "Password must contain at least one uppercase letter",
        //     ));
        // }
        // if !has_lowercase {
        //     validate.push(String::from(
        //         "Password must contain at least one lowercase letter",
        //     ));
        // }
        // if !has_digit {
        //     validate.push(String::from("Password must contain at least one digit"));
        // }
        // if !has_special {
        //     validate.push(String::from(
        //         "Password must contain at least one special character",
        //     ));
        // }

        validate
    }
    pub fn validate_password_confirm(&self) -> Vec<String> {
        let mut validate = Vec::new();

        let compare = self.password == self.confirm_password;
        if !compare {
            validate.push(String::from("Passwords do not match"));
        }

        validate
    }
    pub fn create_with_ott(google_user_data: GooglePayload) -> Result<Self, loco_rs::Error> {
        let pg = PasswordGenerator::new()
            .length(12)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .spaces(true)
            .exclude_similar_characters(true)
            .strict(true)
            .generate_one();

        let pg = match pg {
            Ok(p) => p,
            Err(e) => {
                dbg!(&e);
                return Err(loco_rs::Error::CustomError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorDetail::new("User Creation Error:", "User Creation Error: 2"),
                ));
            }
        };

        let register = RegisterParams {
            name: google_user_data
                .name
                .or(google_user_data.given_name)
                .ok_or_else(|| {
                    tracing::warn!(
                        "Google token payload missing name for sub: {:?}",
                        google_user_data.sub
                    );
                    loco_rs::Error::CustomError(
                        StatusCode::BAD_REQUEST,
                        ErrorDetail::new("MISSING_NAME", "Name not provided by Google."),
                    )
                })?,
            email: google_user_data.email.ok_or_else(|| {
                tracing::warn!(
                    "Google token payload missing email for sub: {:?}",
                    google_user_data.sub
                );
                loco_rs::Error::CustomError(
                    StatusCode::BAD_REQUEST,
                    ErrorDetail::new("MISSING_EMAIL", "Email not provided by Google."),
                )
            })?,
            password: pg.to_owned(),
            confirm_password: pg,
            email_notifications: true,
            marketing: true,
            theme_preference: Default::default(),
            language: Default::default(),
            picture: google_user_data.picture,
        };

        Ok(register)
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PasswordChangeParams {
    pub current_password: String,
    #[validate(must_match(other = "confirm_password", message = "Passwords do not match"))]
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Clone, Constructor, AsRef, From)]
pub struct UserId(i32);

#[derive(Debug, Serialize, Clone, AsRef, From)]
pub struct UserPid(String);
impl UserPid {
    pub fn new<K: Into<String>>(key: K) -> Self {
        Self(key.into())
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub pid: Uuid,
    pub name: String,
    pub email: String,
}
impl User {
    pub async fn load_user(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let user_model = UserModel::find_by_pid(db, &pid.to_string()).await?;
        let user = Self {
            id: user_model.id,
            pid: user_model.pid,
            name: user_model.name,
            email: user_model.email,
        };
        Ok(user)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterError {
    pub passed: bool,
    pub name: Vec<String>,
    pub email: Vec<String>,
    pub password: Vec<String>,
    pub password_confirm: Vec<String>,
    pub global: Vec<String>,
}
impl RegisterError {
    pub fn validate(register: &RegisterParams) -> Self {
        // Validate name, email, and password
        let name = register.validate_name();
        let email = register.validate_email();
        let password = register.validate_password();
        let password_confirm = register.validate_password_confirm();

        // Check if all validations passed (no errors in any field)
        let passed = name.is_empty()
            && email.is_empty()
            && password.is_empty()
            && password_confirm.is_empty();

        // Create and return the RegisterError struct
        Self {
            passed,
            name,
            email,
            password,
            password_confirm,
            global: Vec::new(),
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.name.as_ref().to_owned(),
            email: self.email.as_ref().to_owned(),
        })
    }
}

/// `OAuth2UserProfile` user profile information via scopes
/// https://developers.google.com/identity/openid-connect/openid-connect#obtainuserinfo
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OAuth2UserProfile {
    // https://www.googleapis.com/auth/userinfo.email	See your primary Google Account email address
    pub email: String,
    // https://www.googleapis.com/auth/userinfo.profile   See your personal info, including any personal info you've made publicly available
    pub name: String,
    // sub field is unique
    pub sub: String,
    pub email_verified: bool,
    pub given_name: Option<String>, // Some accounts don't have this field
    pub family_name: Option<String>, // Some accounts don't have this field
    pub picture: Option<String>,    // Some accounts don't have this field
    pub locale: Option<String>,     // Some accounts don't have this field
}

#[async_trait]
impl OAuth2UserTrait<OAuth2UserProfile> for Model {
    /// Asynchronously finds user by OAuth2 session id.
    /// # Arguments
    /// * `db` - Database connection
    /// * `cookie` - OAuth2 session id
    ///
    /// # Returns
    /// * `Self` - The `OAuth2UserTrait` struct
    ///
    /// # Errors
    /// * `ModelError` - When could not find the user in the DB
    async fn find_by_oauth2_session_id(
        db: &DatabaseConnection,
        session_id: &str,
    ) -> ModelResult<Self> {
        // find the session by the session id
        let session = o_auth2_sessions::Entity::find()
            .filter(super::_entities::o_auth2_sessions::Column::SessionId.eq(session_id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        // if the session is found, find the user by the user id
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(session.user_id))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    /// Asynchronously upsert user with OAuth data and saves it to the
    /// database.
    /// # Arguments
    /// * `db` - Database connection
    /// * `profile` - OAuth profile
    ///
    /// # Returns
    /// * `Self` - The `OAuth2UserTrait` struct
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    async fn upsert_with_oauth(
        db: &DatabaseConnection,
        profile: &OAuth2UserProfile,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let user = match users::Entity::find()
            .filter(users::Column::Email.eq(&profile.email))
            .one(&txn)
            .await?
        {
            None => {
                let pg = PasswordGenerator::new()
                    .length(8)
                    .numbers(true)
                    .lowercase_letters(true)
                    .uppercase_letters(true)
                    .symbols(true)
                    .spaces(true)
                    .exclude_similar_characters(true)
                    .strict(true);
                let password = pg.generate_one().map_err(|e| ModelError::Any(e.into()))?;
                // We use the sub field as the user fake password since sub is unique
                let password_hash =
                    hash::hash_password(&password).map_err(|e| ModelError::Any(e.into()))?;
                // Create the user into the database
                users::ActiveModel {
                    email: ActiveValue::set(profile.email.to_string()),
                    name: ActiveValue::set(profile.name.to_string()),
                    email_verified_at: ActiveValue::set(Some(Local::now().into())),
                    password: ActiveValue::set(password_hash),
                    ..Default::default()
                }
                .insert(&txn)
                .await
                .map_err(|e| {
                    tracing::error!("Error while trying to create user: {e}");
                    ModelError::Any(e.into())
                })?
            }
            // Do nothing if user exists
            Some(user) => user,
        };
        txn.commit().await?;
        Ok(user)
    }

    /// Generates a JWT
    /// # Arguments
    /// * `secret` - JWT secret
    /// * `expiration` - JWT expiration time
    ///
    /// # Returns
    /// * `String` - JWT token
    ///
    /// # Errors
    /// * `ModelError` - When could not generate the JWT
    fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        self.generate_jwt(secret, expiration.to_owned())
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::users::ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if insert {
            let mut this = self;
            this.pid = ActiveValue::Set(Uuid::new_v4());
            this.api_key = ActiveValue::Set(format!("lo-{}", Uuid::new_v4()));
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

#[async_trait]
impl Authenticable for Model {
    async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_pid(db, claims_key).await
    }
}

impl Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, email)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::EmailVerificationToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the magic token and verify and token expiration
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error ot token expired
    pub async fn find_by_magic_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                query::condition()
                    .eq(users::Column::MagicLinkToken, token)
                    .build(),
            )
            .one(db)
            .await?;

        let user = user.ok_or_else(|| ModelError::EntityNotFound)?;
        if let Some(expired_at) = user.magic_link_expiration {
            if expired_at >= Local::now() {
                Ok(user)
            } else {
                tracing::debug!(
                    user_pid = user.pid.to_string(),
                    token_expiration = expired_at.to_string(),
                    "magic token expired for the user."
                );
                Err(ModelError::msg("magic token expired"))
            }
        } else {
            tracing::error!(
                user_pid = user.pid.to_string(),
                "magic link expiration time not exists"
            );
            Err(ModelError::msg("expiration token not exists"))
        }
    }

    /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ResetToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided pid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided api key
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn upsert_with_ott(
        db: &DatabaseConnection,
        params: &RegisterParams,
        stripe_client: &StripeClient,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let user_find = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, &params.email)
                    .build(),
            )
            .one(&txn)
            .await?;

        if user_find.is_some() {
            txn.commit().await?;
            return Ok(user_find.unwrap());
        }

        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let user_pid = Uuid::new_v4();

        let stripe_customer = match stripe_client
            .create_customer(&params.name, &params.email, &user_pid)
            .await
        {
            Ok(stripe_customer) => Some(stripe_customer.id.to_string()),
            Err(_) => None,
        };

        let user = users::ActiveModel {
            pid: ActiveValue::set(user_pid),
            email: ActiveValue::set(params.email.to_string()),
            password: ActiveValue::set(password_hash),
            name: ActiveValue::set(params.name.to_string()),
            stripe_customer_id: ActiveValue::set(stripe_customer),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let user_credits_init = UserCreditsInit::default();
        UserCreditActiveModel {
            pid: ActiveValue::set(user_credits_init.pid),
            user_id: ActiveValue::set(user.id),
            model_amount: ActiveValue::set(user_credits_init.model_amount),
            credit_amount: ActiveValue::set(user_credits_init.credit_amount),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        UserSettingsActiveModel {
            user_id: ActiveValue::set(user.id),
            enable_notification_email: ActiveValue::set(params.email_notifications),
            enable_marketing_email: ActiveValue::set(params.marketing),
            theme: ActiveValue::set(params.theme_preference.clone()),
            language: ActiveValue::set(params.language.clone()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(user)
    }

    pub async fn create_with_password(
        db: &DatabaseConnection,
        params: &RegisterParams,
        stripe_client: &StripeClient,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, &params.email)
                    .build(),
            )
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists {});
        }

        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let user_pid = Uuid::new_v4();

        let stripe_customer = match stripe_client
            .create_customer(&params.name, &params.email, &user_pid)
            .await
        {
            Ok(stripe_customer) => Some(stripe_customer.id.to_string()),
            Err(_) => None,
        };

        let user = users::ActiveModel {
            pid: ActiveValue::set(user_pid),
            email: ActiveValue::set(params.email.to_string()),
            password: ActiveValue::set(password_hash),
            name: ActiveValue::set(params.name.to_string()),
            stripe_customer_id: ActiveValue::set(stripe_customer),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let user_credits_init = UserCreditsInit::default();
        UserCreditActiveModel {
            pid: ActiveValue::set(user_credits_init.pid),
            user_id: ActiveValue::set(user.id),
            model_amount: ActiveValue::set(user_credits_init.model_amount),
            credit_amount: ActiveValue::set(user_credits_init.credit_amount),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        UserSettingsActiveModel {
            user_id: ActiveValue::set(user.id),
            enable_notification_email: ActiveValue::set(params.email_notifications),
            enable_marketing_email: ActiveValue::set(params.marketing),
            theme: ActiveValue::set(params.theme_preference.clone()),
            language: ActiveValue::set(params.language.clone()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(user)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: u64) -> ModelResult<String> {
        Ok(jwt::JWT::new(secret).generate_token(expiration, self.pid.to_string(), Map::new())?)
    }
}

impl ActiveModel {
    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.email_verification_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.reset_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.reset_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        Ok(self.update(db).await?)
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.email_verified_at = ActiveValue::set(Some(Local::now().into()));
        Ok(self.update(db).await?)
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<Model> {
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        self.reset_token = ActiveValue::Set(None);
        self.reset_sent_at = ActiveValue::Set(None);
        Ok(self.update(db).await?)
    }

    pub async fn update_stripe_customer_id(
        mut self,
        stripe_customer_id: &str,
        db: &impl ConnectionTrait,
    ) -> ModelResult<Model> {
        self.stripe_customer_id = ActiveValue::set(Some(stripe_customer_id.to_string()));
        Ok(self.update(db).await?)
    }

    /// Creates a magic link token for passwordless authentication.
    ///
    /// Generates a random token with a specified length and sets an expiration time
    /// for the magic link. This method is used to initiate the magic link authentication flow.
    ///
    /// # Errors
    /// - Returns an error if database update fails
    pub async fn create_magic_link(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        let random_str = hash::random_string(MAGIC_LINK_LENGTH as usize);
        let expired = Local::now() + Duration::minutes(MAGIC_LINK_EXPIRATION_MIN.into());

        self.magic_link_token = ActiveValue::set(Some(random_str));
        self.magic_link_expiration = ActiveValue::set(Some(expired.into()));
        Ok(self.update(db).await?)
    }

    /// Verifies and invalidates the magic link after successful authentication.
    ///
    /// Clears the magic link token and expiration time after the user has
    /// successfully authenticated using the magic link.
    ///
    /// # Errors
    /// - Returns an error if database update fails
    pub async fn clear_magic_link(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.magic_link_token = ActiveValue::set(None);
        self.magic_link_expiration = ActiveValue::set(None);
        Ok(self.update(db).await?)
    }
}
