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
                    .add_column(ColumnDef::new(Slot::OpponentUserId).integer().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .from_tbl(Slot::Table)
                            .from_col(Slot::OpponentUserId)
                            .to_tbl(AppUser::Table)
                            .to_col(AppUser::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Slot {
    Table,
    OpponentUserId,
}
