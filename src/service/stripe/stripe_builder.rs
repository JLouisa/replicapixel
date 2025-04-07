use super::stripe::{PlansNames, StripeClient, StripeClientError};
use crate::models::UserModel;
use sea_orm::DatabaseConnection;

use stripe::{
    CheckoutSession, CheckoutSessionMode, CheckoutSessionUiMode, Currency, ParseIdError,
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
    plan: Option<&'a PlansNames>,
    ui_mode: CheckoutSessionUiMode,
    mode: CheckoutSessionMode,
    currency: Currency,
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
        }
    }

    pub fn plan(&mut self, plan: &'a PlansNames) -> &mut Self {
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

    pub async fn build(&self) -> Result<CheckoutSession, StripeCheckoutBuilderErr> {
        let plan = self
            .plan
            .ok_or(StripeCheckoutBuilderErr::MissingField("plan"))?;
        let user = self
            .user
            .ok_or(StripeCheckoutBuilderErr::MissingField("user"))?;
        let checkout_session = self
            .client
            .create_checkout(
                user,
                plan,
                &self.mode,
                &self.ui_mode,
                &self.currency,
                self.db,
            )
            .await?;

        Ok(checkout_session)
    }
}
