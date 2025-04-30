// auth mailer
#![allow(non_upper_case_globals)]

use chrono::Datelike;
use chrono::Local;
use loco_rs::prelude::*;
use serde_json::json;

use crate::domain::website::WebsiteBasicInfo;
use crate::models::users;

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static forgot: Dir<'_> = include_dir!("src/mailers/auth/forgot");
static magic_link: Dir<'_> = include_dir!("src/mailers/auth/magic_link");
// #[derive(Mailer)] // -- disabled for faster build speed. it works. but lets
// move on for now.

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    /// Sending welcome email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(
        ctx: &AppContext,
        user: &users::Model,
        website: &WebsiteBasicInfo,
    ) -> Result<()> {
        Self::mail_template(
            ctx,
            &welcome,
            mailer::Args {
                from: Some(website.from_mail()),
                to: user.email.to_string(),
                locals: json!({
                  "company": website.name.to_string(),
                  "name": user.name,
                  "verifyToken": user.email_verification_token,
                  "domain": ctx.config.server.full_url(),
                  "company_address": "Netherland".to_string(),
                  "current_year": Local::now().year(),
                  "twitter_url": "https://twitter.com/".to_string(),
                  "facebook_url": "https://www.facebook.com/".to_string(),
                  "linkedin_url": "https://www.linkedin.com/".to_string(),
                  "unsubscribe_url": "https://unsubscribe.com/".to_string(),
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
    /// Sending forgot password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(
        ctx: &AppContext,
        user: &users::Model,
        website: &WebsiteBasicInfo,
    ) -> Result<()> {
        Self::mail_template(
            ctx,
            &forgot,
            mailer::Args {
                from: Some(website.from_mail()),
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "domain": ctx.config.server.full_url(),
                  "company": website.name.to_string(),
                  "current_year": Local::now().year(),
                  "company_address": "Netherland".to_string(),
                  "twitter_url": "https://twitter.com/".to_string(),
                  "facebook_url": "https://www.facebook.com/".to_string(),
                  "linkedin_url": "https://www.linkedin.com/".to_string(),
                  "support_email": "support@".to_string(),
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sends a magic link authentication email to the user.
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_magic_link(
        ctx: &AppContext,
        user: &users::Model,
        website: &WebsiteBasicInfo,
    ) -> Result<()> {
        Self::mail_template(
            ctx,
            &magic_link,
            mailer::Args {
                from: Some(website.from_mail()),
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "token": user.magic_link_token.clone().ok_or_else(|| Error::string(
                            "the user model not contains magic link token",
                    ))?,
                  "host": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}
