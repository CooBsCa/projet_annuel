use axum::{
    routing::{get, post},
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
    // .route("/slots/{zone_id}", get(get_slots_by_zone))
}
