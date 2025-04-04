use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use fluent_templates::{ArcLoader, FluentLoader};
use loco_rs::{
    app::{AppContext, Initializer},
    controller::views::{engines, ViewEngine},
    Error, Result,
};
use tracing::info;

use crate::{domain::settings::Settings, service::aws::s3::AwsS3};

#[allow(clippy::module_name_repetitions)]
pub struct S3;

#[async_trait]
impl Initializer for S3 {
    fn name(&self) -> String {
        "s3".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let s3_settings: Settings =
            serde_json::from_value(ctx.config.settings.clone().expect("No settings found"))
                .expect("Failed to parse settings");

        let aws_client = AwsS3::new(&s3_settings.aws).await;

        let router = router.layer(Extension(aws_client));
        Ok(router)
    }
}
