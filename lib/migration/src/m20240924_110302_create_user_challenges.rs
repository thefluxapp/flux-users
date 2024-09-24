use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(UserChallenges::Table)
                    .col(text(UserChallenges::Id).primary_key())
                    .col(uuid(UserChallenges::UserId))
                    .col(text_null(UserChallenges::UserName))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserChallenges::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserChallenges {
    Table,
    Id,
    UserId,
    UserName,
}
