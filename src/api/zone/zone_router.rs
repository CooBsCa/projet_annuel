use axum::{
    routing::{get, post},
    Router,
};

use crate::api::state::AppState;

use super::zone_handler::{create_zone, get_zones_by_club};

pub fn get_zone_router() -> Router<AppState> {
    Router::new()
        .route("/zone", post(create_zone))
        .route("/zones/:club_id", get(get_zones_by_club))
}
