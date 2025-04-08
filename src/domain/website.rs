use super::{dashboard_sidebar::DashboardSidebar, packs::Packs};
use crate::{
    controllers::dashboard::routes,
    models::_entities::sea_orm_active_enums::{ImageSize, PlanNames},
};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use stripe::PriceId;
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, Clone, Deserialize, Serialize, Constructor, Default)]
pub struct WebsiteSettings {
    pub site: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct Website {
    pub website_settings: WebsiteSettings,
    pub dashboard_sidebar: DashboardSidebar,
    pub image_sizes: (Vec<(ImageSize, String)>),
    pub sidebar_routes: routes::Sidebar,
    pub packs: Vec<Packs>,
    pub payment_plans: Vec<Plan>,
}
impl Website {
    pub fn init(settings: WebsiteSettings) -> Website {
        Website {
            website_settings: settings,
            dashboard_sidebar: DashboardSidebar::init(),
            image_sizes: ImageSize::iter()
                .map(|s| (s.clone(), s.to_string()))
                .collect::<Vec<_>>(),

            sidebar_routes: routes::Dashboard::sidebar(),
            packs: Packs::get_packs(),
            payment_plans: get_plans(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feature {
    pub bold: Option<bool>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: String,
    pub plan_type: PlanNames,
    pub title: String,
    pub subtitle: String,
    pub price: f64,
    pub features: Vec<Feature>,
    pub cta: String,
    pub is_popular: bool,
}
pub fn get_plans() -> Vec<Plan> {
    vec![Plan::basic(), Plan::premium(), Plan::max()]
}
impl Plan {
    pub fn basic() -> Self {
        Self {
            id: "basic".to_owned(),
            plan_type: PlanNames::Basic,
            title: "Basic".to_owned(),
            subtitle: "For individuals".to_owned(),
            price: 9.99,
            features: vec![
                Feature {
                    bold: Some(true),
                    description: "60 AI Photos (credits)".to_owned(),
                },
                Feature {
                    bold: Some(true),
                    description: "1 AI Model".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "No monthly subscription!".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "Use any photo pack".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "No Watermarked photos".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "24/7 Support".to_owned(),
                },
            ],
            cta: "Choose Basic".to_owned(),
            is_popular: false,
        }
    }
    pub fn premium() -> Self {
        Self {
            id: "premium".to_owned(),
            plan_type: PlanNames::Premium,
            title: "For professionals".to_owned(),
            subtitle: "For large teams".to_owned(),
            price: 39.99,
            features: vec![
                Feature {
                    bold: Some(true),
                    description: "300 AI Photos (credits)".to_owned(),
                },
                Feature {
                    bold: Some(true),
                    description: "7 AI Model".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "No monthly subscription!".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "Use any photo pack".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "No Watermarked photos".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "24/7 Support".to_owned(),
                },
            ],
            cta: "Choose Premium".to_owned(),
            is_popular: true,
        }
    }
    pub fn max() -> Self {
        Self {
            id: "max".to_owned(),
            plan_type: PlanNames::Max,
            title: "Business".to_owned(),
            subtitle: "For large teams".to_owned(),
            price: 99.99,
            features: vec![
                Feature {
                    bold: Some(true),
                    description: "1100 AI Photos (credits)".to_owned(),
                },
                Feature {
                    bold: Some(true),
                    description: "16 AI Model".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "No monthly subscription!".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "Use any photo pack".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "No Watermarked photos".to_owned(),
                },
                Feature {
                    bold: Some(false),
                    description: "24/7 Support".to_owned(),
                },
            ],
            cta: "Choose Max".to_owned(),
            is_popular: false,
        }
    }
}
