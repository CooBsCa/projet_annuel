use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::api::state::AppState;

use super::slot_handler;

pub fn get_slot_router() -> Router<AppState> {
    Router::new()
        .route("/claimed-slots", get(slot_handler::get_claimed_slots))
        .route("/claim-slot", post(slot_handler::claim_slot))
        .route(
            "/get-available-slots",
            post(slot_handler::get_available_slots),
        )
        .route(
            "/future-claimed-slots",
            get(slot_handler::get_future_claimed_slots),
        )
        .route(
            "/past-claimed-slots",
            get(slot_handler::get_past_claimed_slots),
        )
        .route("/cancel-slot/:id", delete(slot_handler::cancel_slot))

    // .route("/slots/{zone_id}", get(get_slots_by_zone))
}
