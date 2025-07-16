use crate::domain::settings::Settings;
use crate::middleware::cookie::CookieConsentLayer;
#[allow(unused_imports)]
use crate::{
    controllers, initializers, models::_entities::users, tasks, workers::downloader::DownloadWorker,
};
use async_trait::async_trait;
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

// use crate::middleware::i18n::I18n;
// use axum::Router as AxumRouter;

// use loco_rs::controller::middleware::{self, MiddlewareLayer};

// #[derive(Clone)]
// pub struct I18nMiddlewareLayer;

// impl MiddlewareLayer for I18nMiddlewareLayer {
//     fn name(&self) -> &'static str {
//         "i18n-path-rewriter"
//     }

//     // This config can be empty as we don't configure it from a file.
//     fn config(&self) -> serde_json::Result<serde_json::Value> {
//         Ok(serde_json::json!({}))
//     }

//     /// This is the key. The `apply` function takes the router and returns
//     /// a new router wrapped in our middleware.
//     fn apply(&self, app: AxumRouter<AppContext>) -> Result<AxumRouter<AppContext>> {
//         // Apply your `tower::Layer` to the router.
//         Ok(app.layer(I18n::new()))
//     }
// }

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
            Box::new(initializers::meta_client::MetaConversionApi),
            Box::new(initializers::stripe::Stripe),
            Box::new(initializers::redis::RedisClient),
            Box::new(initializers::axum_session::AxumSessionInitializer),
            Box::new(initializers::oauth2::OAuth2StoreInitializer),
        ])
    }

    // async fn before_routes(_ctx: &AppContext) -> Result<AxumRouter<AppContext>> {
    //     let router = AxumRouter::new().layer(I18n::new());
    //     Ok(router)
    // }

    // fn middlewares(ctx: &AppContext) -> Vec<Box<dyn MiddlewareLayer>> {
    //     let mut default_stack = middleware::default_middleware_stack(ctx);
    //     default_stack.push(Box::new(I18nMiddlewareLayer));
    //     default_stack
    // }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        let mut routes = AppRoutes::with_default_routes()
            .add_route(controllers::video::routes())
            .add_route(controllers::other::routes())
            .add_route(controllers::starter::routes())
            .add_route(controllers::packs::routes())
            .add_route(controllers::settings::routes())
            .add_route(controllers::features::routes())
            .add_route(controllers::oauth2::routes())
            .add_route(controllers::payment::routes().layer(CookieConsentLayer::new()))
            .add_route(controllers::images::routes())
            .add_route(controllers::home::routes().layer(CookieConsentLayer::new()))
            .add_route(controllers::dashboard::routes().layer(CookieConsentLayer::new()))
            .add_route(controllers::training_models::routes())
            .add_route(controllers::webhooks::routes())
            .add_route(controllers::policy::routes())
            .add_route(controllers::auth::routes().layer(CookieConsentLayer::new()));

        if cfg!(debug_assertions) {
            routes = routes.add_route(controllers::admin::routes());
        }
        routes
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue
            .register(crate::workers::meta_worker::MetaConversionApiWorker::build(
                ctx,
            ))
            .await?;
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    // async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
    //     let router_with_i18n = router.layer(I18n::new());
    //     Ok(router_with_i18n)
    // }

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
    //         cache: cache::Cache::new(cache::drivers::inmem::new()).into(),
    //         storage: Storage::single(storage::drivers::local::new_with_prefix("storage")?).into(),
    //         ..ctx
    //     })
    // }
    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        use loco_rs::cache;
        use loco_rs::storage;
        use loco_rs::storage::drivers::aws;
        use loco_rs::storage::{
            drivers::StoreDriver, strategies, strategies::StorageStrategy, Storage,
        };
        use std::collections::BTreeMap;

        let aws_settings = Settings::init(&ctx).aws;
        let credential = aws::Credential {
            key_id: aws_settings.access_key_id.to_string(),
            secret_key: aws_settings.secret_access_key.to_string(),
            token: None,
        };
        let aws_driver = aws::with_credentials(
            &aws_settings.s3.bucket_name,
            &aws_settings.s3.region,
            credential,
        )?;
        // let aws_store: Box<dyn storage::drivers::StoreDriver> = Box::new(aws_init);

        let local_driver = storage::drivers::local::new_with_prefix("storage").unwrap();

        let mut backends: BTreeMap<String, Box<dyn StoreDriver>> = BTreeMap::new();
        backends.insert("default".to_string(), local_driver);
        backends.insert("aws".to_string(), aws_driver);

        let strategy: Box<dyn StorageStrategy> =
            Box::new(strategies::single::SingleStrategy::new("default"));

        let storage = Storage::new(backends, strategy);

        Ok(AppContext {
            cache: cache::Cache::new(cache::drivers::inmem::new()).into(),
            storage: storage.into(),
            ..ctx
        })
    }
}
