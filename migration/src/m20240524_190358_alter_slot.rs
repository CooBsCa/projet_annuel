use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::AppUser;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Slot::Table)
                    .modify_column(ColumnDef::new(Slot::UserId).integer().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Slot {
    Table,
    UserId,
}
