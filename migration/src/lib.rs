pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220101_000002_alter_table;
mod m20220101_000003_alter_table;
mod m20240408_075723_create_club;
mod m20240408_075736_create_zone;
mod m20240408_094016_create_slot;
mod m20240408_095814_alter_user;
mod m20240409_090416_create_session;
mod m20240418_125342_alter_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220101_000002_alter_table::Migration),
            Box::new(m20220101_000003_alter_table::Migration),
            Box::new(m20240408_075723_create_club::Migration),
            Box::new(m20240408_075736_create_zone::Migration),
            Box::new(m20240408_094016_create_slot::Migration),
            Box::new(m20240408_095814_alter_user::Migration),
            Box::new(m20240409_090416_create_session::Migration),
            Box::new(m20240418_125342_alter_user::Migration),
        ]
    }
}
