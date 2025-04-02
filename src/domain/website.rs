// use super::{dashboard_sidebar::DashboardSidebar, packs::Packs};
// use crate::{controllers::dashboard::routes, models::_entities::sea_orm_active_enums::ImageSize};
// use serde::Serialize;
// use strum::{EnumIter, IntoEnumIterator};

// #[derive(Debug, Serialize)]
// pub struct Website {
//     pub name: String,
//     pub url: String,
//     pub dashboard_sidebar: DashboardSidebar,
//     pub image_sizes: (Vec<(ImageSize, String)>),
//     pub sidebar_routes: routes::Sidebar,
//     pub packs: Vec<Packs>,
// }
// impl Website {
//     pub fn init() -> Website {
//         Website {
//             name: String::from("DreamPhoto"),
//             url: String::from("https://dreamphoto.ai"),
//             dashboard_sidebar: DashboardSidebar::init(),
//             image_sizes: ImageSize::iter()
//                 .map(|s| (s.clone(), s.to_string()))
//                 .collect::<Vec<_>>(),

//             sidebar_routes: routes::Dashboard::sidebar(),
//             packs: Packs::get_packs(),
//         }
//     }
// }
