use chrono::NaiveTime;
use entity::zone;
use serde::{Deserialize, Serialize};

use crate::utils::time_format;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneDto {
    pub name: String,
    pub club_id: i32,
    #[serde(with = "time_format")]
    pub open_at: NaiveTime,
    #[serde(with = "time_format")]
    pub close_at: NaiveTime,
    pub reservation_time: i32,
}

impl From<zone::Model> for ZoneDto {
    fn from(model: zone::Model) -> Self {
        Self {
            name: model.name,
            club_id: model.club_id,
            open_at: model.open_at,
            close_at: model.close_at,
            reservation_time: model.reservation_time,
        }
    }
}
