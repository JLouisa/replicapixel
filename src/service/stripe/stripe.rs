use crate::{
    controllers::payment::routes,
    domain::url::Url,
    models::{UserActiveModel, UserModel},
};
use axum::http::StatusCode;
use axum::{extract::Json, routing::post, Router};
use derive_more::{AsRef, Constructor, From};
use loco_rs::prelude::Error as LocoError;
use loco_rs::{controller::ErrorDetail, model::ModelError};
use sea_orm::entity::prelude::*;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use stripe::{
    CheckoutSession, CheckoutSessionMode, CheckoutSessionUiMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, Currency, Customer, CustomerId, ParseIdError,
    PriceId, StripeError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StripeClientError {
    // Renamed for clarity
    #[error("Stripe API call failed: {0}")]
    StripeApi(#[from] StripeError), // Keep this, maybe rename variant

    #[error("Failed to parse ID: {0}")] // Keep if parsing happens inside client methods
    ParseId(#[from] ParseIdError),

    #[error("Database interaction failed: {0}")] // Add more specific errors if possible
    Database(#[from] sea_orm::DbErr), // Example if using SeaORM

    #[error("Database interaction failed: {0}")]
    DbModel(#[from] ModelError),

    #[error("Internal client error: {0}")] // Generic fallback
    Internal(String),

    #[error("Required configuration missing for client: {0}")] // e.g., API key
    Configuration(String),
}

enum MetaEntity<'a> {
    UserModel(&'a UserModel),
    // Product(&'a Product),
    // Register(&'a Register),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PlansNames {
    Basic,
    Premium,
    Max,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripeSettings {
    pub site: String,
    pub stripe_secret_key: String,
    pub stripe_publishable_key: String,
    pub stripe_webhook_secret: String,
    pub stripe_products: StripeProductsId,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StripeUrls {
    pub stripe_success_url: Url,
    pub stripe_cancel_url: Url,
    pub stripe_return_url: Url,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StripeProductsId {
    basic: String,
    premium: String,
    max: String,
}

#[derive(Deserialize, Debug, Constructor, Clone)]
pub struct StripeProducts {
    basic: PriceId,
    premium: PriceId,
    max: PriceId,
}
impl StripeProducts {
    pub fn get_price_id(&self, plan: &PlansNames) -> PriceId {
        match plan {
            PlansNames::Basic => self.basic.clone(),
            PlansNames::Premium => self.premium.clone(),
            PlansNames::Max => self.max.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Constructor, AsRef, From)]
pub struct StripePublishableKey(String);

#[derive(Clone)]
pub struct StripeClient {
    pub client: Client,
    pub settings: StripeSettings,
    pub stripe_publishable_key: StripePublishableKey,
    pub stripe_products: StripeProducts,
    pub stripe_url: StripeUrls,
}

impl StripeClient {
    pub fn new(settings: &StripeSettings) -> Self {
        let client = stripe::Client::new(&settings.stripe_secret_key);
        let stripe_publishable_key: StripePublishableKey =
            StripePublishableKey(String::from(&settings.stripe_publishable_key));
        let stripe_products = StripeProducts {
            basic: PriceId::from_str(&settings.stripe_products.basic)
                .expect("Invalid Price ID Basic"),
            premium: PriceId::from_str(&settings.stripe_products.premium)
                .expect("Invalid Price ID Premium"),
            max: PriceId::from_str(&settings.stripe_products.max).expect("Invalid Price ID Max"),
        };
        let stripe_url = StripeUrls {
            stripe_success_url: Url::new(format!(
                "{}{}{}?session_id={{CHECKOUT_SESSION_ID}}",
                settings.site,
                routes::Payment::BASE,
                routes::Payment::API_STRIPE_SUCCESS
            )),
            stripe_return_url: Url::new(format!(
                "{}{}{}",
                settings.site,
                routes::Payment::BASE,
                routes::Payment::API_STRIPE_RETURN
            )),
            stripe_cancel_url: Url::new(format!(
                "{}{}{}",
                settings.site,
                routes::Payment::BASE,
                routes::Payment::API_STRIPE_CANCEL
            )),
        };
        Self {
            client,
            settings: settings.clone(),
            stripe_publishable_key,
            stripe_products,
            stripe_url,
        }
    }

    pub async fn create_checkout(
        &self,
        user: &UserModel,
        plan: &PlansNames,
        mode: &CheckoutSessionMode,
        ui_mode: &CheckoutSessionUiMode,
        currency: &Currency,
        txn: &impl ConnectionTrait,
    ) -> Result<CheckoutSession, StripeClientError> {
        let customer_id: CustomerId = match user.stripe_customer_id.to_owned() {
            Some(existing_stripe_id_str) => CustomerId::from_str(&existing_stripe_id_str)?,
            None => {
                let stripe_customer = self
                    .create_customer(&user.name, &user.email, &user.pid)
                    .await?;
                Self::save_stripe_customer_id_to_db(&user, stripe_customer.id.as_str(), txn)
                    .await?;
                stripe_customer.id
            }
        };

        let checkout_params = CreateCheckoutSession {
            success_url: Some(self.stripe_url.stripe_success_url.as_ref()),
            cancel_url: match ui_mode == &CheckoutSessionUiMode::Hosted {
                true => Some(self.stripe_url.stripe_cancel_url.as_ref()),
                false => None,
            },
            return_url: match ui_mode == &CheckoutSessionUiMode::Embedded {
                true => Some(self.stripe_url.stripe_return_url.as_ref()),
                false => None,
            },
            mode: Some(*mode),
            ui_mode: Some(*ui_mode),
            currency: Some(*currency),
            customer: Some(customer_id),
            line_items: Some(vec![CreateCheckoutSessionLineItems {
                price: Some(self.stripe_products.get_price_id(&plan).to_string()),
                quantity: Some(1),
                ..Default::default()
            }]),
            ..Default::default()
        };

        dbg!("Creating checkout session: {:?}", &checkout_params);

        let session = CheckoutSession::create(&self.client, checkout_params).await?;

        Ok(session)
    }

    fn build_metadata(&self, entity: MetaEntity) -> std::collections::HashMap<String, String> {
        let mut meta = HashMap::new();
        match entity {
            MetaEntity::UserModel(user) => {
                meta.insert("user_id".to_string(), user.id.to_string());
                meta.insert("email".to_string(), user.email.to_owned());
            }
        }
        meta
    }

    pub async fn create_customer(
        &self,
        name: &str,
        email: &str,
        pid: &Uuid,
    ) -> Result<Customer, StripeClientError> {
        let mut meta = HashMap::new();
        meta.insert("user_id".to_string(), pid.to_string());
        meta.insert("email".to_string(), email.to_owned());

        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                name: Some(&name),
                email: Some(&email),
                description: None,
                metadata: Some(meta),
                ..Default::default()
            },
        )
        .await?;

        Ok(customer)
    }

    pub async fn get_customer(&self, user: &UserModel) -> Result<Customer, StripeClientError> {
        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                name: Some(&user.name),
                email: Some(&user.email),
                description: None,
                metadata: Some(self.build_metadata(MetaEntity::UserModel(user))),
                ..Default::default()
            },
        )
        .await?;

        Ok(customer)
    }

    async fn save_stripe_customer_id_to_db(
        user: &UserModel,
        stripe_customer_id: &str,
        db: &impl ConnectionTrait,
    ) -> Result<UserModel, ModelError> {
        let user = UserActiveModel::from(user.clone())
            .update_stripe_customer_id(stripe_customer_id, db)
            .await?;
        Ok(user)
    }
}
