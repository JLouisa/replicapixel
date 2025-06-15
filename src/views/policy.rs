use loco_rs::prelude::*;

use crate::domain::website::WebsiteOptions;

pub fn cookie(v: impl ViewRenderer, website_options: &WebsiteOptions) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "policies/cookie-policy.html",
        data!({ "options": website_options }),
    )
}

pub fn terms(v: impl ViewRenderer, website_options: &WebsiteOptions) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "policies/term-conditions.html",
        data!({ "options": website_options }),
    )
}

pub fn privacy(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "policies/privacy-policy.html",
        data!({ "options": website_options }),
    )
}

pub fn model_consent(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "policies/model-consent-policy.html",
        data!({ "options": website_options }),
    )
}
