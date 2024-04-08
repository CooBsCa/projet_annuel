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
                    .add_column(ColumnDef::new(AppUser::Password).string().not_null())
                    .add_column(ColumnDef::new(AppUser::Password_Len).integer().not_null())
                    .add_column(ColumnDef::new(AppUser::Salt).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AppUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AppUser {
    Table,
    Password,
    #[allow(non_camel_case_types)]
    Password_Len,
    Salt,
}
