use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

// pid:uuid_col! users:references name:string! age:tiny_int! sex:string! ethnicity:string! type_model:string!
// eye_color:string! bald:bool! steps:int! create_mask:bool! is_style:bool! trigger_word:string!
// tensor_path:string thumbnail:string training_status:string! fal_output:json! output_images:json fal_ai_request_id:string s3_key:string! is_verified:bool!

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto_tz(TrainingModels::Table)
            .col(pk_auto(TrainingModels::Id))
            .col(uuid(TrainingModels::Pid).unique_key())
            .col(uuid(TrainingModels::UserId).unique_key())
            .col(string(TrainingModels::Name))
            .col(string(TrainingModels::Age))
            .col(string(TrainingModels::Sex).enumeration(
                Alias::new("Sex"),
                [Alias::new("Male"), Alias::new("Female")],
            ))
            .col(string(TrainingModels::Ethnicity).enumeration(
                Alias::new("Ethnicity"),
                [
                    Alias::new("White"),
                    Alias::new("Black"),
                    Alias::new("Pacific"),
                    Alias::new("Hispanic"),
                    Alias::new("Asian"),
                    Alias::new("SouthEastAsian"),
                    Alias::new("SouthAsian"),
                    Alias::new("MiddleEastern"),
                ],
            ))
            .col(string(TrainingModels::TypeModel).enumeration(
                Alias::new("TypeModel"),
                [
                    Alias::new("BasedRealPerson"),
                    Alias::new("CreateInfluencerAI"),
                ],
            ))
            .col(string_null(TrainingModels::EyeColor).enumeration(
                Alias::new("EyeColor"),
                [
                    Alias::new("Brown"),
                    Alias::new("Blue"),
                    Alias::new("Green"),
                    Alias::new("Grey"),
                    Alias::new("Hazel"),
                    Alias::new("Red"),
                ],
            ))
            .col(boolean(TrainingModels::Bald))
            .col(integer(TrainingModels::Steps))
            .col(boolean(TrainingModels::CreateMask))
            .col(boolean(TrainingModels::IsStyle))
            .col(string(TrainingModels::TriggerWord))
            .col(string_null(TrainingModels::TensorPath))
            .col(string_null(TrainingModels::Thumbnail))
            .col(string(TrainingModels::TrainingStatus))
            .col(json_null(TrainingModels::FalOutput))
            .col(json_null(TrainingModels::TrainingImages))
            .col(string_null(TrainingModels::FalAiRequestId))
            .col(string(TrainingModels::S3Key))
            .col(boolean(TrainingModels::IsVerified))
            .to_owned();
        manager.create_table(table).await?;

        // --- Index Creation (FIXED) ---
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-training_models-pid")
                    .table(TrainingModels::Table)
                    .col(TrainingModels::Pid)
                    .to_owned(),
            )
            .await?;

        // --- Foreign Key Creation (FIXED) ---
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-training_models-pid_to_users-id")
                    .from_tbl(TrainingModels::Table)
                    .from_col(TrainingModels::Pid)
                    .to_tbl(Users::Table)
                    .to_col(Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-training_models-pid_to_users-id")
                    .table(TrainingModels::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx-training_models-pid")
                    .table(TrainingModels::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(TrainingModels::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TrainingModels {
    Table,
    Id,
    Pid,
    UserId,
    Name,
    Age,
    Sex,
    Ethnicity,
    TypeModel,
    EyeColor,
    Bald,
    Steps,
    CreateMask,
    IsStyle,
    TriggerWord,
    TensorPath,
    Thumbnail,
    TrainingStatus,
    FalOutput,
    TrainingImages,
    FalAiRequestId,
    S3Key,
    IsVerified,
}
