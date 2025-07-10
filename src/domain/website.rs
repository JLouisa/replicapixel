use derive_more::Constructor;
use loco_rs::app::AppContext;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use tokio::join;

use crate::controllers::auth::AuthError;
use crate::controllers::dashboard::routes::DashboardRoutes;
use crate::controllers::dashboard::CurrentPage;
use crate::controllers::home::{load_packs_translated, load_pricing_translated};
use crate::controllers::other::routes::OtherRoutes;
use crate::controllers::packs::routes::PackRoutes;
use crate::controllers::payment::StripeWebOptions;
use crate::controllers::settings::routes::SettingRoutes;
use crate::controllers::starter::routes::StarterRoutes;
use crate::controllers::{
    auth::routes::AuthRoutes, features::routes::FeatureRoutes, home::routes::HomeRoutes,
    images::routes::ImageRoutes, oauth2::routes::OAuth2Routes, payment::routes::PaymentRoutes,
    policy::routes::PolicyRoutes, training_models::routes::TrainingModelRoutes,
};
use crate::domain::features::{FeatureView, FeatureViewList};
use crate::domain::settings::Settings;
use crate::middleware::cookie::CookieConsent;
use crate::models::users::{LoginParams, RegisterParams};
use crate::models::PlanModel;
use crate::models::_entities::sea_orm_active_enums::ImageSize;
use crate::models::_entities::sea_orm_active_enums::Language;
use crate::models::_entities::sea_orm_active_enums::{BasedOn, Emotion, Ethnicity, EyeColor, Sex};
use crate::service::fal_ai::fal_client::FalAiImageModel;
use crate::service::redis::redis::RedisCacheDriver;
use crate::views::auth::{UserCreditsView, UserView};
use crate::views::dashboard::TransactionViewList;
use crate::views::images::{ImageView, ImageViewList};
use crate::views::packs::{PackView, PackViewList};
use crate::views::payment::{PricingView, PricingViewList};
use crate::views::settings::UserSettingsView;
use crate::views::training_models::{TrainingModelView, TrainingModelViewList};

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
    pub other_routes: OtherRoutes,
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
            other_routes: OtherRoutes::init(),
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
    pub meta_pixel_id: Option<i64>,
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
    pub quality_model: Vec<(String, String)>,
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
                quality_model: FalAiImageModel::to_fields(),
            },
            website_home: WebsiteHome {
                pricing,
                reviews: HomeReview::reviews(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct WebGallery {
    images_r0: Vec<String>,
    images_r1: Vec<String>,
    images_r2: Vec<String>,
    images_r3: Vec<String>,
    images_r4: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
struct WebBeforeAfter {
    before: String,
    after: String,
}

#[derive(Debug, Serialize, Deserialize, Constructor, Clone)]
pub struct WebImages {
    hero_panel: Vec<String>,
    pub gallery: WebGallery,
    before_after: WebBeforeAfter,
    studio: String,
    pub packs: PackViewList,
    creators: Vec<String>,
    plans: PricingViewList,
}
impl WebImages {
    pub fn packs(&self) -> &PackViewList {
        &self.packs
    }
    pub async fn web_images(
        db: &DatabaseConnection,
        lang: &Language,
        cache: &RedisCacheDriver,
    ) -> WebImages {
        // Loading functions concurrently.
        let (packs_result, plans_result) = join!(
            load_packs_translated(db, lang, cache),
            load_pricing_translated(db, lang, cache)
        );

        let packs: PackViewList = match packs_result {
            Ok(packs) => packs,
            Err(e) => {
                tracing::error!("Failed to load packs: {}", e);
                PackViewList::default()
            }
        }
        .into();
        let plans = match plans_result {
            Ok(plans) => plans,
            Err(e) => {
                tracing::error!("Failed to load plans: {}", e);
                PricingViewList::default()
            }
        };

        // Randomize gallary images
        let mut packs_cloned = packs.clone().into_inner();
        let mut rng = rng();
        let web_images0 = get_web_img_urls(&mut rng, &mut packs_cloned);
        let web_images1 = get_web_img_urls(&mut rng, &mut packs_cloned);
        let web_images2 = get_web_img_urls(&mut rng, &mut packs_cloned);
        let web_images3 = get_web_img_urls(&mut rng, &mut packs_cloned);
        let web_images4 = get_web_img_urls(&mut rng, &mut packs_cloned);

        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/nature-hero.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/corporate-headshot.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/mma-fe.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/wife1.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/street-fighter.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/nature3.webp"),
        // ];
        // let web_images1 = vec![
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/nature2.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/cosplay/f193a28b-83e3-4a1c-b13e-3637acb85c84.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/sexy+halloween/e5557da7-416a-466c-a5a7-bf7232232ee3.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/cosplay1-small.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/machina2.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/cosplay2-small.webp"),
        // ];
        // let web_images2 = vec![
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/cosplay/a97bb59a-be4f-4b3f-92b5-e8c25a03e361.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/machina1.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/angel.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/cosplay/f193a28b-83e3-4a1c-b13e-3637acb85c84.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/emo-girl.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/blackwidow.webp"),
        // ];
        // let web_images3 = vec![
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/sexy+halloween/a22ec84c-dcd7-4cbd-b872-1963aa140355.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/cosplay/53d42133-d8be-47a8-863b-1a489b2a736e.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/nature1.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/dracula-wife.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/cosplay3.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/model-show.webp"),
        // ];
        // let web_images4 = vec![
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/spiritual/e1ee3b51-53a0-4254-9a09-8d734ea7195a.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/easter1.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/model-makeup.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/white-dress.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/gallery/model-closeup.webp"),
        //     String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/sexy+halloween/f861732f-79ed-4c0d-904d-c43b714807c8.webp"),
        // ];

        let hero_panel = vec![
            String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/sexy+halloween/a22ec84c-dcd7-4cbd-b872-1963aa140355.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/nature/f40a699f-8064-4015-80d2-ffb68228ac2e.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/cosplay/f193a28b-83e3-4a1c-b13e-3637acb85c84.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/cosplay/a97bb59a-be4f-4b3f-92b5-e8c25a03e361.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/cosplay/53d42133-d8be-47a8-863b-1a489b2a736e.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/sexy-valentine/fcf51df7-27d6-48ad-a34e-a96a78ddeb02.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/packs/spiritual/3b7e781a-6b40-4ef8-8d58-b52bcabddc87.webp"),
        ];
        // let web_images0 = vec![

        let gallery = WebGallery::new(
            web_images0,
            web_images1,
            web_images2,
            web_images3,
            web_images4,
        );
        let before_after = WebBeforeAfter::new(
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/home-before.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/home-after.webp"),
        );

        let studio = String::from("https://d2npyy9ae7osp9.cloudfront.net/others/studio.webp");

        let creators = vec![
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/got.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/dynasty.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/cosplay-widow.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/elf-queen.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/dynasty2.webp"),
            String::from("https://d2npyy9ae7osp9.cloudfront.net/others/cosplay-lara.webp"),
        ];

        let web_images = WebImages::new(
            hero_panel,
            gallery,
            before_after,
            studio,
            packs,
            creators,
            plans,
        );
        web_images
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

/// Removes and returns a random item from the vector.
/// Returns `None` if the vector is empty.
// pub fn pop_random_item<T>(vec: &mut Vec<T>) -> Option<T> {
//     if vec.is_empty() {
//         return None;
//     }
//     let mut rng = rng();
//     let idx = rng.random_range(0..vec.len());
//     Some(vec.remove(idx))
// }

fn pop_random_item<T>(rng: &mut ThreadRng, vec: &mut Vec<T>) -> T {
    let idx = rng.random_range(0..vec.len());
    vec.remove(idx)
}

fn get_image_url(rng: &mut ThreadRng, packs: PackView) -> String {
    let mut images = match packs.images {
        Some(list) => list,
        None => unreachable!("All Packs should have images"),
    };
    let image_url = pop_random_item(rng, &mut images);
    image_url
}

fn get_web_img_urls(rng: &mut ThreadRng, packs: &mut Vec<PackView>) -> Vec<String> {
    let mut images = Vec::new();
    for _ in 0..=5 {
        let pack = pop_random_item(rng, packs);
        let image_url = get_image_url(rng, pack);
        images.push(image_url);
    }
    images
}

#[derive(Serialize, Default)]
pub struct MarketingPurchase {
    pub value: String,
    pub currency: String,
}

#[derive(Serialize, Default)]
#[must_use]
pub struct WebsiteOptions<'a> {
    pub website: Option<&'a Website>,
    pub language: Language,
    pub cc_cookie: Option<&'a CookieConsent>,
    pub current_page: Option<CurrentPage>,
    pub user: Option<UserView>,
    pub user_credits: Option<UserCreditsView>,
    pub orders: Option<&'a TransactionViewList>,
    pub plan: Option<&'a PricingView>,
    pub plans: Option<HashMap<i32, PlanModel>>,
    pub feature: Option<&'a FeatureView>,
    pub features: Option<&'a FeatureViewList>,
    pub user_settings: Option<UserSettingsView>,
    pub training_model: Option<TrainingModelView>,
    pub training_models: Option<TrainingModelViewList>,
    pub pack: Option<PackView>,
    pub packs: Option<&'a PackViewList>,
    pub pack_images: Option<WebGallery>,
    pub image: Option<&'a ImageView>,
    pub images: Option<&'a ImageViewList>,
    pub web_gallery: Option<&'a WebGallery>,
    pub web_images: Option<&'a WebImages>,
    pub link: Option<&'a str>,
    pub message: Option<&'a str>,
    pub register: Option<&'a RegisterParams>,
    pub login: Option<&'a LoginParams>,
    pub auth_error: Option<&'a AuthError>,
    pub stripe_options: Option<&'a StripeWebOptions>,
    pub marketing_purchase: Option<MarketingPurchase>,
    pub is_marketing_initiate_checkout: bool,
    pub is_marketing_purchase: bool,
    pub is_logged_in: bool,
    pub is_ott: bool,
    pub is_home: bool,
    pub is_initial_load: bool,
    pub is_pack_partial: bool,
    pub is_pack: bool,
    pub is_deleted: bool,
    pub is_favorite: bool,
    pub is_image_gen: bool,
    pub is_other: bool,
    pub is_production: bool,
}

impl<'a> WebsiteOptions<'a> {
    /// Creates a new, empty set of options to begin a builder chain.
    /// This is an alias for `WebsiteOptions::default()`.
    pub fn new() -> Self {
        Self {
            is_production: !cfg!(debug_assertions),
            ..Self::default()
        }
    }
    /// Sets the options required for a full-page layout.
    pub fn website(self, website: &'a Website) -> Self {
        Self {
            website: Some(website),
            ..self
        }
    }
    /// Sets the options required for a full-page layout.
    pub fn language(self, language: &'a Language) -> Self {
        Self {
            language: language.clone(),
            ..self
        }
    }
    /// Sets the concent of cookies.
    pub fn cc_cookie(self, cc_cookie: &'a CookieConsent) -> Self {
        Self {
            cc_cookie: Some(cc_cookie),
            ..self
        }
    }
    /// Sets the authenticated user.
    pub fn user(self, user: UserView) -> Self {
        Self {
            user: Some(user),
            ..self
        }
    }
    /// Sets the authenticated user.
    pub fn set_user(self, user: Option<UserView>) -> Self {
        Self { user, ..self }
    }
    /// Sets the user's credits.
    pub fn user_credits(self, user_credits: UserCreditsView) -> Self {
        Self {
            user_credits: Some(user_credits),
            ..self
        }
    }
    /// Sets the user's orders.
    pub fn orders(self, orders: &'a TransactionViewList) -> Self {
        Self {
            orders: Some(orders),
            ..self
        }
    }
    // Sets the current page.
    pub fn current_page(self, current_page: CurrentPage) -> Self {
        Self {
            current_page: Some(current_page),
            ..self
        }
    }
    // Sets the features.
    pub fn feature(self, feature: &'a FeatureView) -> Self {
        Self {
            feature: Some(feature),
            ..self
        }
    }
    // Sets the features.
    pub fn features(self, features: &'a FeatureViewList) -> Self {
        Self {
            features: Some(features),
            ..self
        }
    }
    // Sets the settings.
    pub fn user_settings(self, settings: UserSettingsView) -> Self {
        Self {
            user_settings: Some(settings),
            ..self
        }
    }
    // Sets a training model.
    pub fn training_model(self, training_model: TrainingModelView) -> Self {
        Self {
            training_model: Some(training_model),
            ..self
        }
    }
    // Sets the training models.
    pub fn training_models(self, training_models: TrainingModelViewList) -> Self {
        Self {
            training_models: Some(training_models),
            ..self
        }
    }
    // Sets one pack.
    pub fn pack(self, pack: PackView) -> Self {
        Self {
            pack: Some(pack),
            ..self
        }
    }
    pub fn set_pack(self, pack: Option<PackView>) -> Self {
        Self { pack, ..self }
    }
    // Sets the packs.
    pub fn packs(self, packs: &'a PackViewList) -> Self {
        Self {
            packs: Some(packs),
            ..self
        }
    }
    // Sets the packs.
    pub fn pack_images(self, packs: WebGallery) -> Self {
        Self {
            pack_images: Some(packs),
            ..self
        }
    }
    // Sets the plan.
    pub fn plan(self, plan: &'a PricingView) -> Self {
        Self {
            plan: Some(plan),
            ..self
        }
    }
    // Sets the images.
    pub fn image(self, image: &'a ImageView) -> Self {
        Self {
            image: Some(image),
            ..self
        }
    }
    // Sets the images.
    pub fn images(self, images: &'a ImageViewList) -> Self {
        Self {
            images: Some(images),
            ..self
        }
    }
    // Sets the web gallery.
    pub fn web_gallery(self, web_gallery: &'a WebGallery) -> Self {
        Self {
            web_gallery: Some(web_gallery),
            ..self
        }
    }
    // Sets the web images.
    pub fn web_images(self, web_images: &'a WebImages) -> Self {
        Self {
            web_images: Some(web_images),
            ..self
        }
    }
    // Sets the link.
    pub fn link(self, link: &'a str) -> Self {
        Self {
            link: Some(link),
            ..self
        }
    }
    pub fn marketing_purchase(self, value: f64) -> Self {
        Self {
            marketing_purchase: Some(MarketingPurchase {
                value: value.to_string(),
                currency: "USD".to_string(),
            }),
            ..self
        }
    }
    // Sets the message.
    pub fn message(self, message: &'a str) -> Self {
        Self {
            message: Some(message),
            ..self
        }
    }
    // Sets the register params.
    pub fn register(self, register: &'a RegisterParams) -> Self {
        Self {
            register: Some(register),
            ..self
        }
    }
    // Sets the register params.
    pub fn login(self, login: &'a LoginParams) -> Self {
        Self {
            login: Some(login),
            ..self
        }
    }
    // Sets the auth error.
    pub fn auth_error(self, auth_error: &'a AuthError) -> Self {
        Self {
            auth_error: Some(auth_error),
            ..self
        }
    }
    // Sets the stripe options.
    pub fn stripe_options(self, stripe_options: &'a StripeWebOptions) -> Self {
        Self {
            stripe_options: Some(stripe_options),
            ..self
        }
    }
    // Sets the bool for is_logged_in.
    pub fn is_logged_in(self) -> Self {
        Self {
            is_logged_in: true,
            ..self
        }
    }
    // Sets the bool for is_home.
    pub fn is_home(self) -> Self {
        Self {
            is_home: true,
            ..self
        }
    }
    // Sets the bool for is_deleted.
    pub fn is_deleted(self) -> Self {
        Self {
            is_deleted: true,
            ..self
        }
    }
    // Sets the bool for is_favorite.
    pub fn is_favorite(self) -> Self {
        Self {
            is_favorite: true,
            ..self
        }
    }
    // Sets the bool for is_initial_load.
    pub fn is_initial_load(self) -> Self {
        Self {
            is_initial_load: true,
            ..self
        }
    }
    // Sets the bool for is_image_gen.
    pub fn is_image_gen(self) -> Self {
        Self {
            is_image_gen: true,
            ..self
        }
    }
    // Sets the bool for is_pack_partial.
    pub fn is_pack_partial(self) -> Self {
        Self {
            is_pack_partial: true,
            ..self
        }
    }
    // Sets the bool for is_pack.
    pub fn is_pack(self) -> Self {
        Self {
            is_pack: true,
            ..self
        }
    }
    // Sets the bool for google ott.
    pub fn is_ott(self) -> Self {
        Self {
            is_ott: true,
            ..self
        }
    }
    // Sets the bool for is_other.
    pub fn is_other(self) -> Self {
        Self {
            is_other: true,
            ..self
        }
    }
    // Sets the bool for is_production.
    pub fn is_production(self) -> Self {
        Self {
            is_production: !cfg!(debug_assertions),
            ..self
        }
    }
    // Sets the bool for is_logged_in.
    pub fn is_marketing_initiate_checkout(self) -> Self {
        Self {
            is_marketing_initiate_checkout: true,
            ..self
        }
    }
    // Sets the bool for is_logged_in.
    pub fn is_marketing_purchase(self) -> Self {
        Self {
            is_marketing_purchase: true,
            ..self
        }
    }
    // Returns the built struct.
    pub fn build(self) -> Self {
        self
    }
}
