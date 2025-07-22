use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250718_121104_create_utilisateur_table::Utilisateur, m20250719_092057_create_room_faritany::FaritanyRoom};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FaritanyPoint::Table)
                    .if_not_exists()
                    .col(pk_auto(FaritanyPoint::Id))
                    .col(ColumnDef::new(FaritanyPoint::UtilisateurId).integer().not_null())
                    .col(ColumnDef::new(FaritanyPoint::RoomId).integer().not_null())
                    .col(ColumnDef::new(FaritanyPoint::Coord).string().not_null())
                    .col(
                        ColumnDef::new(FaritanyPoint::DateCreation)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(ForeignKey::create().name("fk-point-utiisateur-id")
                        .from(FaritanyPoint::Table, FaritanyPoint::UtilisateurId)
                        .to(Utilisateur::Table, Utilisateur::Id)
                        .on_delete(ForeignKeyAction::SetNull)
                    )
                    .foreign_key(ForeignKey::create().name("fk-point-room-id")
                        .from(FaritanyPoint::Table, FaritanyPoint::RoomId)
                        .to(FaritanyRoom::Table, FaritanyRoom::Id)
                        .on_delete(ForeignKeyAction::SetNull)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(FaritanyPoint::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FaritanyPoint {
    Table,
    Id,
    UtilisateurId,
    RoomId,
    Coord,
    DateCreation
}
