use std::collections::HashMap;

use super::stripe::{StripeClient, StripeClientError};
use crate::models::{UserModel, _entities::sea_orm_active_enums::PlanNames};
use sea_orm::DatabaseConnection;

use stripe::{
    CheckoutSession, CheckoutSessionMode, CheckoutSessionUiMode, Currency, Metadata, ParseIdError,
    StripeError,
};

use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::Error as LocoError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StripeCheckoutBuilderErr {
    #[error("Required field missing: {0}")]
    MissingField(&'static str),

    #[error("Conversion Error: {0}")]
    ParseIdError(#[from] ParseIdError),

    #[error("Stripe API error: {0}")]
    StripeError(#[from] StripeError),

    #[error("Stripe client operation failed: {0}")]
    ClientOperation(#[from] StripeClientError),
}

pub struct CheckoutSessionBuilder<'a> {
    client: &'a StripeClient,
    db: &'a DatabaseConnection,
    user: Option<&'a UserModel>,
    plan: Option<&'a PlanNames>,
    ui_mode: CheckoutSessionUiMode,
    mode: CheckoutSessionMode,
    currency: Currency,
    provided_metadata: Option<Metadata>,
    transaction_id: Option<&'a uuid::Uuid>,
}

impl<'a> CheckoutSessionBuilder<'a> {
    pub fn new(client: &'a StripeClient, db: &'a DatabaseConnection) -> Self {
        Self {
            client,
            db,
            user: None,
            plan: None,
            ui_mode: CheckoutSessionUiMode::Hosted,
            mode: CheckoutSessionMode::Payment,
            currency: Currency::USD,
            provided_metadata: None,
            transaction_id: None,
        }
    }

    pub fn plan(&mut self, plan: &'a PlanNames) -> &mut Self {
        self.plan = Some(plan);
        self
    }

    pub fn user(&mut self, user: &'a UserModel) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn embedded(&mut self) -> &mut Self {
        self.ui_mode = CheckoutSessionUiMode::Embedded;
        self
    }

    pub fn subscription(&mut self) -> &mut Self {
        self.mode = CheckoutSessionMode::Subscription;
        self
    }

    pub fn eur(&mut self) -> &mut Self {
        self.currency = Currency::EUR;
        self
    }

    pub fn transaction_id(&mut self, id: &'a uuid::Uuid) -> &mut Self {
        self.transaction_id = Some(id);
        self
    }

    pub fn metadata(&mut self) -> &mut Self {
        let user = match self.user {
            Some(user) => user,
            None => return self,
        };
        let plan = match self.plan {
            Some(plan) => plan,
            None => return self,
        };
        let meta = Metadata::from([
            ("user_id".to_string(), user.id.to_string()),
            ("email".to_string(), user.email.to_owned()),
            ("plan".to_string(), plan.to_string()),
        ]);
        self.provided_metadata = Some(meta);
        self
    }

    fn process_metadata(&self, plan: &PlanNames, user: &UserModel) -> Metadata {
        let mut session_metadata = Metadata::new();
        session_metadata.insert("user_id".to_string(), user.id.to_string());
        session_metadata.insert("email".to_string(), user.email.clone());
        session_metadata.insert("plan".to_string(), plan.to_string());
        // Optional: Merge with other metadata if you have self.provided_metadata
        if let Some(provided) = &self.provided_metadata {
            for (key, value) in provided.iter() {
                session_metadata.insert(key.clone(), value.clone());
            }
        }
        session_metadata
    }

    pub async fn build(&self) -> Result<CheckoutSession, StripeCheckoutBuilderErr> {
        let plan = self
            .plan
            .ok_or(StripeCheckoutBuilderErr::MissingField("plan"))?;
        let user = self
            .user
            .ok_or(StripeCheckoutBuilderErr::MissingField("user"))?;
        let transaction_id = self
            .transaction_id
            .ok_or(StripeCheckoutBuilderErr::MissingField("transaction_id"))?;

        let session_metadata = self.process_metadata(plan, user);
        let checkout_session = self
            .client
            .create_checkout(
                user,
                plan,
                transaction_id.to_string(),
                &self.mode,
                &self.ui_mode,
                &self.currency,
                session_metadata,
                self.db,
            )
            .await?;

        Ok(checkout_session)
    }
}
