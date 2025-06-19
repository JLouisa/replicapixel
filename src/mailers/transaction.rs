#![allow(non_upper_case_globals)]

use crate::{
    domain::mailer_options::MailerOptions,
    models::{PlanModel, TransactionModel, UserModel},
};
use loco_rs::prelude::*;
use serde::Serialize;
use serde_json::json;

static checkout_completed: Dir<'_> = include_dir!("src/mailers/transaction/checkout_completed");

#[derive(Serialize, Clone)]
pub struct CheckoutCompletedEmailData {
    pub user: UserModel,
    pub transaction: TransactionModel,
    pub plan: PlanModel,
    pub stripe_receipt_url: Option<String>,
}

#[allow(clippy::module_name_repetitions)]
pub struct CheckoutMailer {}
impl Mailer for CheckoutMailer {}
impl CheckoutMailer {
    /// Send an email
    ///
    /// # Errors
    /// When email sending is failed
    pub async fn send_checkout_completed(
        ctx: &AppContext,
        mail_options: &MailerOptions,
    ) -> Result<()> {
        // --- Send Mail ---
        Self::mail_template(
            ctx,
            &checkout_completed,
            mailer::Args {
                from: Some(mail_options.from_mail.to_string()),
                to: mail_options.to_mail.to_string(),
                locals: json!({ "options": &mail_options }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}
