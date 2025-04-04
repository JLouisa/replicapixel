use crate::modules::stripe::stripe::StripeIdPrice;
use derive_more::From;
use impl_custom_macro::ImplCustom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, From, ImplCustom)]
pub struct StripePriceId(Option<StripeIdPrice>);

impl Default for StripePriceId {
    fn default() -> Self {
        Self(None)
    }
}
