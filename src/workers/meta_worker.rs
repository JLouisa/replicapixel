use derive_more::Constructor;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    domain::website::MetaPixel,
    service::meta::meta::{EventData, MetaConversionApiClient},
};

pub struct MetaConversionApiWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize, Clone, Constructor)]
pub struct MetaConversionApiWorkerArgs {
    pub data: EventData,
    pub meta: MetaPixel,
}

#[async_trait]
impl BackgroundWorker<MetaConversionApiWorkerArgs> for MetaConversionApiWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
    async fn perform(&self, _args: MetaConversionApiWorkerArgs) -> Result<()> {
        let client = MetaConversionApiClient::new(&_args.meta);
        client.meta_conversion_api(&_args.data).await?;
        Ok(())
    }
}
