use loco_rs::prelude::*;

use super::{auth::UserView, packs::PackViewList};

pub fn packs(
    v: impl ViewRenderer,
    packs: &PackViewList,
    user: &UserView,
    is_img: bool,
) -> Result<impl IntoResponse> {
    format::render().view(
        &v,
        "admin/packs.html",
        data!(
            {
                "user": user, "packs": packs, "is_img": is_img
            }
        ),
    )
}

pub fn packs_form_partial(
    v: impl ViewRenderer,
    packs: &PackViewList,
    user: &UserView,
    is_img: bool,
) -> Result<impl IntoResponse> {
    let key = if is_img {
        "admin/pack_form_img_partial.html"
    } else {
        "admin/pack_form_partial.html"
    };
    format::render().view(
        &v,
        key,
        data!(
            {
                "user": user, "packs": packs, "is_img": is_img
            }
        ),
    )
}
