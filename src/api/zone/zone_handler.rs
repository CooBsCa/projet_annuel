use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::DbConn;

use crate::{api::api_error::ApiError, dto::zone::ZoneDto, services::zone_services};

#[utoipa::path(
        post,
        path = "/zone",
        responses(
            (status = OK, description = "Created zone", body = ZoneDto),
        ),
        tag = "Zone",
    )]
pub async fn create_zone(
    State(db): State<DbConn>,
    Json(data): Json<ZoneDto>,
) -> Result<Json<ZoneDto>, ApiError> {
    zone_services::create_zone(&db, data)
        .await
        .map(|zone| Json(zone.into()))
        .map_err(|_| ApiError::Internal)
}

#[utoipa::path(
        get,
        path = "/zones/{club_id}",
        responses(
            (status = OK, description = "Get all zones by club", body = Vec<ZoneDto>),
        ),
        tag = "Zone",
    )]
pub async fn get_zones_by_club(
    State(db): State<DbConn>,
    Path(club_id): Path<i32>,
) -> Result<Json<Vec<ZoneDto>>, ApiError> {
    zone_services::get_zones_by_club(&db, club_id)
        .await
        .map(|zones| Json(zones.into_iter().map(|z| z.into()).collect()))
        .map_err(|_| ApiError::Internal)
}
