use axum::{extract::State, Json};
use sea_orm::DbConn;

use crate::{
    api::api_error::ApiError,
    dto::slot::{ClaimSlotDto, CreateSlotDto, QuerySlotDto, SlotDto},
    services::slot_services,
};

#[utoipa::path(
        post,
        path = "/get-available-slots",
        responses(
            (status = OK, description = "Get available slot", body = Vec<SlotDto>),
        ),
        tag = "Slot",
    )]
pub async fn get_available_slots(
    State(db): State<DbConn>,
    Json(data): Json<QuerySlotDto>,
) -> Result<Json<Vec<SlotDto>>, ApiError> {
    Ok(Json(
        slot_services::get_available_slots(&db, data)
            .await
            .map(|slots| slots.into_iter().map(|slot| slot.into()).collect())
            .map_err(|_| ApiError::Internal)?,
    ))
}

#[utoipa::path(
        post,
        path = "/claim-slot",
        responses(
            (status = OK, description = "Claimed slot", body = SlotDto),
        ),
        tag = "Slot",
    )]
pub async fn claim_slot(
    State(db): State<DbConn>,
    Json(data): Json<ClaimSlotDto>,
) -> Result<Json<SlotDto>, ApiError> {
    slot_services::claim_slot(&db, data.slot_id, data.user_id)
        .await
        .map(|slot| slot.into())
        .map(Json)
        .map_err(|_| ApiError::NotFound)
}

#[utoipa::path(
        post,
        path = "/slots",
        responses(
            (status = OK, description = "Created slot", body = Vec<SlotDto>),
        ),
        tag = "Slot",
    )]
pub async fn create_slots(
    State(db): State<DbConn>,
    Json(data): Json<CreateSlotDto>,
) -> Result<Json<Vec<SlotDto>>, ApiError> {
    Ok(Json(
        slot_services::create_slots(&db, data)
            .await
            .map(|slots| slots.into_iter().map(|slot| slot.into()).collect())
            .map_err(|_| ApiError::Internal)?,
    ))
}

#[utoipa::path(post, path = "/delete-slots", tag = "Slot")]
pub async fn delete_slots(
    State(db): State<DbConn>,
    Json(data): Json<QuerySlotDto>,
) -> Result<(), ApiError> {
    slot_services::delete_slots(&db, data)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(())
}
