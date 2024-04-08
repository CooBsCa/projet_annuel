use axum::{extract::State, Json};
use sea_orm::DbConn;

use crate::{
    api::api_error::ApiError,
    dto::club::{ClubDto, CreateClubDto},
};

#[utoipa::path(
        get,
        path = "/clubs",
        responses(
            (status = OK, description = "Get all clubs", body = Vec<ClubDto>),
        ),
        tag = "Club",
    )]
/// Get all clubs
pub async fn get_clubs(State(db): State<DbConn>) -> Result<Json<Vec<ClubDto>>, ApiError> {
    let clubs = crate::services::club_services::get_clubs(&db)
        .await
        .expect("Failed to get clubs");
    Ok(Json(clubs.into_iter().map(|c| c.into()).collect()))
}

#[utoipa::path(
        post,
        path = "/club",
        responses(
            (status = OK, description = "Created club", body = ClubDto),
        ),
        tag = "Club",
    )]
/// Create a club
pub async fn create_club(
    State(db): State<DbConn>,
    Json(data): Json<CreateClubDto>,
) -> Result<Json<ClubDto>, ApiError> {
    let club = crate::services::club_services::create_club(&db, data)
        .await
        .map_err(|_| ApiError::Internal);
    Ok(Json(club?.into()))
}
