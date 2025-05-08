use crate::{domain::website::Website, middleware::cookie::CookieConsent};
use loco_rs::prelude::*;

use super::auth::{UserCreditsView, UserView};

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

pub fn home_partial(
    v: impl ViewRenderer,
    website: &Website,
    is_home: bool,
    cc_cookie: &CookieConsent,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home_partial.html",
        data!({ "website": website, "cc_cookie": cc_cookie, "is_home": is_home }),
    )
}

pub fn google_ott(
    v: &impl ViewRenderer,
    website: &Website,
    user: &UserView,
    credits: &UserCreditsView,
) -> Result<impl IntoResponse> {
    format::render().view(
        v,
        "partials/parts/home_validated.html",
        data!({"website": website, "user": user, "ott": true}),
    )
}
