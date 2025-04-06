use crate::domain::website::Website;
use loco_rs::prelude::*;

pub fn payment_home(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/payment.html",
        data!({"plans": website.payment_plans, "website": website.website_settings}),
    )
}
