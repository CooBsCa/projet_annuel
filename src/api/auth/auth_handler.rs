use axum::{extract::State, http::StatusCode, Json};
use entity::session;
use sea_orm::DbConn;

use crate::{
    api::api_error::ApiError,
    dto::{
        app_user::{CreateUserDto, LoginUserDto},
        session::SessionUuidDto,
    },
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
) -> Result<Json<SessionUuidDto>, ApiError> {
    let session_uuid = users_services::create_user(&db, create_user_dto)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(Json(session_uuid.into()))
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
) -> Result<Json<SessionUuidDto>, ApiError> {
    let session_uuid = users_services::search_user(&db, user_login)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(Json(session_uuid.into()))
}
