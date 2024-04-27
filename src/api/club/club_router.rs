use axum::{
    routing::{get, post, put},
    Router,
};

use crate::api::state::AppState;

use super::club_handler;

pub fn get_clubs_router() -> Router<AppState> {
    Router::new()
        .route("/clubs", get(club_handler::get_clubs))
        .route("/club", post(club_handler::create_club))
        .route("/club", get(club_handler::get_club))
        .route("/club", put(club_handler::update_club))
}
