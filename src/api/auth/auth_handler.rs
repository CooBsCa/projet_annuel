use axum::{extract::State, Json};
use sea_orm::DbConn;

use crate::{
    dto::app_user::{AppUserDto, CreateUserDto, LoginUserDto},
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
/// Register user
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

#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = OK, description = "User login"),
    ),
    tag = "Auth",
)]
/// Log user
pub async fn login_user(
    State(db): State<DbConn>,
    Json(user_login): Json<LoginUserDto>,
) -> Json<AppUserDto> {
    Json(
        users_services::seach_user(&db, user_login)
            .await
            .unwrap()
            .into(),
    )
}
