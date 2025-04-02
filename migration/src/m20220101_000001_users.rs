use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto_tz(Users::Table)
            .col(pk_auto(Users::Id))
            .col(uuid_uniq(Users::Pid))
            .col(string_uniq(Users::Email))
            .col(string(Users::Password))
            .col(string_uniq(Users::ApiKey))
            .col(string(Users::Name))
            .col(string_null(Users::ResetToken))
            .col(timestamp_with_time_zone_null(Users::ResetSentAt))
            .col(string_null(Users::EmailVerificationToken))
            .col(timestamp_with_time_zone_null(
                Users::EmailVerificationSentAt,
            ))
            .col(timestamp_with_time_zone_null(Users::EmailVerifiedAt))
            .col(string_null(Users::MagicLinkToken))
            .col(timestamp_with_time_zone_null(Users::MagicLinkExpiration))
            .to_owned();
        manager.create_table(table).await?;

        // --- Index Creation (FIXED) ---
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-users-pid")
                    .table(Users::Table)
                    .col(Users::Pid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-users-email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await?;
        // --- Foreign Key Creation (FIXED) ---

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx-users-pid")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx-users-email")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Pid,
    Email,
    Name,
    Password,
    ApiKey,
    ResetToken,
    ResetSentAt,
    EmailVerificationToken,
    EmailVerificationSentAt,
    EmailVerifiedAt,
    MagicLinkToken,
    MagicLinkExpiration,
}
