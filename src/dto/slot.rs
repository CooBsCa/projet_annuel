use chrono::NaiveDateTime;
use entity::slot;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SlotDto {
    pub id: i32,
    pub user_id: Option<i32>,
    pub zone_id: i32,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

impl From<slot::Model> for SlotDto {
    fn from(value: slot::Model) -> Self {
        SlotDto {
            id: value.id,
            user_id: value.user_id,
            zone_id: value.zone_id,
            start_at: value.start_at,
            end_at: value.end_at,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QuerySlotDto {
    pub zone_id: i32,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateSlotDto {
    pub zone_id: i32,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ClaimSlotDto {
    pub user_id: i32,
    pub slot_id: i32,
}
