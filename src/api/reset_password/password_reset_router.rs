use super::password_reset_handler;
use crate::api::state::AppState;
use axum::{http::Method, routing::post, Router};
use http::header::AUTHORIZATION;
use tower_http::cors::{Any, CorsLayer};

pub fn get_reset_password_router() -> Router<AppState> {
    let router = Router::new();
    router.route("/password_reset", post(password_reset_handler::send_email))
}
