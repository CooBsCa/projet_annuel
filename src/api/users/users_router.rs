use axum::{routing::get, Router};

use crate::api::server::AppState;

use super::users_handler;

pub fn get_users_router() -> Router<AppState> {
    Router::new().route("/users", get(users_handler::get_users))
    // .route("/users", post(users_handler::create_user))
    // .route("/users/:id", get(users_handler::get_user))
    // .route("/users/:id", put(users_handler::update_user))
    // .route("/users/:id", delete(users_handler::delete_user))
}
