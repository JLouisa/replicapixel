use crate::controllers::dashboard::WebsiteOptions;
use loco_rs::prelude::*;

pub fn payment_home_partial(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/payment_plans/payment_plan_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn payment_home(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/payment_plans/payment_plan.html",
        data!({ "options": website_options }),
    )
}

pub fn prepare(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "base_stripe.html",
        data!({ "options": website_options }),
    )
}

pub fn stripe_status(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/stripe/stripe.html",
        data!({ "options": website_options }),
    )
}

pub fn stripe_status_partials(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "payment/stripe/stripe_partials.html",
        data!({ "options": website_options }),
    )
}
