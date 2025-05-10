use crate::service::{aws::s3::AwsS3, fal_ai::fal_client::Lora};

pub use super::_entities::packs::{ActiveModel, Entity, Model};
use super::{
    PackModel, TrainingModelModel,
    _entities::{plans, sea_orm_active_enums::ImageSize},
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

#[derive(Debug, Clone)]
pub struct PacksDomain {
    pub id: i32,
    pub pid: Uuid,
    pub title: String,
    pub pack_prompts: String,
    pub short_description: String,
    pub full_description: String,
    pub credits: i32,
    pub num_images: i32,
    pub num_inference_steps: i32,
    pub image_url: String,
    pub image_size: ImageSize,
}
impl PacksDomain {
    pub fn from_model(packs: PackModel, image_size: ImageSize) -> Self {
        Self {
            id: packs.id,
            pid: packs.pid,
            title: packs.title,
            pack_prompts: packs.pack_prompts,
            short_description: packs.short_description,
            full_description: packs.full_description,
            credits: packs.credits,
            num_images: packs.num_images,
            num_inference_steps: packs.num_inference_steps,
            image_url: packs.image_url,
            image_size,
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
        (0..self.num_images)
            .map(|_| {
                let uuid = Uuid::new_v4();
                let s3_key = AwsS3::init_img_s3_key(&user_pid, &uuid);
                ImageNew {
                    pid: uuid,
                    image_s3_key: s3_key,
                    user_id: model.user_id,
                    training_model_id: model.id,
                    pack_id: Some(self.id),
                    sys_prompt: sys_prompt.to_owned(),
                    user_prompt: user_prompt.to_owned(),
                    alt: alt.to_owned(),
                    num_inference_steps: 50,
                    loras: lora.clone(),
                    image_size: image_size.clone(),
                    ..Default::default()
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
