use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Utilisateur::Table)
                    .if_not_exists()
                    .col(pk_auto(Utilisateur::Id))
                    .col(string(Utilisateur::Pseudo).unique_key())
                    .col(string(Utilisateur::Email).unique_key())
                    .col(string(Utilisateur::Password))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Utilisateur::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Utilisateur {
    Table,
    Id,
    Pseudo,
    Email,
    Password
}
