use loco_rs::prelude::*;

use crate::domain::website::WebsiteOptions;

pub fn home(v: impl ViewRenderer, website_options: &WebsiteOptions) -> Result<impl IntoResponse> {
    format::render().view(&v, "home/home.html", data!({ "options": website_options }))
}

pub fn home_partial(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn google_ott(
    v: &impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        v,
        "partials/parts/validated/validated_ott.html",
        data!({ "options": website_options }),
    )
}
