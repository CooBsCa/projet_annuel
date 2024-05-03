use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sea_orm::DbConn;

use crate::{
    api::api_error::ApiError,
    dto::{
        app_user::AppUserDto,
        slot::{ClaimSlotDto, CreateSlotDto, QuerySlotDto, SlotDto},
    },
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
) -> Result<Json<Vec<CreateSlotDto>>, ApiError> {
    Ok(Json(
        slot_services::get_available_slots(&db, data)
            .await
            .map(|slots| slots.into_iter().collect())
            .map_err(|_| ApiError::Internal)?,
    ))
}

#[utoipa::path(
        get,
        path = "/claimed-slots",
        responses(
            (status = OK, description = "Get claimed slot for current usr", body = Vec<SlotDto>),
        ),
        tag = "Slot",
    )]
/// Get slot claimed by current user
pub async fn get_claimed_slots(
    State(db): State<DbConn>,
    Extension(usr): Extension<AppUserDto>,
) -> Result<Json<Vec<SlotDto>>, ApiError> {
    Ok(Json(
        slot_services::get_claimed_slots(&db, usr.id)
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
    Extension(usr): Extension<AppUserDto>,
    Json(data): Json<ClaimSlotDto>,
) -> Result<Json<SlotDto>, ApiError> {
    let slot = slot_services::create_slot(&db, data, usr.id)
        .await
        .map_err(|_| ApiError::Internal)?;

    slot_services::claim_slot(&db, slot.id, slot.user_id)
        .await
        .map(|slot| slot.into())
        .map(Json)
        .map_err(|_| ApiError::NotFound)
}

#[utoipa::path(
        get,
        path = "/future-claimed-slots",
        responses(
            (status = OK, description = "Get number of future claimed slots", body = i32),
        ),
        tag = "Slot",
    )]
/// Get number of future claimed slots
pub async fn get_future_claimed_slots(
    State(db): State<DbConn>,
    Extension(usr): Extension<AppUserDto>,
) -> Result<Json<usize>, ApiError> {
    Ok(Json(
        slot_services::get_future_claimed_slots(&db, usr.id)
            .await
            .map_err(|_| ApiError::Internal)?
            .len(),
    ))
}

#[utoipa::path(
        get,
        path = "/past-claimed-slots",
        responses(
            (status = OK, description = "Get number of past claimed slots", body = i32),
        ),
        tag = "Slot",
    )]
/// Get number of past claimed slots
pub async fn get_past_claimed_slots(
    State(db): State<DbConn>,
    Extension(usr): Extension<AppUserDto>,
) -> Result<Json<u64>, ApiError> {
    Ok(Json(
        slot_services::count_past_claimed_slots(&db, usr.id)
            .await
            .map_err(|_| ApiError::Internal)?,
    ))
}

pub async fn cancel_slot(State(db): State<DbConn>, Path(id): Path<i32>) -> Result<(), ApiError> {
    slot_services::cancel_slot(&db, id)
        .await
        .map_err(|_| ApiError::NotFound)
}
