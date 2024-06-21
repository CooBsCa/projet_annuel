use crate::dto::slot::ClaimSlotDto;
use crate::dto::slot::CreateSlotDto;
use crate::dto::slot::QuerySlotDto;
use crate::dto::slot::RequestSlotsOfDayDto;
use crate::utils::date_utils::get_days_between_dates;
use crate::utils::date_utils::split_day_into_slots;
use crate::utils::periode_utils::is_period_overlap;
use crate::utils::periode_utils::Period;
use entity::slot;
use entity::zone;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::TryIntoModel;
use sea_orm::{DbConn, DbErr};

pub async fn get_available_slots(
    db: &DbConn,
    data: QuerySlotDto,
) -> Result<Vec<CreateSlotDto>, DbErr> {
    let claimed_slot = slot::Entity::find()
        .filter(slot::Column::ZoneId.eq(data.zone_id))
        .filter(slot::Column::StartAt.gt(data.start_at))
        .filter(slot::Column::EndAt.lt(data.end_at))
        .all(db)
        .await?;

    let base_slots = compute_slots(db, data)
        .await?
        .into_iter()
        .filter(|slot| is_slot_available(&claimed_slot, slot))
        .collect::<Vec<CreateSlotDto>>();

    // let
    Ok(base_slots)
}

pub fn is_slot_available(claimed_slots: &[slot::Model], slot: &CreateSlotDto) -> bool {
    for claimed in claimed_slots {
        if is_period_overlap(
            &Period::new(claimed.start_at, claimed.end_at),
            &Period::new(slot.start_at, slot.end_at),
        ) {
            return false;
        }
    }
    true
}

pub async fn get_claimed_slots(db: &DbConn, user_id: i32) -> Result<Vec<slot::Model>, DbErr> {
    slot::Entity::find()
        .filter(
            Condition::any()
                .add(slot::Column::UserId.eq(user_id))
                .add(slot::Column::OpponentUserId.eq(user_id)),
        )
        .all(db)
        .await
}

pub async fn get_all_claimed_slots_by_day(
    db: &DbConn,
    data: RequestSlotsOfDayDto,
) -> Result<Vec<slot::Model>, DbErr> {
    // Query to get all slots for a day
    print!("on passe ici");
    slot::Entity::find()
        .filter(slot::Column::StartAt.gte(data.start_of_day))
        .filter(slot::Column::StartAt.lte(data.end_of_day))
        .all(db)
        .await
}

pub async fn get_future_claimed_slots(
    db: &DbConn,
    user_id: i32,
) -> Result<Vec<slot::Model>, DbErr> {
    slot::Entity::find()
        .filter(
            Condition::any()
                .add(slot::Column::UserId.eq(user_id))
                .add(slot::Column::OpponentUserId.eq(user_id)),
        )
        .filter(slot::Column::StartAt.gt(chrono::Utc::now().naive_utc()))
        .all(db)
        .await
}

pub async fn count_past_claimed_slots(db: &DbConn, user_id: i32) -> Result<u64, DbErr> {
    slot::Entity::find()
        .filter(
            Condition::any()
                .add(slot::Column::UserId.eq(user_id))
                .add(slot::Column::OpponentUserId.eq(user_id)),
        )
        .filter(slot::Column::StartAt.lt(chrono::Utc::now().naive_utc()))
        .count(db)
        .await
}

pub async fn claim_slot(db: &DbConn, slot_id: i32, user_id: i32) -> Result<slot::Model, DbErr> {
    let slot = slot::Entity::find_by_id(slot_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound)
        .map_err(|_| DbErr::RecordNotFound("Slot not found".to_string()))?;

    let mut active_model = slot::ActiveModel::from(slot);
    active_model.user_id = Set(user_id);
    active_model.save(db).await?.try_into_model()
}

pub async fn compute_slots(db: &DbConn, data: QuerySlotDto) -> Result<Vec<CreateSlotDto>, DbErr> {
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
            slots.push(CreateSlotDto {
                zone_id: data.zone_id,
                start_at,
                end_at,
            })
        }
    }

    Ok(slots)
}

/// Create a slot in the database
pub async fn create_slot(
    db: &DbConn,
    data: ClaimSlotDto,
    user_id: i32,
) -> Result<slot::Model, DbErr> {
    let slot = slot::ActiveModel {
        user_id: Set(user_id),
        zone_id: Set(data.zone_id),
        start_at: Set(data.start_at),
        end_at: Set(data.end_at),
        opponent_user_id: Set(data.opponent_user_id),
        ..Default::default()
    };
    slot.save(db).await?.try_into_model()
}

pub async fn cancel_slot(db: &DbConn, slot_id: i32) -> Result<(), DbErr> {
    slot::Entity::delete_by_id(slot_id).exec(db).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn build_test_slot(start: &str, end: &str) -> slot::Model {
        slot::Model {
            id: 1,
            user_id: 1,
            zone_id: 1,
            start_at: chrono::NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S").unwrap(),
            end_at: chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").unwrap(),
            opponent_user_id: 2,
        }
    }

    fn build_test_slots(slots: Vec<(&str, &str)>) -> Vec<slot::Model> {
        slots
            .iter()
            .map(|(start, end)| build_test_slot(start, end))
            .collect()
    }

    #[test]
    fn test_is_slot_available() {
        let claimed_slots = build_test_slots(vec![
            ("2021-01-01 10:00:00", "2021-01-01 12:00:00"),
            ("2021-01-01 14:00:00", "2021-01-01 16:00:00"),
        ]);

        let slot = build_test_slot("2021-01-01 13:00:00", "2021-01-01 15:00:00");
        assert!(!is_slot_available(&claimed_slots, &slot.into()));

        let slot = build_test_slot("2021-01-01 10:00:00", "2021-01-01 12:00:00");
        assert!(!is_slot_available(&claimed_slots, &slot.into()));

        let slot = build_test_slot("2021-01-01 12:00:00", "2021-01-01 14:00:00");
        assert!(is_slot_available(&claimed_slots, &slot.into()));
    }
}
