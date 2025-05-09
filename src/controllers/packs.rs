#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::{
    domain::{domain_services::image_generation::ImageGenerationService, website::Website},
    models::{
        images::ImagesModelList,
        join::{
            user_credits_models::load_user_credit_training,
            user_pack::load_user_one_training_model_one_pack,
        },
        users::UserPid,
        ImageModel,
        _entities::sea_orm_active_enums::ImageSize,
    },
    service::{aws::s3::AwsS3, fal_ai::fal_client::FalAiClient, redis::redis::RedisCacheDriver},
    views::{self, images::ImageViewList},
};

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PackRoutes {
        pub base: String,
        pub gen_pack: String,
    }
    impl PackRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Pack::BASE),
                gen_pack: format!("{}{}", Pack::BASE, Pack::GEN_PACK),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Pack;
    impl Pack {
        pub const BASE: &'static str = "/api/pack";
        pub const GEN_PACK_PID: &'static str = "/gen/{pid}";
        pub const GEN_PACK: &'static str = "/gen";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(routes::Pack::BASE)
        .add(routes::Pack::GEN_PACK_PID, post(generate_packs_images))
}

async fn load_first_images(
    db: &DatabaseConnection,
    id: i32,
    fav: bool,
    del: bool,
) -> Result<ImagesModelList> {
    let list = ImageModel::find_x_images_by_user_id(db, id, fav, del, 30).await?;
    Ok(ImagesModelList::new(list))
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackParams {
    // pack_pid: Uuid,
    model_pid: Uuid,
    image_size: ImageSize,
}

#[debug_handler]
pub async fn generate_packs_images(
    auth: auth::JWT,
    Path(pack_pid): Path<Uuid>,
    Extension(fal_ai_client): Extension<FalAiClient>,
    Extension(s3_client): Extension<AwsS3>,
    Extension(cache): Extension<RedisCacheDriver>,
    Extension(website): Extension<Website>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(form): Json<PackParams>,
) -> Result<impl IntoResponse> {
    // 1. Load User, Pack and Training Model
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, training_model, pack) =
        load_user_one_training_model_one_pack(&ctx.db, &user_pid, &form.model_pid, &pack_pid)
            .await?;

    dbg!(&user);
    dbg!(&training_model);
    dbg!(&pack);

    // 2. Call the Domain Service to perform the core logic
    let (_, _) = ImageGenerationService::generate_with_packs(
        &ctx,
        &fal_ai_client,
        pack,
        &user,
        &training_model,
        &form.image_size,
    )
    .await?;

    // 3. Render the view using the View Models
    let is_deleted = false;
    let is_favorite = false;
    let images: ImageViewList = load_first_images(&ctx.db, user.id, is_favorite, is_deleted)
        .await?
        .into();
    let images = images.populate_s3_pre_urls(&s3_client, &cache).await;
    let (_, user_credits, training_models) = load_user_credit_training(&ctx.db, &user_pid).await?;

    views::dashboard::photo_partial_dashboard(
        v,
        &website,
        &images,
        training_models.into(),
        &user_credits.into(),
        is_deleted,
        is_favorite,
    )
}
