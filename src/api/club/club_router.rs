use axum::{
    routing::{get, post},
    Router,
};

use crate::api::state::AppState;

use super::club_handler;

pub fn get_clubs_router() -> Router<AppState> {
    Router::new()
        .route("/clubs", get(club_handler::get_clubs))
        .route("/club", post(club_handler::create_club))
}
