use serde::Serialize;
use stripe::{CheckoutSession, CheckoutSessionId, CheckoutSessionStatus};

use super::stripe::StripeClient;

#[derive(Debug, Serialize)]
pub struct Session {
    status: Option<CheckoutSessionStatus>,
    customer_email: Option<String>,
}

pub struct StripeStatusService;

impl StripeStatusService {
    pub async fn handle_status(
        session_id: &CheckoutSessionId,
        stripe_client: &StripeClient,
    ) -> Result<Session, loco_rs::Error> {
        let session = CheckoutSession::retrieve(&stripe_client.client, &session_id, &[])
            .await
            .map_err(|_| loco_rs::Error::Message(String::from("Error checking storage: 101")))?;

        let email = match session.customer_details {
            Some(customer_details) => customer_details.email,
            None => {
                return Err(loco_rs::Error::Message(String::from(
                    "Error checking storage: 101",
                )))
            }
        };

        let session = Session {
            status: session.status,
            customer_email: email,
        };
        Ok(session)
    }
}
