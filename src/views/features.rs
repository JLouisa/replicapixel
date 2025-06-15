use loco_rs::prelude::*;

use crate::domain::website::WebsiteOptions;

pub fn vote_update(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/vote_count_partial.html",
        data!({ "options": website_options }),
    )
}

pub fn form_reset(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/feature_form.html",
        data!({ "options": website_options }),
    )
}
