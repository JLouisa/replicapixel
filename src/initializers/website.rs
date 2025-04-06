use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use fluent_templates::{ArcLoader, FluentLoader};
use loco_rs::{
    app::{AppContext, Initializer},
    controller::views::{engines, ViewEngine},
    Error, Result,
};
use tracing::info;

use crate::domain::{
    settings::Settings,
    website::{Website as WebsiteInit, WebsiteSettings},
};

#[allow(clippy::module_name_repetitions)]
pub struct Website;

#[async_trait]
impl Initializer for Website {
    fn name(&self) -> String {
        "website".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let settings: Settings =
            serde_json::from_value(ctx.config.settings.clone().expect("No settings found"))
                .expect("Failed to parse settings");

        let website_settings = settings.website;
        let website = WebsiteInit::init(website_settings);

        let router = router.layer(Extension(website));
        Ok(router)
    }
}
