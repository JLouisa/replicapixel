use std::collections::HashMap;

use super::_entities::plans;
pub use super::_entities::plans::{ActiveModel, Entity, Model};
use derive_more::{AsRef, Constructor};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::Condition;
use serde::Serialize;
pub type Plans = Entity;
use crate::models::_entities::plans_translations as plans_translations_entity;
use crate::models::_entities::sea_orm_active_enums::PlanCategory;
use crate::models::{
    PlanTranslationModel,
    _entities::sea_orm_active_enums::{Language, PlanNames},
    plans_translations::PlanTranslationModelList,
};

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

#[derive(Debug, Constructor, Clone, AsRef)]
pub struct PlanModelList(pub Vec<Model>);

#[derive(Debug, Constructor, Serialize, Clone)]
pub struct PlanDomain {
    pub id: i32,
    pub pid: Uuid,
    pub plan_name: PlanNames,
    pub credit_amount: i32,
    pub model_amount: i32,
    pub credit_amount_plus: Option<i32>,
    pub model_amount_plus: Option<i32>,
    pub price_cents: i64,
    pub is_popular: bool,
    pub name: String,
    pub subtitle: String,
    pub features: Option<Vec<String>>,
    pub cta: String,
}
impl From<Model> for PlanDomain {
    fn from(plan: Model) -> Self {
        Self {
            id: plan.id,
            pid: plan.pid,
            plan_name: plan.plan_name,
            credit_amount: plan.credit_amount,
            model_amount: plan.model_amount,
            credit_amount_plus: plan.credit_amount_plus,
            model_amount_plus: plan.model_amount_plus,
            price_cents: plan.price_cents,
            is_popular: plan.is_popular,
            name: plan.name,
            subtitle: plan.subtitle,
            features: plan.features,
            cta: plan.cta,
        }
    }
}
impl PlanDomain {
    pub fn from_model(plan: Model) -> Self {
        Self {
            id: plan.id,
            pid: plan.pid,
            plan_name: plan.plan_name,
            credit_amount: plan.credit_amount,
            model_amount: plan.model_amount,
            credit_amount_plus: plan.credit_amount_plus,
            model_amount_plus: plan.model_amount_plus,
            price_cents: plan.price_cents,
            is_popular: plan.is_popular,
            name: plan.name,
            subtitle: plan.subtitle,
            features: plan.features,
            cta: plan.cta,
        }
    }
    pub fn translate(plan: Model, translation: PlanTranslationModel) -> Self {
        Self {
            id: plan.id,
            pid: plan.pid,
            plan_name: plan.plan_name,
            credit_amount: plan.credit_amount,
            model_amount: plan.model_amount,
            credit_amount_plus: plan.credit_amount_plus,
            model_amount_plus: plan.model_amount_plus,
            price_cents: plan.price_cents,
            is_popular: plan.is_popular,
            name: plan.name,
            subtitle: translation.subtitle,
            features: translation.features,
            cta: translation.cta,
        }
    }
}

#[derive(Debug, Constructor, Serialize, Clone)]
pub struct PlanDomainList(pub Vec<PlanDomain>);
impl From<PlanModelList> for PlanDomainList {
    fn from(plans: PlanModelList) -> Self {
        Self(
            plans
                .0
                .iter()
                .map(|p| PlanDomain::from_model(p.clone()))
                .collect(),
        )
    }
}

impl PlanDomainList {
    pub fn translate(
        plans: PlanModelList,
        translations: PlanTranslationModelList,
        lang: &Language,
    ) -> Self {
        let language = lang.clone();
        if language == Language::English {
            return plans.into();
        }
        let mut translation_map = HashMap::new();
        for translation in translations.0 {
            if translation.language == language {
                translation_map.insert(translation.plan_id, translation);
            }
        }

        let mut list = Vec::with_capacity(plans.as_ref().len());
        for plan in plans.0 {
            if let Some(translation) = translation_map.get(&plan.id) {
                list.push(PlanDomain::translate(plan, (*translation).clone()));
            }
        }

        Self(list)
    }
    pub fn from_related(
        packs_with_translations: Vec<(Model, Vec<PlanTranslationModel>)>,
        lang: &Language,
    ) -> Self {
        let pack_list = packs_with_translations
            .into_iter()
            .map(|(pack, translations)| {
                if *lang == Language::English {
                    return PlanDomain::from_model(pack);
                }
                let target_translation = translations
                    .iter()
                    .find(|translation| translation.language == *lang);

                match target_translation {
                    Some(translation) => PlanDomain::translate(pack, translation.clone()),
                    None => PlanDomain::from_model(pack),
                }
            })
            .collect();

        Self(pack_list)
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn load_plan_and_all_translated(
        db: &DatabaseConnection,
        lang: &Language,
    ) -> ModelResult<PlanDomainList> {
        let plans_with_translations = Entity::find()
            .filter(
                model::query::condition()
                    .eq(plans::Column::Category, PlanCategory::Main)
                    .build(),
            )
            .find_with_related(plans_translations_entity::Entity)
            .all(db)
            .await?;
        Ok(PlanDomainList::from_related(plans_with_translations, lang))
    }
    pub async fn find_all_translated(
        db: &DatabaseConnection,
        lang: &Language,
    ) -> ModelResult<PlanDomainList> {
        if *lang == Language::English {
            return Ok(Model::find_all_main(db).await?.into());
        }
        Ok(Model::load_plan_and_all_translated(db, lang).await?.into())
    }
    pub async fn find_by_pid(db: &impl ConnectionTrait, pid: &Uuid) -> ModelResult<Self> {
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
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(model::query::condition().eq(plans::Column::Id, id).build())
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_name(db: &DatabaseConnection, name: &PlanNames) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(plans::Column::Name, name.to_owned())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_by_name_string(
        db: &impl ConnectionTrait,
        name: &String,
    ) -> ModelResult<Self> {
        let user = Entity::find()
            .filter(
                model::query::condition()
                    .eq(plans::Column::Name, name.to_owned())
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    pub async fn find_all(db: &impl ConnectionTrait) -> ModelResult<PlanModelList> {
        let plans = Entity::find().all(db).await?;
        let plans = PlanModelList(plans);
        Ok(plans)
    }
    pub async fn find_all_main(db: &impl ConnectionTrait) -> ModelResult<PlanModelList> {
        let condition = Condition::all().add(plans::Column::Category.eq(PlanCategory::Main));
        let plans = Entity::find().filter(condition).all(db).await?;
        let plans = PlanModelList(plans);
        Ok(plans)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
