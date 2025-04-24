use crate::domain::{features::FeatureView, website::Website};
use loco_rs::prelude::*;

use super::auth::UserView;

pub fn vote_update(
    v: impl ViewRenderer,
    website: &Website,
    feature: &FeatureView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/vote_count_partial.html",
        data!({ "website": website, "feature": feature }),
    )
}

pub fn form_reset(
    v: impl ViewRenderer,
    website: &Website,
    user: &UserView,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "dashboard/content/features/feature_form.html",
        data!({ "website": website, "user": user }),
    )
}
