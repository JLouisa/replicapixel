use loco_rs::prelude::*;
use sea_orm::prelude::*;

use sea_orm::DatabaseConnection;
use sea_orm::{query::*, JoinType};

use super::user_credits_models::JoinError;
use crate::models::_entities::packs as pack_entity;
use crate::models::_entities::packs_translations as pack_translations_entity;
use crate::models::_entities::sea_orm_active_enums::Language;
use crate::models::packs::PackTranslated;
use crate::models::packs::PackTranslatedList;
use crate::models::PackEntity;
use crate::models::_entities::packs::Relation as PackRelation;

pub async fn load_pack_and_translation(
    db: &DatabaseConnection,
    title_url: &str,
    lang: &Language,
) -> Result<PackTranslated, JoinError> {
    let query_results = PackEntity::find()
        .filter(pack_entity::Column::TitleUrl.eq(title_url.to_owned()))
        .join(JoinType::InnerJoin, PackRelation::PacksTranslations.def())
        .filter(pack_translations_entity::Column::Language.eq(lang.to_owned()))
        .select_also(pack_translations_entity::Entity)
        .one(db)
        .await
        .map_err(JoinError::Database)?;

    let (pack, pack_translation) =
        query_results.ok_or_else(|| JoinError::PackNotFound(title_url.to_string()))?;
    let pack_translation = pack_translation
        .ok_or_else(|| JoinError::ModelNotFound("Translation not found".to_string()))?;

    let pack_translated = PackTranslated::translate(pack, pack_translation);
    Ok(pack_translated)
}

pub async fn load_pack_and_all_translated(
    db: &DatabaseConnection,
    lang: &Language,
) -> ModelResult<PackTranslatedList> {
    let packs_with_translations = PackEntity::find()
        .find_with_related(pack_translations_entity::Entity)
        .all(db)
        .await?;

    let packs_translated = PackTranslatedList::from_related(packs_with_translations, lang);

    Ok(packs_translated)
}
