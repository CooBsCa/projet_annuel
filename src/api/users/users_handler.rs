use crate::{dto::app_user::AppUserDto, services::users_services};
use axum::{extract::State, Extension, Json};
use sea_orm::DbConn;

#[utoipa::path(
        get,
        path = "/users",
        responses(
            (status = OK, description = "Get all users", body = Vec<AppUserDto>),
        ),
        tag = "Users",
    )]
/// Get all users
pub async fn get_users(State(db): State<DbConn>) -> Json<Vec<AppUserDto>> {
    Json(
        users_services::get_users(&db)
            .await
            .unwrap()
            .into_iter()
            .map(AppUserDto::from)
            .collect(),
    )
}

#[utoipa::path(
        get,
        path = "/user",
        responses(
            (status = OK, description = "Get current user", body = AppUserDto),
        ),
        tag = "Users",
    )]
/// Get current user
pub async fn get_current_user(Extension(user): Extension<AppUserDto>) -> Json<AppUserDto> {
    Json(user)
}
