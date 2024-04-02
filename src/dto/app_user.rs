use entity::app_user;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AppUserDto {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl From<app_user::Model> for AppUserDto {
    fn from(model: app_user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
        }
    }
}
