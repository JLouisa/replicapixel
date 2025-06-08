use crate::controllers::dashboard::WebsiteOptions;
use loco_rs::prelude::*;

pub fn home(v: impl ViewRenderer, website_options: &WebsiteOptions) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home.html",
        data!(
            {
                "website": website_options.website, "cc_cookie": website_options.cc_cookie,
                "is_home": website_options.is_home, "web_images": website_options.web_images,
                "user": website_options.user, "options": website_options
            }
        ),
    )
}

pub fn home_partial(
    v: impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "home/home_partial.html",
        data!(
            {
                "website": website_options.website, "cc_cookie": website_options.cc_cookie,
                "is_home": website_options.is_home, "web_images": website_options.web_images,
                "user": website_options.user, "options": website_options
            }
        ),
    )
}

pub fn google_ott(
    v: &impl ViewRenderer,
    website_options: &WebsiteOptions,
) -> Result<impl IntoResponse> {
    format::render().view(
        v,
        "partials/parts/validated/validated_ott.html",
        data!(
            {
                "website": website_options.website, "user": website_options.user,
                "is_ott": website_options.is_ott, "options": website_options
            }
        ),
    )
}
