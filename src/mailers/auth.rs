// auth mailer
#![allow(non_upper_case_globals)]

use loco_rs::prelude::*;
use serde_json::json;

use crate::domain::mailer_options::MailerOptions;

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static forgot: Dir<'_> = include_dir!("src/mailers/auth/forgot");
static magic_link: Dir<'_> = include_dir!("src/mailers/auth/magic_link");
static verification_link: Dir<'_> = include_dir!("src/mailers/auth/verification_link");
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
    pub async fn send_welcome(ctx: &AppContext, mailer_options: &MailerOptions) -> Result<()> {
        Self::mail_template(
            ctx,
            &welcome,
            mailer::Args {
                from: Some(mailer_options.from_mail.to_string()),
                to: mailer_options.to_mail.to_string(),
                locals: json!({ "options": &mailer_options }),
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
    pub async fn forgot_password(ctx: &AppContext, mailer_options: &MailerOptions) -> Result<()> {
        Self::mail_template(
            ctx,
            &forgot,
            mailer::Args {
                from: Some(mailer_options.from_mail.to_string()),
                to: mailer_options.to_mail.to_string(),
                locals: json!({ "options": &mailer_options }),
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
    pub async fn send_magic_link(ctx: &AppContext, mailer_options: &MailerOptions) -> Result<()> {
        Self::mail_template(
            ctx,
            &magic_link,
            mailer::Args {
                from: Some(mailer_options.from_mail.to_string()),
                to: mailer_options.to_mail.to_string(),
                locals: json!({ "options": &mailer_options }),
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
    pub async fn send_verification_link(
        ctx: &AppContext,
        mailer_options: &MailerOptions,
    ) -> Result<()> {
        Self::mail_template(
            ctx,
            &verification_link,
            mailer::Args {
                from: Some(mailer_options.from_mail.to_string()),
                to: mailer_options.to_mail.to_string(),
                locals: json!({ "options": &mailer_options }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}
