// use super::active_enums::{Ethnicity, EyeColor, ImageFormat, ImageSize, Sex, Status, TypeModel};
// use crate::{
//     controllers::{
//         images::{ImageDomainList, ImageGenRequestForm},
//         training_models::Params,
//         PictureParams,
//     },
//     models::{self, pictures::ActiveModel, TrainingModel},
// };
// use derive_more::{AsRef, Constructor};
// use sea_orm::prelude::DateTimeWithTimeZone;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct ImageGenerateDomain {
//     pub user_id: i32,
//     pub training_model_id: i32,
//     pub user_prompt: UserPrompt,
//     pub sys_prompt: SysPrompt,
//     pub image_size: ImageSize,
//     pub num_inference_steps: u16,
//     pub num_images: u8,
// }
// impl ImageGenerateDomain {
//     pub fn process(form: ImageGenRequestForm, model: &TrainingDomain) -> Self {
//         Self {
//             user_id: model.user_id,
//             training_model_id: form.training_model_id,
//             image_size: form.image_size,
//             num_inference_steps: form.num_inference_steps,
//             num_images: form.num_images,
//             sys_prompt: form.prompt.formatted_prompt(model),
//             user_prompt: form.prompt,
//         }
//     }

//     pub fn process_par(form: ImageGenRequestForm, model: &TrainingDomain) -> Vec<PictureParams> {
//         let sys_prompt = form.prompt.formatted_prompt(model);
//         (0..form.num_images)
//             .map(|_| PictureParams {
//                 pid: Uuid::new_v4(),
//                 user_id: model.user_id,
//                 training_model_id: form.training_model_id,
//                 sys_prompt: sys_prompt.as_ref().to_owned(),
//                 user_prompt: form.prompt.as_ref().to_owned(),
//                 num_inference_steps: form.num_inference_steps as i32,
//                 content_type: ImageFormat::Jpeg.to_string(),
//                 status: Status::Pending.to_string(),
//                 image_size: form.image_size.to_string(),
//                 fal_ai_request_id: None,
//                 width: None,
//                 height: None,
//                 image_url: None,
//                 image_url_s3: None,
//                 is_favorite: false,
//                 deleted_at: None,
//             })
//             .collect()
//     }

//     pub fn from(self) -> ImageDomainList {
//         let mut list: Vec<ImageDomain> = Vec::with_capacity(self.num_images as usize);
//         for _ in 0..self.num_images {
//             let img = self.clone().into();
//             list.push(img);
//         }
//         ImageDomainList::new(list)
//     }
// }

// impl From<ImageGenerateDomain> for ImageDomainList {
//     fn from(form: ImageGenerateDomain) -> Self {
//         let list = (0..form.num_images)
//             .map(|_| ImageDomain::from(form.clone())) // Only shallow clone
//             .collect();
//         ImageDomainList::new(list)
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct ImageDomain {
//     pub pid: Uuid,
//     pub user_id: i32,
//     pub training_model_id: i32,
//     pub pack_id: Option<i32>,
//     pub user_prompt: UserPrompt,
//     pub sys_prompt: SysPrompt,
//     pub num_inference_steps: i32,
//     pub content_type: ImageFormat,
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

// impl From<ImageGenerateDomain> for ImageDomain {
//     fn from(form: ImageGenerateDomain) -> Self {
//         Self {
//             pid: Uuid::new_v4(),
//             user_id: form.user_id,
//             training_model_id: form.training_model_id,
//             pack_id: None,
//             user_prompt: form.user_prompt,
//             sys_prompt: form.sys_prompt,
//             num_inference_steps: form.num_inference_steps as i32,
//             content_type: ImageFormat::Jpeg,
//             image_size: form.image_size,
//             status: Status::Pending,
//             fal_ai_request_id: None,
//             width: None,
//             height: None,
//             image_url: None,
//             image_url_s3: None,
//             is_favorite: false,
//             deleted_at: None,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct FluxLoraImageGenerate {
//     pub prompt: SysPrompt,
//     pub image_size: ImageSize,
//     pub num_inference_steps: u16,
//     pub guidance_scale: f32,
//     pub num_images: u8,
//     pub enable_safety_checker: bool,
//     pub output_format: ImageFormat,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, AsRef, PartialEq)]
// pub struct UserPrompt(String);
// impl UserPrompt {
//     pub fn new<K: Into<String>>(key: K) -> Self {
//         Self(key.into())
//     }
//     pub fn into_inner(self) -> String {
//         self.0
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize, AsRef, PartialEq)]
// pub struct SysPrompt(String);
// impl SysPrompt {
//     pub fn new<K: Into<String>>(key: K) -> Self {
//         Self(key.into())
//     }
//     pub fn into_inner(self) -> String {
//         self.0
//     }
// }

