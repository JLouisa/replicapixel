use crate::{domain::website::Website, middleware::cookie::CookieConsent};
use axum::Extension;
use loco_rs::prelude::*;

pub fn home(
    v: impl ViewRenderer,
    website: &Website,
    validate_route: &str,
    is_home: bool,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    let concent_scripts = website..;
    format::render().view(
        &v,
        "home/home.html",
        data!(
                {
                "website": website.website_settings, "cc_cookie": cc_cookie,
                "validate_route": validate_route, "is_home": is_home
                }
        ),
    )
}
