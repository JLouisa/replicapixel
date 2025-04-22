use crate::domain::website::Website;
use loco_rs::prelude::*;

pub fn cookie(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "policies/cookie-policy.html",
        data!({"website": website.website_settings}),
    )
}

pub fn terms(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "policies/term-conditions.html",
        data!({"website": website.website_settings}),
    )
}

pub fn privacy(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "policies/privacy-policy.html",
        data!({"website": website.website_settings}),
    )
}
