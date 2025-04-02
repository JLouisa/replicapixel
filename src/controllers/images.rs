// #![allow(clippy::missing_errors_doc)]
// #![allow(clippy::unnecessary_struct_initialization)]
// #![allow(clippy::unused_async)]
// use loco_rs::prelude::*;

// use crate::models::_entities::training_models;
// use crate::{
//     controllers::PictureParams,
//     domain::active_enums::ImageSize,
//     domain::{
//         active_enums::{ImageFormat, Status},
//         image::Image,
//         training_models::{ImageDomain, ImageGenerateDomain, TrainingDomain, UserPrompt},
//     },
//     models::{ImagesActiveModel, TrainingActiveModel, _entities::images::ActiveModel},
//     models::{
//         _entities::images::{Entity, Model},
//         user_credits::UserCreditsDomain,
//         users::{self, UserDomain},
//     },
//     service::fal_ai::fal_client::{FalAiClient, FluxLoraImageGenerate},
//     views,
// };
// use axum::{debug_handler, Extension};
// use axum::{http::HeaderMap, http::StatusCode, response::IntoResponse, Json};
// use derive_more::{AsRef, Constructor, Display};
// use serde::{Deserialize, Serialize};
// use training_models::Model as TrainingModel;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct ImageGenRequestForm {
//     pub training_model_id: i32,
//     pub prompt: UserPrompt,
//     pub image_size: ImageSize,
//     pub num_inference_steps: u16,
//     pub num_images: u8,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, Constructor, AsRef)]
// pub struct ImageDomainList(Vec<ImageDomain>);
// impl ImageDomainList {
//     pub fn into_inner(self) -> Vec<ImageDomain> {
//         self.0
//     }
//     pub fn extend(&mut self, list: ImageDomainList) {
//         self.0.extend(list.0);
//     }
//     pub async fn save_all(&self, ctx: &AppContext) -> Result<(), loco_rs::Error> {
//         let mut list: Vec<Params> = self
//             .as_ref()
//             .iter()
//             .cloned()
//             .map(|img_data| img_data.into())
//             .collect();

