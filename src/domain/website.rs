use crate::controllers::dashboard::routes::DashboardRoutes;
use crate::controllers::packs::routes::PackRoutes;
use crate::controllers::payment::PricingViewList;
use crate::controllers::settings::routes::SettingRoutes;
use crate::controllers::starter::routes::StarterRoutes;
use crate::controllers::{
    auth::routes::AuthRoutes, features::routes::FeatureRoutes, home::routes::HomeRoutes,
    images::routes::ImageRoutes, oauth2::routes::OAuth2Routes, payment::routes::PaymentRoutes,
    policy::routes::PolicyRoutes, training_models::routes::TrainingModelRoutes,
};
use crate::domain::settings::Settings;
use crate::models::PlanModel;
use crate::models::_entities::sea_orm_active_enums::Language;
use crate::models::_entities::sea_orm_active_enums::{BasedOn, Emotion, Ethnicity, EyeColor, Sex};
use crate::models::_entities::sea_orm_active_enums::{ImageSize, PlanNames};
use derive_more::Constructor;
use loco_rs::app::AppContext;
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
    pub languages: Vec<Language>,
    pub create_model: CreateModel,
}

#[derive(Debug, Clone, Deserialize, Serialize, Constructor, Default)]
pub struct WebsiteHome {
    pub pricing: PricingViewList,
    pub reviews: Vec<HomeReview>,
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
    pub website_home: WebsiteHome,
}
impl Website {
    pub async fn init(settings: &Settings, ctx: &AppContext) -> Website {
        let pricing: PricingViewList = match PlanModel::find_all(&ctx.db).await {
            Ok(pricing) => pricing.into(),
            Err(e) => {
                tracing::error!("Failed to load pricing: {}", e);
                std::process::exit(1);
            }
        };
        Website {
            website_basic_info: settings.website.clone(),
            website_routes: WebsiteRoutes::init(),
            website_fields: WebsiteFormFields {
                image_sizes: ImageSize::iter()
                    .map(|s| (s.clone(), s.to_string()))
                    .collect::<Vec<_>>(),
                languages: Language::iter().collect(),
                create_model: CreateModel::init(),
            },
            website_home: WebsiteHome {
                pricing,
                reviews: HomeReview::reviews(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HomeReview {
    pub id: i32,
    pub text: String,
    pub name: String,
    pub job_title: String,
    pub rating: Vec<u8>,
    pub image_url: String,
    pub is_big: bool,
}
impl HomeReview {
    pub fn reviews() -> Vec<HomeReview> {
        let list = vec![
    HomeReview {
        id: 1,
        text: "This AI has completely transformed my workflow for social media visuals! I can generate eye-catching, unique images in minutes instead of hours searching stock photos.".to_owned(),
        name: "Alisha Khan".to_owned(),
        job_title: "Digital Marketing Lead".to_owned(),
        rating: vec![1; 5], // 5 stars
        image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
        is_big: false,
    },
    HomeReview {
        id: 2,
        text: "As a blogger, coming up with fresh featured images was a constant struggle. Now, I just describe my article's theme, and the AI delivers stunning, relevant artwork. My readers love it!".to_owned(),
        name: "Ben Carter".to_owned(),
        job_title: "Indie Content Creator".to_owned(),
        rating: vec![1; 5], // 5 stars
        image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
        is_big: false,
    },
    HomeReview {
        id: 3,
        text: "Great for brainstorming and rapid prototyping visual concepts for my clients. While I still do final touches, it saves hours on initial ideation and mood boarding. The variety is impressive.".to_owned(),
        name: "Chloe Davis".to_owned(),
        job_title: "Freelance Graphic Designer".to_owned(),
        rating: vec![1; 4], // 4 stars
        image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
        is_big: false,
    },
    HomeReview {
        id: 4,
        text: "Our startup needed professional-looking ad creatives and website banners on a tight budget. This AI tool is a game-changer, allowing us to produce high-quality assets without hiring an expensive agency.".to_owned(),
        name: "Marcus Reid".to_owned(),
        job_title: "Founder, EcoBloom Solutions".to_owned(),
        rating: vec![1; 5], // 5 stars
        image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
        is_big: false,
    },
    HomeReview {
        id: 5,
        text: "Incredibly intuitive! I'm not a designer, but I can create stunning images for presentations, social posts, and even personal art projects. The range of styles and prompt flexibility is amazing.".to_owned(),
        name: "Olivia Chen".to_owned(),
        job_title: "Educator & Hobbyist Artist".to_owned(),
        rating: vec![1; 4], // 4 stars
        image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
        is_big: false,
    },
    HomeReview {
        id: 6,
        text: "The image generation is top-notch. The content suggestions for captions and short descriptions are a good starting point, helping me get past writer's block quickly for my e-commerce product listings.".to_owned(),
        name: "Samuel Green".to_owned(),
        job_title: "E-commerce Manager".to_owned(),
        rating: vec![1; 5], // 5 stars
        image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
        is_big: false,
    },
    // // Adding a couple more for variety
    // HomeReview {
    //     id: 7,
    //     text: "I use this AI to generate unique textures and patterns for my 3D models and game assets. It's a fantastic tool for adding detail and originality that would take ages to create manually.".to_owned(),
    //     name: "Kevin Park".to_owned(),
    //     job_title: "Indie Game Developer".to_owned(),
    //     rating: vec![1; 5], // 5 stars
    //     image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
    //     is_big: true,
    // },
    // HomeReview {
    //     id: 8,
    //     text: "This AI is an incredible playground for creativity! I'm constantly amazed by the novel concepts and artistic styles I can explore. It's like having an artistic co-pilot that pushes my boundaries.".to_owned(),
    //     name: "Isabelle Moreau".to_owned(),
    //     job_title: "Concept Artist".to_owned(),
    //     rating: vec![1; 5], // 5 stars
    //     image_url: "https://img.daisyui.com/images/profile/demo/spiderperson@192.webp".to_owned(),
    //     is_big: false,
    // },
];
        list
    }
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct Feature(String);

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
pub fn mock_plans() -> Vec<Plan> {
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
                Feature::new("50 AI Photo Credits".to_owned()),
                Feature::new("1 AI Model".to_owned()),
                Feature::new("No monthly subscription!".to_owned()),
                Feature::new("Use any photo pack".to_owned()),
                Feature::new("No Watermarked photos".to_owned()),
                Feature::new("24/7 Support".to_owned()),
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
                Feature::new("250 AI Photos (credits)".to_owned()),
                Feature::new("7 AI Model".to_owned()),
                Feature::new("No monthly subscription!".to_owned()),
                Feature::new("Use any photo pack".to_owned()),
                Feature::new("No Watermarked photos".to_owned()),
                Feature::new("24/7 Support".to_owned()),
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
                Feature::new("1100 AI Photos (credits)".to_owned()),
                Feature::new("16 AI Model".to_owned()),
                Feature::new("No monthly subscription!".to_owned()),
                Feature::new("Use any photo pack".to_owned()),
                Feature::new("No Watermarked photos".to_owned()),
                Feature::new("24/7 Support".to_owned()),
            ],
            cta: "Choose Max".to_owned(),
            is_popular: false,
        }
    }
}
