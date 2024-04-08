use super::m20240408_075723_create_club::Club;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Zone::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Zone::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Zone::Name).string().not_null())
                    .col(
                        ColumnDef::new(Zone::OpenAt)
                            .time()
                            .not_null()
                            .comment("Hour of opening"),
                    )
                    .col(
                        ColumnDef::new(Zone::CloseAt)
                            .time()
                            .not_null()
                            .comment("Hour of closing"),
                    )
                    .col(
                        ColumnDef::new(Zone::ReservationTime)
                            .integer()
                            .not_null()
                            .comment("Duration of a reservation in minutes"),
                    )
                    .col(ColumnDef::new(Zone::ClubId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Zone::Table, Zone::ClubId)
                            .to(Club::Table, Club::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Zone::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Zone {
    Table,
    Id,
    Name,
    OpenAt,
    CloseAt,
    ReservationTime,
    ClubId,
}
