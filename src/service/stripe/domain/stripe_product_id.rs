use crate::modules::stripe::stripe::StripeIdProduct;
use derive_more::From;
use impl_custom_macro::ImplCustom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, From, ImplCustom)]
pub struct StripeProductId(Option<StripeIdProduct>);

impl Default for StripeProductId {
    fn default() -> Self {
        Self(None)
    }
}
