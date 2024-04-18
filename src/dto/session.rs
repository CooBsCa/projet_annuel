use chrono::NaiveDateTime;
use entity::session;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionDto {
    pub user_id: i32,
    pub uuid: String,
    pub end_session_date: NaiveDateTime,
}

impl From<session::Model> for SessionDto {
    fn from(model: session::Model) -> Self {
        Self {
            user_id: model.user_id,
            uuid: model.uuid,
            end_session_date: model.end_session_date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUuidDto {
    pub uuid: String,
}

impl From<String> for SessionUuidDto {
    fn from(value: String) -> Self {
        Self { uuid: value }
    }
}
