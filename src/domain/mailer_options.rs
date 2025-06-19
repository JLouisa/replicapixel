use chrono::Datelike;
use chrono::Local;
use serde::Serialize;

use crate::controllers::payment::PricingView;
use crate::models::UserModel;
use crate::models::_entities::sea_orm_active_enums::Currency;
use crate::{
    domain::website::Website,
    models::{_entities::sea_orm_active_enums::Language, transactions::TransactionDomain},
    views::auth::UserView,
};

#[derive(Serialize)]
#[must_use]
pub struct MailerOptions {
    pub from_mail: String,
    pub to_mail: String,
    pub website: Option<Website>,
    pub user: Option<UserView>,
    pub language: Language,
    pub currency: Currency,
    pub transaction: Option<TransactionDomain>,
    pub plan: Option<PricingView>,
    pub token: Option<String>,
    pub stripe_receipt_url: Option<String>,
    pub logo_url: Option<String>,
    pub current_year: Option<i32>,
    pub company_address_line1: Option<String>,
    pub company_address_line2: Option<String>,
    pub support_email: Option<String>,
    pub twitter_url: Option<String>,
    pub facebook_url: Option<String>,
    pub linkedin_url: Option<String>,
}

impl Default for MailerOptions {
    fn default() -> Self {
        Self {
            from_mail: String::new(),
            to_mail: String::new(),
            logo_url: Some(String::from(
                "https://d2npyy9ae7osp9.cloudfront.net/others/logo.svg",
            )),
            current_year: Some(Local::now().year()),
            company_address_line1: Some("Netherland".to_string()),
            company_address_line2: Some("Netherland".to_string()),
            support_email: Some("jissicko@gmail.com".to_string()),
            twitter_url: Some("https://twitter.com".to_string()),
            facebook_url: Some("https://www.facebook.com".to_string()),
            linkedin_url: Some("https://www.linkedin.com".to_string()),
            language: Language::default(),
            currency: Currency::default(),
            website: None,
            plan: None,
            transaction: None,
            user: None,
            token: None,
            stripe_receipt_url: None,
        }
    }
}

impl MailerOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn website(self, website: &Website) -> Self {
        Self {
            from_mail: website.website_basic_info.from_mail(),
            website: Some(website.clone()),
            ..self
        }
    }
    pub fn into_user(self, user: &UserModel) -> Self {
        Self {
            to_mail: user.email.to_string(),
            token: user
                .magic_link_token
                .clone()
                .or_else(|| user.email_verification_token.clone()),
            user: Some(user.to_owned().into()),
            ..self
        }
    }
    pub fn user(self, user: &UserView) -> Self {
        Self {
            to_mail: user.email.to_string(),
            user: Some(user.to_owned()),
            ..self
        }
    }
    pub fn set_user(self, user: Option<UserView>) -> Self {
        Self { user, ..self }
    }
    pub fn language(self, language: &Language) -> Self {
        Self {
            language: language.clone(),
            ..self
        }
    }
    pub fn currency(self, currency: &Currency) -> Self {
        Self {
            currency: currency.clone(),
            ..self
        }
    }
    pub fn transaction(self, transaction: &TransactionDomain) -> Self {
        Self {
            transaction: Some(transaction.clone()),
            ..self
        }
    }
    pub fn plan(self, plan: &PricingView) -> Self {
        Self {
            plan: Some(plan.to_owned()),
            ..self
        }
    }
    pub fn token(self, token: &str) -> Self {
        Self {
            token: Some(token.to_string()),
            ..self
        }
    }
    pub fn set_token(self, token: Option<String>) -> Self {
        Self { token, ..self }
    }
    pub fn stripe_receipt_url(self, stripe_receipt_url: &str) -> Self {
        Self {
            stripe_receipt_url: Some(stripe_receipt_url.to_string()),
            ..self
        }
    }
    pub fn set_stripe_receipt_url(self, stripe_receipt_url: Option<String>) -> Self {
        Self {
            stripe_receipt_url,
            ..self
        }
    }
}
