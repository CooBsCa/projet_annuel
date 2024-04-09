use crate::dto::slot::CreateSlotDto;
use crate::dto::slot::QuerySlotDto;
use crate::utils::date_utils::get_days_between_dates;
use crate::utils::date_utils::split_day_into_slots;
use entity::slot;
use entity::zone;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::TryIntoModel;
use sea_orm::{DbConn, DbErr};

pub async fn get_available_slots(
    db: &DbConn,
    data: QuerySlotDto,
) -> Result<Vec<slot::Model>, DbErr> {
    slot::Entity::find()
        .filter(slot::Column::ZoneId.eq(data.zone_id))
        .filter(slot::Column::StartAt.gt(data.start_at))
        .filter(slot::Column::EndAt.lt(data.end_at))
        .all(db)
        .await
}

pub async fn claim_slot(db: &DbConn, slot_id: i32, user_id: i32) -> Result<slot::Model, DbErr> {
    let slot = slot::Entity::find_by_id(slot_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound)
        .map_err(|_| DbErr::RecordNotFound("Slot not found".to_string()))?;

    let mut active_model = slot::ActiveModel::from(slot);
    active_model.user_id = Set(Some(user_id));
    active_model.save(db).await?.try_into_model()
}

pub async fn create_slots(db: &DbConn, data: CreateSlotDto) -> Result<Vec<slot::Model>, DbErr> {
    let zone = zone::Entity::find_by_id(data.zone_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound)
        .map_err(|_| DbErr::RecordNotFound("Zone not found".to_string()))?;

    let mut slots = vec![];
    for day in get_days_between_dates(data.start_at.date(), data.end_at.date()) {
        let start = day.and_time(zone.open_at);
        let end = day.and_time(zone.close_at);
        for (start_at, end_at) in split_day_into_slots(start, end, zone.reservation_time) {
            let slot = create_slot(
                db,
                CreateSlotDto {
                    zone_id: data.zone_id,
                    start_at,
                    end_at,
                },
            )
            .await?;
            slots.push(slot);
        }
    }
    Ok(slots)
}

/// Create a slot in the database
async fn create_slot(db: &DbConn, data: CreateSlotDto) -> Result<slot::Model, DbErr> {
    let slot = slot::ActiveModel {
        user_id: Set(None),
        zone_id: Set(data.zone_id),
        start_at: Set(data.start_at),
        end_at: Set(data.end_at),
        ..Default::default()
    };
    slot.save(db).await?.try_into_model()
}

pub async fn delete_slots(db: &DbConn, data: QuerySlotDto) -> Result<(), DbErr> {
    let slots = get_available_slots(db, data).await?;
    for slot in slots {
        slot.delete(db).await?;
    }
    Ok(())
}
