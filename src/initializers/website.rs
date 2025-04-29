use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::domain::settings::Settings;
use crate::domain::website::Website as WebsiteInit;

#[allow(clippy::module_name_repetitions)]
pub struct Website;

#[async_trait]
impl Initializer for Website {
    fn name(&self) -> String {
        "website".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let website = Settings::init(&ctx).website();
        let router = router.layer(Extension(website));
        Ok(router)
    }
}
