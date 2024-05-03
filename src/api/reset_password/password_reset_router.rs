use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::api::state::AppState;

use super::password_reset_handler;

pub fn get_reset_password_router() -> Router<AppState> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let mut router = Router::new();
    router = router.route("/password_reset", post(password_reset_handler::send_email));
    router.layer(cors)
}
