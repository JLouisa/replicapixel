use crate::controllers::dashboard::routes::DashboardRoutes;
use crate::controllers::packs::routes::PackRoutes;
use crate::controllers::settings::routes::SettingRoutes;
use crate::controllers::starter::routes::StarterRoutes;
use crate::controllers::{
    auth::routes::AuthRoutes, features::routes::FeatureRoutes, home::routes::HomeRoutes,
    images::routes::ImageRoutes, oauth2::routes::OAuth2Routes, payment::routes::PaymentRoutes,
    policy::routes::PolicyRoutes, training_models::routes::TrainingModelRoutes,
};
use crate::domain::settings::Settings;
use crate::models::_entities::sea_orm_active_enums::Language;
use crate::models::_entities::sea_orm_active_enums::{BasedOn, Emotion, Ethnicity, EyeColor, Sex};
use crate::models::_entities::sea_orm_active_enums::{ImageSize, PlanNames};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct WebsiteRoutes {
    pub auth_routes: AuthRoutes,
    pub dashboard_routes: DashboardRoutes,
    pub feature_routes: FeatureRoutes,
    pub home: HomeRoutes,
    pub image: ImageRoutes,
    pub oauth2: OAuth2Routes,
    pub payment: PaymentRoutes,
    pub policy: PolicyRoutes,
    pub training_models: TrainingModelRoutes,
    pub settings: SettingRoutes,
    pub packs_routes: PackRoutes,
    pub starter_routes: StarterRoutes,
}
impl WebsiteRoutes {
    pub fn init() -> Self {
        Self {
            auth_routes: AuthRoutes::init(),
            dashboard_routes: DashboardRoutes::init(),
            feature_routes: FeatureRoutes::init(),
            home: HomeRoutes::init(),
            image: ImageRoutes::init(),
            oauth2: OAuth2Routes::init(),
            payment: PaymentRoutes::init(),
            policy: PolicyRoutes::init(),
            training_models: TrainingModelRoutes::init(),
            settings: SettingRoutes::init(),
            packs_routes: PackRoutes::init(),
            starter_routes: StarterRoutes::init(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GoogleAnalytics {
    pub google_client_id: String,
    pub google_analytics_id: Option<String>,
    pub google_analytics_secret: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MetaPixel {
    pub meta_pixel_id: Option<String>,
    pub meta_pixel_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Constructor, Default)]
pub struct WebsiteBasicInfo {
    pub name: String,
    pub site: String,
    pub site_domain: String,
    pub from_email: String,
    pub google: GoogleAnalytics,
    pub meta_pixel: MetaPixel,
}
impl WebsiteBasicInfo {
    pub fn from_mail(&self) -> String {
        format!("{} <{}@{}>", self.name, self.from_email, self.site_domain)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Constructor, Default)]
pub struct WebsiteFormFields {
    pub image_sizes: Vec<(ImageSize, String)>,
    pub payment_plans: Vec<Plan>,
    pub languages: Vec<Language>,
    pub create_model: CreateModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateModel {
    type_model: Vec<BasedOn>,
    ethnicity: Vec<Ethnicity>,
    eye_color: Vec<EyeColor>,
    emotion: Vec<Emotion>,
    sex: Vec<Sex>,
}
impl CreateModel {
    pub fn init() -> Self {
        Self {
            type_model: BasedOn::iter().collect(),
            sex: Sex::iter().collect(),
            ethnicity: Ethnicity::iter().collect(),
            eye_color: EyeColor::iter().collect(),
            emotion: Emotion::iter().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct Website {
    pub website_basic_info: WebsiteBasicInfo,
    pub website_routes: WebsiteRoutes,
    pub website_fields: WebsiteFormFields,
}
impl Website {
    pub fn init(settings: &Settings) -> Website {
        Website {
            website_basic_info: settings.website.clone(),
            website_routes: WebsiteRoutes::init(),
            website_fields: WebsiteFormFields {
                image_sizes: ImageSize::iter()
                    .map(|s| (s.clone(), s.to_string()))
                    .collect::<Vec<_>>(),
                payment_plans: get_plans(),
                languages: Language::iter().collect(),
                create_model: CreateModel::init(),
            },
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
                    description: "50 AI Photos (credits)".to_owned(),
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
                    description: "250 AI Photos (credits)".to_owned(),
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
