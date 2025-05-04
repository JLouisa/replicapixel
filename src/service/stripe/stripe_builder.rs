use super::stripe::{StripeClient, StripeClientError};
use crate::models::{UserModel, _entities::sea_orm_active_enums::PlanNames};
use sea_orm::DatabaseConnection;

use stripe::{
    CheckoutSession, CheckoutSessionMode, CheckoutSessionUiMode, Currency, Metadata, ParseIdError,
    StripeError,
};
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

    pub fn metadata(&mut self) -> &mut Self {
        let plan = match self.plan {
            Some(plan) => plan,
            None => return self,
        };
        self.provided_metadata = Some(self.process_metadata(plan));
        self
    }

    fn process_metadata(&self, plan: &PlanNames) -> Metadata {
        let mut session_metadata = Metadata::new();
        session_metadata.insert("plan".to_string(), plan.to_string());
        session_metadata
    }

    pub async fn build(&self) -> Result<CheckoutSession, StripeCheckoutBuilderErr> {
        let plan = self
            .plan
            .ok_or(StripeCheckoutBuilderErr::MissingField("plan"))?;
        let user = self
            .user
            .ok_or(StripeCheckoutBuilderErr::MissingField("user"))?;
        let meta_data: Metadata = match &self.provided_metadata {
            Some(meta) => meta.clone(),
            None => self.process_metadata(plan),
        };

        let checkout_session = self
            .client
            .create_checkout(
                user,
                plan,
                &self.mode,
                &self.ui_mode,
                &self.currency,
                meta_data,
                self.db,
            )
            .await?;

        Ok(checkout_session)
    }
}
