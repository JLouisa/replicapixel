use crate::domain::website::Website;
use loco_rs::prelude::*;

pub fn home(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home.html",
        data!({"website": website.website_settings}),
    )
}
