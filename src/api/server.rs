use axum::Router;
use sea_orm::DbConn;

use super::users::users_router::get_users_router;

/// Start axum http server
pub async fn start_server(db: DbConn) {
    let state = AppState { db };
    let app = get_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on port 3001");
    axum::serve(listener, app).await.unwrap();
}

fn get_router() -> Router<AppState> {
    Router::new().merge(get_users_router())
}

#[derive(Clone)]
pub struct AppState {
    db: DbConn,
}
