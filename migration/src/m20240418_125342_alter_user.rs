use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AppUser::Table)
                    .add_column(ColumnDef::new(AppUser::IdClub).integer().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AppUser {
    Table,
    IdClub,
}
