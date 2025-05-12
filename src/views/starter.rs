// src/controllers/dashboard.rs
use crate::domain::website::Website;
use loco_rs::prelude::*;

pub fn starter(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "starter/starter_base.html",
        data!({ "website": website }),
    )
}

pub fn starter_partial(v: impl ViewRenderer, website: &Website) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "starter/sections/part_one.html",
        data!({ "website": website }),
    )
}
