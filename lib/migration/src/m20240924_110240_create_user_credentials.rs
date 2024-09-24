use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240924_105951_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(UserCredentials::Table)
                    .col(text(UserCredentials::Id).primary_key())
                    .col(uuid(UserCredentials::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_credentials_user_id")
                            .from(UserCredentials::Table, UserCredentials::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(binary(UserCredentials::PublicKey))
                    .col(binary(UserCredentials::PublicKeyAlgorithm))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserCredentials::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserCredentials {
    Table,
    Id,
    UserId,
    PublicKey,
    PublicKeyAlgorithm,
}
