use chrono::NaiveDateTime;
use entity::slot;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, FromQueryResult)]
pub struct SlotDto {
    pub id: i32,
    pub user_id: i32,
    pub zone_id: i32,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub opponent_user_id: i32,
    pub user_name: String,
    pub opponent_user_name: String,
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

impl From<slot::Model> for CreateSlotDto {
    fn from(value: slot::Model) -> Self {
        CreateSlotDto {
            zone_id: value.zone_id,
            start_at: value.start_at,
            end_at: value.end_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClaimSlotDto {
    pub zone_id: i32,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub opponent_user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RequestSlotsOfDayDto {
    pub start_of_day: NaiveDateTime,
    pub end_of_day: NaiveDateTime,
}
