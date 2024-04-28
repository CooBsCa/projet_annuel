use entity::zone;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use sea_orm::{ColumnTrait, TryIntoModel};
use sea_orm::{DbConn, DbErr, Set};

use crate::dto::zone::ZoneDto;

pub async fn create_zone(db: &DbConn, data: ZoneDto) -> Result<zone::Model, DbErr> {
    let data = zone::ActiveModel {
        name: Set(data.name),
        club_id: Set(data.club_id),
        open_at: Set(data.open_at),
        close_at: Set(data.close_at),
        reservation_time: Set(data.reservation_time),
        ..Default::default()
    };
    data.save(db).await?.try_into_model()
}

pub async fn get_zones_by_club(db: &DbConn, club_id: i32) -> Result<Vec<zone::Model>, DbErr> {
    zone::Entity::find()
        .filter(zone::Column::ClubId.eq(club_id))
        .all(db)
        .await
}

pub async fn delete_zone(db: &DbConn, zone_id: i32) -> Result<(), DbErr> {
    zone::Entity::delete_by_id(zone_id).exec(db).await?;
    Ok(())
}
