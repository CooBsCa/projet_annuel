use axum::{extract::State, Json};
use sea_orm::DbConn;

use crate::{
    dto::app_user::{AppUserDto, CreateUserDto},
    services::users_services,
};

#[utoipa::path(
        post,
        path = "/register",
        responses(
            (status = OK, description = "Register user", body = AppUserDto),
        ),
        tag = "Auth",
    )]
/// Get all users
pub async fn register_user(
    State(db): State<DbConn>,
    Json(create_user_dto): Json<CreateUserDto>,
) -> Json<AppUserDto> {
    Json(
        users_services::create_user(&db, create_user_dto)
            .await
            .unwrap()
            .into(),
    )
}
