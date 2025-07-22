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
                    .table(FaritanyRoomUtilisateur::Table)
                    .if_not_exists()
                    .col(pk_auto(FaritanyRoomUtilisateur::Id))
                    .col(ColumnDef::new(FaritanyRoomUtilisateur::UtilisateurId).integer().not_null())
                    .col(ColumnDef::new(FaritanyRoomUtilisateur::RoomId).integer().not_null())
                    .col(ColumnDef::new(FaritanyRoomUtilisateur::EstSpectateur).boolean().not_null().default(false))
                    .foreign_key(ForeignKey::create().name("fk-room-utiisateur-id")
                        .from(FaritanyRoomUtilisateur::Table, FaritanyRoomUtilisateur::UtilisateurId)
                        .to(Utilisateur::Table, Utilisateur::Id)
                        .on_delete(ForeignKeyAction::SetNull)
                    )
                    .foreign_key(ForeignKey::create().name("fk-room-id")
                        .from(FaritanyRoomUtilisateur::Table, FaritanyRoomUtilisateur::RoomId)
                        .to(FaritanyRoom::Table, FaritanyRoom::Id)
                        .on_delete(ForeignKeyAction::SetNull)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(FaritanyRoomUtilisateur::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FaritanyRoomUtilisateur {
    Table,
    Id,
    UtilisateurId,
    RoomId,
    EstSpectateur
}
