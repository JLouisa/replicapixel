#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;

use crate::{
    domain::{features::FeatureView, website::Website},
    models::{
        feature_request::FeatureParams, users::UserPid, FeatureRequestActiveModel,
        FeatureRequestEntity, FeatureRequestModel, UserModel,
    },
    views,
};

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FeatureRoutes {
        pub base: String,
        pub add: String,
        pub vote: String,
    }
    impl FeatureRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Features::BASE),
                add: format!("{}{}", Features::BASE, Features::FEATURE_ADD),
                vote: format!("{}{}", Features::BASE, Features::FEATURE_VOTE),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Features;
    impl Features {
        pub const BASE: &'static str = "/api/features";
        pub const FEATURE_ADD: &'static str = "/add";
        pub const FEATURE_VOTE_ID: &'static str = "/vote/{id}";
        pub const FEATURE_VOTE: &'static str = "/vote";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Features::BASE)
        .add(routes::Features::FEATURE_ADD, post(add))
        .add(routes::Features::FEATURE_VOTE_ID, patch(vote))
}

async fn load_user(db: &DatabaseConnection, pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, &pid.as_ref().to_string()).await?;
    Ok(item)
}
async fn load_item(ctx: &AppContext, id: i32) -> Result<FeatureRequestModel> {
    let item = FeatureRequestEntity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}
async fn vote_on_feature(
    db: &DatabaseConnection,
    feature_id: i32,
    user_id: i32,
) -> Result<(FeatureRequestModel, bool)> {
    let feature = FeatureRequestActiveModel::toggle_vote_count(db, user_id, feature_id).await?;
    Ok(feature)
}

#[debug_handler]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Extension(website): Extension<Website>,
    Json(params): Json<FeatureParams>,
) -> Result<impl IntoResponse> {
    dbg!(&params);
    params.validate()?;
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;

    if user.id != params.user_id {
        return Err(Error::Unauthorized(
            "Unauthorized to create feature suggestion".to_string(),
        ));
    }

    let mut item = FeatureRequestActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    views::features::form_reset(v, &website, &user.into())
}

#[debug_handler]
pub async fn vote(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Extension(website): Extension<Website>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    let feature = load_item(&ctx, id).await?;
    let feature_processed = vote_on_feature(&ctx.db, feature.id, user.id).await?;
    let feature_view = FeatureView::process_one(&feature_processed.0, feature_processed.1);
    views::features::vote_update(v, &website, &feature_view)
}
