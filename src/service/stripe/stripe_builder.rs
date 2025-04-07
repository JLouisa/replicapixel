use stripe::CheckoutSession;

use super::{
    StripeCustomerId,
    _stripe::{StripeClient, StripeCurrency},
};
// use crate::{
//     data::Database,
//     domain::{
//         checkout::{CheckoutSessionGuest, CheckoutSessionUser},
//         error::GlobalError,
//         guest_order_items::GuestOrderItemList,
//         guest_orders::GuestOrder,
//         order_items::OrderItemList,
//         orders::Order,
//         payments::PaymentSessionId,
//         users::User,
//     },
// };
use std::sync::Arc;

#[derive(Clone)]
pub struct CheckoutSessionBuilder<'a> {
    client: &'a StripeClient,
    db: &'a Arc<Database>,
    user: Option<&'a User>,
    order: Option<Order>,
    order_item: Option<&'a OrderItemList>,
    guest_order: Option<GuestOrder>,
    guest_order_item: Option<&'a GuestOrderItemList>,
    stripe_currency: Option<&'a StripeCurrency>,
}

impl<'a> CheckoutSessionBuilder<'a> {
    // Initialize the builder with a reference to StripeClient
    pub fn new(client: &'a StripeClient, db: &'a Arc<Database>) -> Self {
        Self {
            client,
            db,
            user: None,
            order: None,
            order_item: None,
            guest_order: None,
            guest_order_item: None,
            stripe_currency: None,
        }
    }

    // Set the stripe_currency field
    pub fn stripe_currency(&mut self, stripe_currency: &'a StripeCurrency) -> &mut Self {
        self.stripe_currency = Some(stripe_currency);
        self
    }

    // Set the user field
    pub fn user_checkout_info(&mut self, user: &'a CheckoutSessionUser) -> &mut Self {
        self.user = Some(&user.user);
        self.order = Some(user.order.clone());
        self.order_item = Some(&user.order_items); // Assuming user.order_items is accessible here
        self
    }

    pub fn guest_checkout_info(&mut self, guest: &'a CheckoutSessionGuest) -> &mut Self {
        self.guest_order = Some(guest.guest_order.clone());
        self.guest_order_item = Some(&guest.guest_order_items);
        self
    }

    // Build the CheckoutSession asynchronously
    pub async fn build(&mut self) -> Result<(CheckoutSession, Order), GlobalError> {
        // Unwrap the required fields or return an error if any are missing

        let stripe_currency = self
            .stripe_currency
            .ok_or_else(|| GlobalError::MissingFieldErr("Currency"))?;
        let user = self
            .user
            .ok_or_else(|| GlobalError::MissingFieldErr("user"))?; // Use `as_mut()` to get a mutable reference
        let order = self
            .order
            .as_mut()
            .ok_or_else(|| GlobalError::MissingFieldErr("user"))?; // Use `as_mut()` to get a mutable reference
        let order_item = self
            .order_item
            .ok_or_else(|| GlobalError::MissingFieldErr("order_item"))?;

        let checkout_session = self
            .client
            .create_checkout_session(stripe_currency, &user, &order_item, self.db)
            .await?;

        // Now you can mutate `user.order.payment_session_id` because `user` is mutable
        order.payment_session_id =
            PaymentSessionId::new(Some(String::from(checkout_session.id.as_str())));

        // Save the user and order details to the database
        order.save(self.db).await?;
        order_item.save(&self.db).await?;

        let new_order: Order = order.clone();

        Ok((checkout_session, new_order))
    }

    // Build the CheckoutSession asynchronously
    pub async fn build_guest(&mut self) -> Result<(CheckoutSession, GuestOrder), GlobalError> {
        // Unwrap the required fields or return an error if any are missing

        let stripe_currency = self
            .stripe_currency
            .ok_or_else(|| GlobalError::MissingFieldErr("Currency"))?;
        let guest_order = self
            .guest_order
            .as_mut()
            .ok_or_else(|| GlobalError::MissingFieldErr("user"))?; // Use `as_mut()` to get a mutable reference
        let guest_order_list = self
            .guest_order_item
            .ok_or_else(|| GlobalError::MissingFieldErr("order_item"))?;

        let customer_id = self.client.create_guest_customer(guest_order).await?.id;

        let checkout_session = self
            .client
            .create_guest_checkout_session(
                stripe_currency,
                &customer_id,
                &guest_order_list,
                self.db,
            )
            .await?;

        // Now you can mutate `user.order.payment_session_id` because `user` is mutable
        guest_order.payment_session_id =
            PaymentSessionId::new(Some(String::from(checkout_session.id.as_str())));
        guest_order.stripe_customer_id = StripeCustomerId::new(Some(customer_id));

        // Save the user and order details to the database
        guest_order.save(self.db).await?;
        guest_order_list.save(&self.db).await?;

        let new_order: GuestOrder = guest_order.clone();

        Ok((checkout_session, new_order))
    }
}
