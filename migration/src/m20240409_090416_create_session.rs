use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::UserId).integer().not_null())
                    .col(ColumnDef::new(Session::Uuid).string().not_null())
                    .col(
                        ColumnDef::new(Session::EndSessionDate)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    Uuid,
    EndSessionDate,
}
