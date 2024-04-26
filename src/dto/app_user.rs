use entity::app_user;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct AppUserDto {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub id_club: i32,
}

impl From<app_user::Model> for AppUserDto {
    fn from(model: app_user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            id_club: model.id_club,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub id_club: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginUserDto {
    pub user_login_input: String,
    pub password: String,
}
