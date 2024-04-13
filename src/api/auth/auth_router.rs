use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::api::state::AppState;

use super::auth_handler;

pub fn get_auth_router() -> Router<AppState> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let mut router = Router::new();
    router = router.route("/register", post(auth_handler::register_user));
    router = router.route("/login", post(auth_handler::login_user));
    router.layer(cors)
    // .route("/users/:id", get(users_handler::get_user))
    // .route("/users/:id", put(users_handler::update_user))
    // .route("/users/:id", delete(users_handler::delete_user))
}
