use super::stripe::StripeClientError;
use std::marker::PhantomData;
use stripe::{
    CheckoutSessionMode, CheckoutSessionUiMode, Currency, Metadata, ParseIdError, StripeError,
};
use thiserror::Error;

use crate::models::{PlanModel, UserModel};

#[derive(Debug, Clone, Default)]
pub struct Missing;
#[derive(Debug, Clone, Default)]
pub struct Present;

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

#[derive(Debug, Clone)]
pub struct StripeOptions<'a> {
    pub user: &'a UserModel,
    pub plan: &'a PlanModel,
    pub ui_mode: CheckoutSessionUiMode,
    pub mode: CheckoutSessionMode,
    pub currency: Currency,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct StripeOptionsBuilder<'a, U = Missing, P = Missing, M = Missing> {
    user: Option<&'a UserModel>,
    plan: Option<&'a PlanModel>,
    metadata: Option<Metadata>,
    ui_mode: CheckoutSessionUiMode,
    mode: CheckoutSessionMode,
    currency: Currency,
    _user: PhantomData<U>,
    _plan: PhantomData<P>,
    _provided_metadata: PhantomData<M>,
}

impl<'a> StripeOptionsBuilder<'a, Missing, Missing, Missing> {
    pub fn new() -> Self {
        Self {
            user: None,
            plan: None,
            metadata: None,
            ui_mode: CheckoutSessionUiMode::Hosted,
            mode: CheckoutSessionMode::Payment,
            currency: Currency::USD,
            _user: PhantomData,
            _plan: PhantomData,
            _provided_metadata: PhantomData,
        }
    }
}

impl<'a, P, M> StripeOptionsBuilder<'a, Missing, P, M> {
    pub fn user(self, user: &'a UserModel) -> StripeOptionsBuilder<'a, Present, P, M> {
        StripeOptionsBuilder {
            user: Some(user),
            plan: self.plan,
            ui_mode: self.ui_mode,
            mode: self.mode,
            currency: self.currency,
            metadata: self.metadata,
            _user: PhantomData,
            _plan: PhantomData,
            _provided_metadata: PhantomData,
        }
    }
}

impl<'a, U, M> StripeOptionsBuilder<'a, U, Missing, M> {
    pub fn plan(self, plan: &'a PlanModel) -> StripeOptionsBuilder<'a, U, Present, M> {
        StripeOptionsBuilder {
            user: self.user,
            plan: Some(plan),
            ui_mode: self.ui_mode,
            mode: self.mode,
            currency: self.currency,
            metadata: self.metadata,
            _user: PhantomData,
            _plan: PhantomData,
            _provided_metadata: PhantomData,
        }
    }
}

impl<'a, U> StripeOptionsBuilder<'a, U, Present, Missing> {
    pub fn metadata(self) -> StripeOptionsBuilder<'a, U, Present, Present> {
        let metadata = self.process_metadata(self.plan.unwrap());
        StripeOptionsBuilder {
            user: self.user,
            plan: self.plan,
            ui_mode: self.ui_mode,
            mode: self.mode,
            currency: self.currency,
            metadata: Some(metadata),
            _user: PhantomData,
            _plan: PhantomData,
            _provided_metadata: PhantomData,
        }
    }
}

impl<'a> StripeOptionsBuilder<'a, Present, Present, Present> {
    pub fn build(self) -> StripeOptions<'a> {
        StripeOptions {
            user: self.user.unwrap(),
            plan: self.plan.unwrap(),
            ui_mode: self.ui_mode,
            mode: self.mode,
            currency: self.currency,
            metadata: self.metadata.unwrap(),
        }
    }
}

impl<'a, U, P, M> StripeOptionsBuilder<'a, U, P, M> {
    pub fn embedded(self) -> Self {
        Self {
            ui_mode: CheckoutSessionUiMode::Embedded,
            ..self
        }
    }

    pub fn subscription(self) -> Self {
        Self {
            mode: CheckoutSessionMode::Subscription,
            ..self
        }
    }

    pub fn eur(self) -> Self {
        Self {
            currency: Currency::EUR,
            ..self
        }
    }

    fn process_metadata(&self, plan: &PlanModel) -> Metadata {
        let mut session_metadata = Metadata::new();
        session_metadata.insert("plan_pid".to_string(), plan.pid.to_string());
        session_metadata
    }
}
