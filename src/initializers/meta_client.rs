use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::domain::settings::Settings;

#[allow(clippy::module_name_repetitions)]
pub struct MetaConversionApi;

#[async_trait]
impl Initializer for MetaConversionApi {
    fn name(&self) -> String {
        "meta_client".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let meta_client = Settings::init(&ctx).meta_conversion_api();
        let router = router.layer(Extension(meta_client));
        Ok(router)
    }
}
