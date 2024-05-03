use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, serde::Deserialize, ToSchema)]
pub struct PasswordResetDto {
    pub email: String,
}
