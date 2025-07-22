pub use sea_orm_migration::prelude::*;

mod m20250718_121104_create_utilisateur_table;
mod m20250719_092057_create_room_faritany;
mod m20250719_093834_create_room_utilisateur_faritany;
mod m20250719_105608_create_point_faritany;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250718_121104_create_utilisateur_table::Migration),
            Box::new(m20250719_092057_create_room_faritany::Migration),
            Box::new(m20250719_093834_create_room_utilisateur_faritany::Migration),
            Box::new(m20250719_105608_create_point_faritany::Migration),
        ]
    }
}
