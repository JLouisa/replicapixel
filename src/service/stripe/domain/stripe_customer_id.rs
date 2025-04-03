use crate::modules::stripe::stripe::StripeIdCustomer;
use derive_more::From;
use impl_custom_macro::ImplCustom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, From, ImplCustom)]
pub struct StripeCustomerId(Option<StripeIdCustomer>);

impl Default for StripeCustomerId {
    fn default() -> Self {
        Self(None)
    }
}
