use crate::domain::website::Website;
use loco_rs::prelude::*;

pub fn home(
    v: impl ViewRenderer,
    website: &Website,
    validate_route: &str,
    is_home: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home.html",
        data!({"website": website.website_settings,"validate_route": validate_route, "is_home": is_home}),
    )
}
