use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(FaritanyRoom::Table)
            .if_not_exists()
            .col(pk_auto(FaritanyRoom::Id))
            .col(
                ColumnDef::new(FaritanyRoom::Uuid)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(FaritanyRoom::Password).string())
            .col(
                ColumnDef::new(FaritanyRoom::Taille)
                    .string()
                    .not_null()
                    .default("[20,30]"),
            )
            .col(
                ColumnDef::new(FaritanyRoom::Reflexion)
                    .integer()
                    .not_null()
                    .default(30),
            )
            .col(
                ColumnDef::new(FaritanyRoom::DateCreation)
                    .timestamp()
                    .not_null()
                    .default(Expr::cust("CURRENT_TIMESTAMP")),
            )
            .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(FaritanyRoom::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FaritanyRoom {
    Table,
    Id,
    Uuid,
    Password,
    Taille,
    Reflexion,
    DateCreation
}