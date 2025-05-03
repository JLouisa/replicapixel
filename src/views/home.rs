use crate::{domain::website::Website, middleware::cookie::CookieConsent};
use loco_rs::prelude::*;

pub fn home(
    v: impl ViewRenderer,
    website: &Website,
    is_home: bool,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home.html",
        data!({ "website": website, "cc_cookie": cc_cookie, "is_home": is_home }),
    )
}

pub fn home2(
    v: impl ViewRenderer,
    website: &Website,
    is_home: bool,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home2.html",
        data!({ "website": website, "cc_cookie": cc_cookie, "is_home": is_home }),
    )
}
