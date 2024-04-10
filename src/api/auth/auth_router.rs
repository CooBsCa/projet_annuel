use axum::{routing::post, Router};

use crate::api::state::AppState;

use super::auth_handler;

pub fn get_auth_router() -> Router<AppState> {
    let mut router = Router::new();
    router = router.route("/register", post(auth_handler::register_user));
    router = router.route("/login", post(auth_handler::login_user));
    router
    // .route("/users/:id", get(users_handler::get_user))
    // .route("/users/:id", put(users_handler::update_user))
    // .route("/users/:id", delete(users_handler::delete_user))
}
