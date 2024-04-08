use axum::{routing::post, Router};

use crate::api::state::AppState;

use super::auth_handler;

pub fn get_auth_router() -> Router<AppState> {
    Router::new().route("/register", post(auth_handler::register_user))
    // .route("/users/:id", get(users_handler::get_user))
    // .route("/users/:id", put(users_handler::update_user))
    // .route("/users/:id", delete(users_handler::delete_user))
}
