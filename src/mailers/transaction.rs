#![allow(non_upper_case_globals)]

use crate::{
    domain::website::WebsiteBasicInfo,
    models::{PlanModel, TransactionModel, UserModel},
};
use chrono::{Datelike, Utc};
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use serde_json::json;

static checkout_completed: Dir<'_> = include_dir!("src/mailers/transaction/checkout_completed");

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
        website: &WebsiteBasicInfo,
        collection: &CheckoutCompletedEmailData,
    ) -> Result<()> {
        // Use loco_rs::Result

        // --- Prepare Variables for Template ---
        let customer_name = collection.user.name.clone();
        let plan_name = collection.plan.name.clone();
        let transaction_date = Utc::now().format("%B %d, %Y at %H:%M %Z").to_string();
        let transaction_id = collection.transaction.pid.to_string();

        // Format Amount and Currency (Adapt based on how you store them in TransactionDomain)
        let amount_paid_formatted = {
            let amount = collection.transaction.payment_amount;
            let currency_code = collection.transaction.currency.to_owned();
            let amount_major = Decimal::new(amount, 2);
            match currency_code.as_str() {
                "usd" => format!("${:.2}", amount_major), // Just symbol $
                "eur" => format!("€{:.2}", amount_major), // Just symbol €
                // Add other specific symbols if needed (e.g., "GBP" => "£{:.2}")
                _ => format!("{:.2} {}", amount_major, currency_code), // Fallback to amount + code
            }
        };

        // 3. Add specific plan details
        let plan_model_amount = collection.plan.model_amount; // Assuming PlanModel has `model_amount: i32` or similar
        let plan_credit_amount = collection.plan.credit_amount; // Assuming PlanModel has `credit_amount: i32` or similar

        // Get config values or set defaults
        let company_name = website.name.to_owned();
        let website_link = ctx.config.server.full_url();
        let dashboard_link = format!("{}/dashboard", ctx.config.server.full_url());
        let help_center_link = format!("{}/help", ctx.config.server.full_url());
        let support_email = format!("{}/help", ctx.config.server.full_url()); //Todo Replace with real support email
        let logo_url = format!("{}/assets/images/logo.png", ctx.config.server.full_url());
        let company_address_line1 = "Netherland".to_string();
        let company_address_line2 = "Netherland".to_string();
        let current_year = Utc::now().year().to_string();

        // --- Locals for the Template ---
        let locals = json!({
            "customer_name": customer_name,
            "plan_name": plan_name,
            "transaction_date": transaction_date,
            "transaction_id": transaction_id,
            "amount_paid": amount_paid_formatted,
            "dashboard_link": dashboard_link,
            "help_center_link": help_center_link,
            "company_name": company_name,
            "website_link": website_link,
            "logo_url": logo_url,
            "company_address_line1": company_address_line1,
            "company_address_line2": company_address_line2,
            "current_year": current_year,
            "support_email": support_email,
            "plan_model_amount": plan_model_amount, // NEW plan detail
            "plan_credit_amount": plan_credit_amount, // NEW plan detail
            "stripe_receipt_url": collection.stripe_receipt_url, // Uncomment if passing and using in template
        });

        // --- Send Mail ---
        Self::mail_template(
            ctx,
            &checkout_completed,
            mailer::Args {
                from: Some(website.from_mail()),
                to: collection.user.email.to_string(),
                locals,
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}
