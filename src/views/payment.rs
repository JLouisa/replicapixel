use crate::domain::website::Website;
use loco_rs::prelude::*;

use super::auth::UserView;

pub fn payment_home(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
    route: String,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/payment_plans/payment_plan_partial.html",
        data!({"plans": website.payment_plans, "website": website.website_settings, "user_p": user, "payment_route": route}),
    )
}

pub fn payment_success(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/stripe/stripe_completed_partial.html",
        data!({"plans": website.payment_plans, "website": website.website_settings}),
    )
}

pub fn payment_completed(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/stripe/stripe_completed_partial.html",
        data!({"plans": website.payment_plans, "website": website.website_settings}),
    )
}

pub fn payment_cancel(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/payment_base.html",
        data!({"plans": website.payment_plans, "website": website.website_settings}),
    )
}

pub fn payment_status(
    v: impl ViewRenderer,
    website: &Website,
    params: &Option<String>,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/stripe/stripe_payment.html",
        data!({"plans": website.payment_plans, "website": website.website_settings, "session_id": params}),
    )
}
