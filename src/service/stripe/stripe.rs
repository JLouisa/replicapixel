use axum::{extract::Json, routing::post, Router};
use derive_more::{AsRef, Constructor, From};
use sea_orm::entity::prelude::*;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, Customer, CustomerId, ParseIdError, PriceId,
    StripeError,
};
use thiserror::Error;

use crate::{controllers::payment::routes, domain::url::Url, models::UserModel};

#[derive(Error, Debug)]
pub enum StripeErr {
    #[error("Conversion Error: {0}")]
    ParseIdError(#[from] ParseIdError),
    #[error("Stripe error: {0}")]
    StripeError(#[from] StripeError),
    #[error("Other error: {0}")]
    Other(String),
}

enum MetaEntity<'a> {
    UserModel(&'a UserModel),
    // Product(&'a Product),
    // Register(&'a Register),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
                "{}{}{}",
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
        self,
        user: &UserModel,
        plan: PlansNames,
        txn: &impl ConnectionTrait,
    ) -> Result<Url, String> {
        let customer_id: CustomerId = match Self::find_stripe_customer_id_in_db(&user.pid, txn)
            .await?
        {
            Some(existing_stripe_id_str) => {
                // Found in DB, parse it back into CustomerId
                CustomerId::from_str(&existing_stripe_id_str).map_err(|_| {
                    format!(
                        "Invalid stored Stripe Customer ID format: {}",
                        existing_stripe_id_str
                    )
                })?
            }
            None => {
                let mut customer_params = CreateCustomer::new();
                customer_params.email = Some(&user.email);
                let mut metadata = HashMap::new();
                metadata.insert("app_user_id".to_string(), user.pid.to_string());
                customer_params.metadata = Some(metadata);
                let customer = Customer::create(&self.client, customer_params)
                    .await
                    .map_err(|e: StripeError| format!("Failed to create Stripe customer: {}", e))?;
                Self::save_stripe_customer_id_to_db(&user.pid, customer.id.as_str(), txn).await?;
                customer.id
            }
        };

        let success_url = format!(
            "{}?session_id={{CHECKOUT_SESSION_ID}}",
            self.stripe_url.stripe_success_url.as_ref()
        );
        let checkout_params = CreateCheckoutSession {
            success_url: Some(success_url.as_str().into()),
            cancel_url: Some(self.stripe_url.stripe_cancel_url.as_ref()),
            return_url: Some(self.stripe_url.stripe_return_url.as_ref()),
            mode: Some(CheckoutSessionMode::Payment),
            customer: Some(customer_id),
            line_items: Some(vec![CreateCheckoutSessionLineItems {
                price: Some(self.stripe_products.get_price_id(&plan).to_string()),
                quantity: Some(1),
                ..Default::default()
            }]),
            ..Default::default()
        };

        dbg!("Creating checkout session: {:?}", &checkout_params);

        let session = CheckoutSession::create(&self.client, checkout_params)
            .await
            .map_err(|e: StripeError| format!("Stripe Checkout Session creation error: {}", e))?;

        // The URL should always be present for hosted checkout sessions
        match session.url {
            Some(url) => Ok(Url::new(url)),
            None => Err("Stripe Checkout Session created without a URL".to_string()),
        }
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
        user_id: &Uuid,
    ) -> Result<Customer, StripeErr> {
        let mut meta = HashMap::new();
        meta.insert("user_id".to_string(), user_id.to_string());
        meta.insert("email".to_string(), email.to_owned());

        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                name: Some(name),
                email: Some(email),
                description: None,
                metadata: Some(meta),
                ..Default::default()
            },
        )
        .await?;

        Ok(customer)
    }

    pub async fn get_customer(&self, user: &UserModel) -> Result<Customer, StripeErr> {
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

    async fn find_stripe_customer_id_in_db(
        user_id: &Uuid,
        db: &impl ConnectionTrait,
    ) -> Result<Option<String>, String> {
        // TODO: Implement database lookup
        todo!();
    }

    async fn save_stripe_customer_id_to_db(
        user_id: &Uuid,
        stripe_customer_id: &str,
        db: &impl ConnectionTrait,
    ) -> Result<(), String> {
        // TODO: Implement database update
        todo!();
    }
}
