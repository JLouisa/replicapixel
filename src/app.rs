#[allow(unused_imports)]
use crate::{
    controllers, initializers, models::_entities::users, tasks, workers::downloader::DownloadWorker,
};
use async_trait::async_trait;
use loco_rs::cache;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(initializers::view_engine::ViewEngineInitializer),
            Box::new(initializers::website::Website),
            Box::new(initializers::s3::S3),
            Box::new(initializers::fal_client::FalAi),
            Box::new(initializers::stripe::Stripe),
            Box::new(initializers::redis::RedisClient),
            Box::new(initializers::axum_session::AxumSessionInitializer),
            Box::new(initializers::oauth2::OAuth2StoreInitializer),
        ])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::starter::routes())
            .add_route(controllers::packs::routes())
            .add_route(controllers::settings::routes())
            .add_route(controllers::features::routes())
            .add_route(controllers::oauth2::routes())
            .add_route(controllers::payment::routes())
            .add_route(controllers::images::routes())
            .add_route(controllers::home::routes())
            .add_route(controllers::dashboard::routes())
            .add_route(controllers::training_models::routes())
            .add_route(controllers::webhooks::routes())
            .add_route(controllers::policy::routes())
            .add_route(controllers::auth::routes())
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        Ok(())
    }
    // async fn after_context(ctx: AppContext) -> Result<AppContext> {
    //     Ok(AppContext {
    //         cache: cache::Cache::new(Box::new(Settings::init(&ctx).redis().await)).into(),
    //         ..ctx
    //     })
    // }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        Ok(AppContext {
            cache: cache::Cache::new(cache::drivers::inmem::new()).into(),
            ..ctx
        })
    }
}
