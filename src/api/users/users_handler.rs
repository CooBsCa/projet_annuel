use crate::{
    api::api_error::ApiError, dto::app_user::AppUserDto, dto::app_user::UpdateEmailDto,
    services::users_services,
};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
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
        path = "/user/{id}",
        responses(
            (status = OK, description = "Get current user"),
        ),
        tag = "Users",
    )]
/// Get use by id
pub async fn get_user_by_id(State(db): State<DbConn>, Path(id): Path<i32>) -> Json<AppUserDto> {
    Json(
        users_services::get_user_by_id(&db, id)
            .await
            .unwrap()
            .into(),
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

#[utoipa::path(
    put,
    path = "/user-email",
    responses(
        (status = OK, description = "Updated user", body = UpdateEmailDto),
    ),
    tag = "User",
)]
/// Update a user
pub async fn update_user_email(
    State(db): State<DbConn>,
    Json(data): Json<UpdateEmailDto>,
) -> Result<Json<AppUserDto>, ApiError> {
    let user: Result<entity::app_user::Model, ApiError> =
        crate::services::users_services::update_user_email(&db, data)
            .await
            .map_err(|_| ApiError::Internal);
    Ok(Json(user?.into()))
}
