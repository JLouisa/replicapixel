use crate::service::{aws::s3::AwsS3, fal_ai::fal_client::Lora};

pub use super::_entities::packs::{ActiveModel, Entity, Model};
use super::{
    PackModel, TrainingModelModel,
    _entities::{
        plans,
        sea_orm_active_enums::{ImageFormat, ImageSize, Status},
    },
    images::{AltText, ImageNew, ImageNewList, SysPrompt, UserPrompt},
};
use derive_more::{AsRef, Constructor};
use sea_orm::entity::prelude::*;
pub type Packs = Entity;
use loco_rs::prelude::*;

#[derive(Debug, Clone, AsRef, Constructor)]
pub struct PackModelList(pub Vec<PackModel>);

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    pub fn process_packs(
        self,
        model: &TrainingModelModel,
        user_pid: &Uuid,
        image_size: &ImageSize,
    ) -> ImageNewList {
        let sys_prompt = SysPrompt::new(&self.pack_prompts);
        let user_prompt = UserPrompt::new(self.pack_prompts);

        let alt = AltText::from(&user_prompt);
        let lora = match model.tensor_path.clone() {
            Some(p) => vec![Lora {
                path: p,
                scale: 1.0,
            }],
            None => vec![],
        };
        (0..self.amount)
            .map(|_| {
                let uuid = Uuid::new_v4();
                let s3_key = AwsS3::init_img_s3_key(&user_pid, &uuid);
                ImageNew {
                    pid: uuid,
                    user_id: model.user_id,
                    training_model_id: model.id,
                    pack_id: None,
                    sys_prompt: sys_prompt.to_owned(),
                    user_prompt: user_prompt.to_owned(),
                    alt: alt.to_owned(),
                    num_inference_steps: 50,
                    content_type: ImageFormat::Jpeg,
                    status: Status::Pending,
                    image_size: image_size.clone(),
                    fal_ai_request_id: None,
                    width: None,
                    height: None,
                    image_url_fal: None,
                    image_s3_key: s3_key,
                    is_favorite: false,
                    deleted_at: None,
                    loras: lora.clone(),
                }
            })
            .collect::<Vec<ImageNew>>()
            .into()
    }

    pub async fn find_by_pid(db: &DatabaseConnection, pid: &Uuid) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(plans::Column::Pid, pid.clone())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_id(db: &DatabaseConnection, id: &i32) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(plans::Column::Id, id.clone())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_all_packs(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let packs = Entity::find().all(db).await?;
        Ok(packs)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
