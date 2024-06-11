use super::password_reset_handler;
use crate::api::state::AppState;
use axum::{routing::post, Router};

pub fn get_reset_password_router() -> Router<AppState> {
    let router = Router::new();
    router.route("/password_reset", post(password_reset_handler::send_email))
}