// impl UserPrompt {
//     pub fn formatted_prompt(&self, training_model: &TrainingDomain) -> SysPrompt {
//         let sys_prompt = format!(
//             "{} {}. The subject is a {} {} {} with {} eyes, aged {}. {}.",
//             training_model.trigger_word,
//             self.0,
//             training_model.ethnicity,
//             training_model.sex,
//             if training_model.bald {
//                 "who is bald"
//             } else {
//                 ""
//             },
//             training_model.eye_color,
//             training_model.age,
//             match training_model.type_model {
//                 TypeModel::BasedRealPerson => "The model is a real person.",
//                 TypeModel::CreateInfluencerAI => "The model is AI-generated.",
//             }
//         );

//         SysPrompt(sys_prompt)
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct TrainingDomain {
//     pub id: i32,
//     pub pid: Uuid,
//     pub user_id: i32,
//     pub name: String,
//     pub sex: Sex,
//     pub age: i16,
//     pub eye_color: EyeColor,
//     pub bald: bool,
//     pub creative: i32,
//     pub create_mask: bool,
//     pub is_style: bool,
//     pub type_model: TypeModel,
//     pub ethnicity: Ethnicity,
//     pub file_type: ImageFormat,
//     pub training_status: Status,
//     pub trigger_word: String, //
// }

// impl From<TrainingModel> for TrainingDomain {
//     fn from(model: TrainingModel) -> Self {
//         let sex = match model.sex.as_str() {
//             "Male" => Sex::Male,
//             "Female" => Sex::Female,
//             _ => Sex::Male,
//         };
//         let eye_color = match model.eye_color.as_str() {
//             "Brown" => EyeColor::Brown,
//             "Blue" => EyeColor::Blue,
//             "Green" => EyeColor::Green,
//             "Grey" => EyeColor::Grey,
//             "Hazel" => EyeColor::Hazel,
//             "Red" => EyeColor::Red,
//             _ => EyeColor::Brown,
//         };
//         let type_model = match model.type_model.as_str() {
//             "BasedRealPerson" => TypeModel::BasedRealPerson,
//             "CreateInfluencerAI" => TypeModel::CreateInfluencerAI,
//             _ => TypeModel::BasedRealPerson,
//         };
//         let ethnicity = match model.ethinicity.as_str() {
//             "White" => Ethnicity::White,
//             "Black" => Ethnicity::Black,
//             "Pacific" => Ethnicity::Pacific,
//             "Hispanic" => Ethnicity::Hispanic,
//             "Asian" => Ethnicity::Asian,
//             "SouthEastAsian" => Ethnicity::SouthEastAsian,
//             "SouthAsian" => Ethnicity::SouthAsian,
//             "MiddleEastern" => Ethnicity::MiddleEastern,
//             _ => Ethnicity::White,
//         };
//         let training_status = match model.training_status.unwrap().as_str() {
//             "Completed" => Status::Completed,
//             "Pending" => Status::Pending,
//             "Processing" => Status::Processing,
//             "Training" => Status::Training,
//             "Failed" => Status::Failed,
//             "Canceled" => Status::Cancelled,
//             _ => Status::Pending,
//         };
//         Self {
//             id: model.id,
//             pid: model.pid,
//             user_id: model.user_id,
//             name: model.name,
//             sex,
//             age: model.age,
//             eye_color,
//             bald: model.bald,
//             creative: 28,
//             create_mask: model.create_mask,
//             is_style: model.is_style,
//             type_model,
//             ethnicity,
//             file_type: ImageFormat::Zip,
//             training_status,
//             trigger_word: model.trigger_word,
//         }
//     }
// }
