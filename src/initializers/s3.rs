use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

use crate::{domain::settings::Settings, service::aws::s3::AwsS3};

#[allow(clippy::module_name_repetitions)]
pub struct S3;

#[async_trait]
impl Initializer for S3 {
    fn name(&self) -> String {
        "s3".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let s3_client = Settings::init(&ctx).aws_s3().await;
        let router = router.layer(Extension(s3_client));
        Ok(router)
    }
}
