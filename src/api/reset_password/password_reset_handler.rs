use axum::{extract::State, http::StatusCode, Json};
use sea_orm::DbConn;

use crate::{
    api::api_error::ApiError, dto::password_reset::PasswordResetDto,
    services::password_reset_services,
};

#[utoipa::path(
        post,
        path = "/password_reset",
        responses(
            (status = OK, description = "Email sender", body = PasswordResetDto),
        ),
        tag = "reset_password",
    )]
/// Register user
pub async fn send_email(
    State(db): State<DbConn>,
    Json(send_email_dto): Json<PasswordResetDto>,
) -> Result<StatusCode, ApiError> {
    let _ = password_reset_services::send_email(send_email_dto.email)
        .await
        .map_err(|_| ApiError::Internal);
    Ok(StatusCode::CREATED)
}
