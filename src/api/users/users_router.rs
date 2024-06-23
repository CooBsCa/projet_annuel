use axum::{
    routing::{get, put},
    Router,
};

use crate::api::state::AppState;

use super::users_handler;

pub fn get_users_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(users_handler::get_users))
        .route("/user", get(users_handler::get_current_user))
        .route("/user/:id", get(users_handler::get_user_by_id))
        .route("/user-email", put(users_handler::update_user_email))
    // .route("/users/:id", get(users_handler::get_user))
    // .route("/users/:id", put(users_handler::update_user))
    // .route("/users/:id", delete(users_handler::delete_user))
}
