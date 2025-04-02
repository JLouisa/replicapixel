use crate::m20220101_000001_users::Users;
use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

// pid:uuid_col! users:references name:string! age:tiny_int! sex:string! ethnicity:string! type_model:string!
// eye_color:string! bald:bool! steps:int! create_mask:bool! is_style:bool! trigger_word:string!
// tensor_path:string thumbnail:string training_status:string! fal_output:json! output_images:json fal_ai_request_id:string s3_key:string! is_verified:bool!

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("Status")) // Ensure alias matches SQL enum type
                    .values([
                        Alias::new("Pending"),
                        Alias::new("Training"),
                        Alias::new("Completed"),
                        Alias::new("Failed"),
                        Alias::new("Cancelled"),
                    ])
                    .to_owned(),
            )
            .await
            .unwrap(); // Ensure this completes before table creation

        let table = table_auto_tz(TrainingModels::Table)
            .col(pk_auto(TrainingModels::Id))
            .col(uuid(TrainingModels::Pid).unique_key())
            .col(uuid(TrainingModels::UserId).unique_key())
            .col(string(TrainingModels::Name))
            .col(string(TrainingModels::Age))
            .col(string(TrainingModels::Sex))
            .col(string(TrainingModels::Ethnicity))
            .col(string(TrainingModels::BasedOn))
            .col(string_null(TrainingModels::EyeColor))
            .col(boolean(TrainingModels::Bald))
            .col(integer(TrainingModels::Steps))
            .col(boolean(TrainingModels::CreateMask))
            .col(boolean(TrainingModels::IsStyle))
            .col(string(TrainingModels::TriggerWord))
            .col(string_null(TrainingModels::TensorPath))
            .col(string_null(TrainingModels::Thumbnail))
            .col(
                ColumnDef::new(TrainingModels::TrainingStatus)
                    .custom(Alias::new("Status")) // Use the correct alias
                    .not_null(),
            )
            .col(json_binary_null(TrainingModels::FalOutput))
            .col(json_binary_null(TrainingModels::TrainingImages))
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
                    .from_col(TrainingModels::UserId)
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
    BasedOn,
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

// #[derive(DeriveIden)]
// #[sea_orm(enum_name = "Status")]
// pub enum Status {
//     #[sea_orm(iden = "Status")]
//     Enum,
//     #[sea_orm(iden = "Pending")]
//     Pending,
//     #[sea_orm(iden = "Training")]
//     Training,
//     #[sea_orm(iden = "Completed")]
//     Completed,
//     #[sea_orm(iden = "Failed")]
//     Failed,
//     #[sea_orm(iden = "Cancelled")]
//     Cancelled,
// }

// --- Create ENUM Types using Manual String Lists ---
// NOTE: These lists MUST be kept in sync manually with your
//       Rust enums defined in pictora/src/models/_entities/active_enums.rs

// manager
//     .create_type(
//         Type::create()
//             .as_enum(Alias::new("Sex"))
//             .values([
//                 Alias::new("Male"),   // Match #[sea_orm(string_value = "Male")]
//                 Alias::new("Female"), // Match #[sea_orm(string_value = "Female")]
//             ])
//             .to_owned(),
//     )
//     .await?;

// manager
//     .create_type(
//         Type::create()
//             .as_enum(Alias::new("Ethnicity"))
//             .values([
//                 Alias::new("White"),
//                 Alias::new("Black"),
//                 Alias::new("Pacific"),
//                 Alias::new("Hispanic"),
//                 Alias::new("Asian"),
//                 Alias::new("SouthEastAsian"),
//                 Alias::new("SouthAsian"),
//                 Alias::new("MiddleEastern"),
//             ])
//             .to_owned(), // Ensure these match your Ethnicity enum string_values
//     )
//     .await?;

// manager
//     .create_type(
//         Type::create()
//             .as_enum(Alias::new("BasedOn"))
//             .values([Alias::new("RealPerson"), Alias::new("CreateInfluencerAI")])
//             .to_owned(), // Ensure these match your BasedOn enum string_values
//     )
//     .await?;

// manager
//     .create_type(
//         Type::create()
//             .as_enum(Alias::new("EyeColor"))
//             .values([
//                 Alias::new("Brown"),
//                 Alias::new("Blue"),
//                 Alias::new("Green"),
//                 Alias::new("Grey"),
//                 Alias::new("Hazel"),
//                 Alias::new("Red"),
//             ])
//             .to_owned(), // Ensure these match your EyeColor enum string_values
//     )
//     .await?;
