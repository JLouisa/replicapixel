use loco_rs::prelude::*;

use crate::{
    controllers::images::{ImageDomainList, ImageGenRequestForm},
    domain::image::Image,      //training_models::ImageDomain},
    models::_entities::images, // user_credits::UserCreditsDomain},
};

// pub fn one(
//     v: &impl ViewRenderer,
//     credits: &UserCreditsDomain,
//     image_list: &ImageDomainList,
//     check_route: String,
// ) -> Result<Response> {
//     format::render().view(
//         v,
//         "dashboard/content/photo/image_loading_partial.html",
//         data!({"credits": credits, "image_list": image_list, "check_route": check_route}),
//     )
// }

pub fn img_completed(v: &impl ViewRenderer, images: &Vec<Image>) -> Result<Response> {
    format::render().view(
        v,
        "dashboard/content/photo/image_partial.html",
        data!({"images": images}),
    )
}

/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<images::Model>) -> Result<Response> {
    format::render().view(v, "images/list.html", data!({"items": items}))
}

/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &images::Model) -> Result<Response> {
    format::render().view(v, "images/show.html", data!({"item": item}))
}

/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "images/create.html", data!({}))
}

/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &images::Model) -> Result<Response> {
    format::render().view(v, "images/edit.html", data!({"item": item}))
}