//         // Bulk insert
//         let models: Vec<ActiveModel> = list
//             .iter()
//             // .iter_mut()
//             .map(|img| {
//                 let mut item = ActiveModel {
//                     ..Default::default()
//                 };
//                 img.update(&mut item);
//                 item
//             })
//             .collect();
//         let results = Entity::insert_many(models).exec(&ctx.db).await?;
//         Ok(())
//     }
// }
// impl From<Vec<ImageDomain>> for ImageDomainList {
//     fn from(list: Vec<ImageDomain>) -> Self {
//         Self::new(list)
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct Params {
//     pub pid: Uuid,
//     pub user_id: i32,
//     pub training_model_id: i32,
//     pub pack_id: Option<i32>,
//     pub user_prompt: String,
//     pub sys_prompt: String,
//     pub num_inference_steps: i32,
//     pub content_type: String,
//     pub status: Status,
//     pub image_size: ImageSize,
//     pub fal_ai_request_id: Option<String>,
//     pub width: Option<i32>,
//     pub height: Option<i32>,
//     pub image_url: Option<String>,
//     pub image_url_s3: Option<String>,
//     pub is_favorite: bool,
//     pub deleted_at: Option<DateTimeWithTimeZone>,
// }

// impl Params {
//     fn update(&self, item: &mut ActiveModel) {
//         item.pid = Set(self.pid.clone());
//         item.user_id = Set(self.user_id.clone());
//         item.training_model_id = Set(self.training_model_id.clone());
//         item.pack_id = Set((self.pack_id.clone()));
//         item.user_prompt = Set(self.user_prompt.clone());
//         item.sys_prompt = Set(self.sys_prompt.clone());
//         item.num_inference_steps = Set(self.num_inference_steps.clone());
//         item.content_type = Set(self.content_type.clone());
//         item.status = Set(self.status.to_string());
//         item.image_size = Set(self.image_size.to_string());
//         item.fal_ai_request_id = Set(self.fal_ai_request_id.clone());
//         item.width = Set(self.width.clone());
//         item.height = Set(self.height.clone());
//         item.image_url = Set(self.image_url.clone());
//         item.image_url_s3 = Set(self.image_url_s3.clone());
//         item.is_favorite = Set(self.is_favorite);
//         item.deleted_at = Set(self.deleted_at.clone());
//     }
// }

// impl From<ImageDomain> for Params {
//     fn from(item: ImageDomain) -> Self {
//         Self {
//             pid: item.pid,
//             user_id: item.user_id,
//             training_model_id: item.training_model_id,
//             pack_id: item.pack_id,
//             user_prompt: item.user_prompt.into_inner(),
//             sys_prompt: item.sys_prompt.into_inner(),
//             num_inference_steps: item.num_inference_steps,
//             content_type: item.content_type.to_string(),
//             status: item.status,
//             image_size: item.image_size,
//             fal_ai_request_id: item.fal_ai_request_id,
//             width: item.width,
//             height: item.height,
//             image_url: item.image_url,
//             image_url_s3: item.image_url_s3,
//             is_favorite: item.is_favorite,
//             deleted_at: item.deleted_at,
//         }
//     }
// }

// impl From<ImageDomainList> for Vec<Params> {
//     fn from(item: ImageDomainList) -> Self {
//         let list = item
//             .into_inner()
//             .into_iter()
//             .map(|img| img.into())
//             .collect::<Vec<Params>>();
//         list
//     }
// }

// pub mod routes {
//     use serde::Serialize;

//     #[derive(Clone, Debug, Serialize)]
//     pub struct Images;
//     impl Images {
//         pub const BASE: &'static str = "/api/images";
//         pub const IMAGE: &'static str = "/";
//         pub const IMAGE_GENERATE: &'static str = "/generate";
//         pub const IMAGE_GENERATE_TEST: &'static str = "/generate/test";
//         pub const IMAGE_CHECK_TEST: &'static str = "/check/test/{id}";
//         pub const IMAGE_CHECK_ID: &'static str = "/check/{id}";
//         pub const IMAGE_CHECK: &'static str = "/check";
//         pub const IMAGE_ID: &'static str = "/{id}";
//         pub const IMAGE_BASE: &'static str = "";
//     }
// }

// pub fn routes() -> Routes {
//     Routes::new()
//         .prefix(routes::Images::BASE)
//         .add(routes::Images::IMAGE, get(list))
//         .add(routes::Images::IMAGE, post(add))
//         .add(routes::Images::IMAGE_GENERATE_TEST, post(generate_test))
//         .add(routes::Images::IMAGE_GENERATE, post(generate_img))
//         .add(routes::Images::IMAGE_CHECK_TEST, get(check_test))
//         .add(routes::Images::IMAGE_ID, get(get_one))
//         .add(routes::Images::IMAGE_ID, delete(remove))
//         .add(routes::Images::IMAGE_ID, put(update))
//         .add(routes::Images::IMAGE_ID, patch(update))
// }

// #[debug_handler]
// pub async fn check_test(
//     Path(pid): Path<Uuid>,
//     State(ctx): State<AppContext>,
//     ViewEngine(v): ViewEngine<TeraView>,
// ) -> Result<Response> {
//     use rand::Rng;

//     let change = rand::rng().random_range(0..=2);
//     let num = rand::rng().random_range(1..=11);
//     if change == 0 {
//         let image = Image::test(num);
//         return views::images::img_completed(&v, &image);
//     }
//     Ok((StatusCode::NO_CONTENT).into_response())
// }

// #[debug_handler]
// pub async fn generate_test(
//     auth: auth::JWT,
//     State(ctx): State<AppContext>,
//     Extension(fal_ai_client): Extension<FalAiClient>,
//     ViewEngine(v): ViewEngine<TeraView>,
//     Json(request): Json<ImageGenRequestForm>,
// ) -> Result<Response> {
//     if request.num_images < 1 || request.num_images > 20 {
//         return Err(loco_rs::Error::Message("Wrong image amount".to_string()).into());
//     }

//     let user = UserDomain::load(&ctx.db, &auth.claims.pid).await?;
//     let train_model: TrainingDomain = TrainingModel::find_by_id(&ctx.db, request.training_model_id)
//         .await?
//         .into();

//     if train_model.user_id != user.id {
//         return Err(loco_rs::Error::Message("Unauthorized".to_string()).into());
//     }

//     let mut user_credits = UserCreditsDomain::load_item_by_user_id(&ctx, &user).await?;

//     if (user_credits.credit_amount - request.num_images as i32) < 1 {
//         return Err(loco_rs::Error::Message("Not enough credits".to_string()).into());
//     }

//     let image_gen: ImageGenerateDomain = ImageGenerateDomain::process(request, &train_model);
//     let image_list = image_gen.from();

//     // Send Image to FAL AI API Queue
//     let mut response = fal_ai_client
//         .send_image_queue_many_async(image_list.clone())
//         .await?;
//     let mut response = fal_ai_client.retry(response, image_list.clone()).await?;

//     // Save in DB
//     response.clone().save_all(&ctx).await?;

//     user_credits.credit_amount = user_credits.credit_amount - response.as_ref().len() as i32;
//     // user_credits.credit_amount = user_credits.credit_amount - image_list.as_ref().len() as i32;

//     let user_credits = user_credits.save(&ctx).await?;

//     let check_route = format!(
//         "{}{}/test",
//         routes::Images::BASE,
//         routes::Images::IMAGE_CHECK
//     );

//     // views::images::one(&v, &user_credits, &image_list)
//     views::images::one(&v, &user_credits, &response, check_route)
// }

// #[debug_handler]
// pub async fn generate_img(
//     auth: auth::JWT,
//     State(ctx): State<AppContext>,
//     Extension(fal_ai_client): Extension<FalAiClient>,
//     Json(request): Json<ImageGenRequestForm>,
// ) -> Result<Response> {
//     let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
//     let train_model: TrainingDomain = TrainingModel::find_by_id(&ctx.db, request.training_model_id)
//         .await?
//         .into();
//     if train_model.user_id != user.id {
//         return Err(loco_rs::Error::Message("Unauthorized".to_string()).into());
//     }
//     // let list_img = request.create_img_many(user.id);
//     let list_img: ImageDomainList = ImageGenerateDomain::process(request, &train_model).into();

//     //Send Image to FAL AI API Queue
//     let mut response = fal_ai_client
//         .send_image_queue_many_async(list_img.clone())
//         .await?;
//     let mut response = fal_ai_client.retry(response, list_img).await?;

//     // Bulk insert
//     let models = response.save_all(&ctx).await?;

//     let mut headers = HeaderMap::new();
//     headers.insert("HX-Redirect", "/dashboard/partial/camera".parse().unwrap()); // HTMX Redirect

//     Ok((StatusCode::OK, headers).into_response())
// }

// async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
//     let item = Entity::find_by_id(id).one(&ctx.db).await?;
//     item.ok_or_else(|| Error::NotFound)
// }

// #[debug_handler]
// pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
//     format::json(Entity::find().all(&ctx.db).await?)
// }

// #[debug_handler]
// pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
//     let mut item = ActiveModel {
//         ..Default::default()
//     };
//     params.update(&mut item);
//     let item = item.insert(&ctx.db).await?;
//     format::json(item)
// }

// #[debug_handler]
// pub async fn update(
//     Path(id): Path<i32>,
//     State(ctx): State<AppContext>,
//     Json(params): Json<Params>,
// ) -> Result<Response> {
//     let item = load_item(&ctx, id).await?;
//     let mut item = item.into_active_model();
//     params.update(&mut item);
//     let item = item.update(&ctx.db).await?;
//     format::json(item)
// }

// #[debug_handler]
// pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
//     load_item(&ctx, id).await?.delete(&ctx.db).await?;
//     format::empty()
// }

// #[debug_handler]
// pub async fn get_one(
//     Path(id): Path<i32>,
//     State(ctx): State<AppContext>,
//     ViewEngine(v): ViewEngine<TeraView>,
// ) -> Result<Response> {
//     let item = load_item(&ctx, id).await?;

//     // views::contacts::show(&v, &item)
//     format::empty()
// }
