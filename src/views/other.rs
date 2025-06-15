use loco_rs::prelude::*;

use crate::domain::website::WebsiteOptions;

pub fn documentation(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "documentation/documentation.html",
        data!({ "options": website_options }),
    )
}
