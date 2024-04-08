use super::m20220101_000001_create_table::AppUser;
use super::m20240408_075736_create_zone::Zone;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Slot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Slot::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Slot::UserId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Slot::Table, Slot::UserId)
                            .to(AppUser::Table, AppUser::Id),
                    )
                    .col(ColumnDef::new(Slot::ZoneId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Slot::Table, Slot::ZoneId)
                            .to(Zone::Table, Zone::Id),
                    )
                    .col(ColumnDef::new(Slot::StartAt).date_time().not_null())
                    .col(ColumnDef::new(Slot::EndAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Slot::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Slot {
    Table,
    Id,
    UserId,
    ZoneId,
    StartAt,
    EndAt,
}
