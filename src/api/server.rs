use axum::{response::IntoResponse, routing::get, Router};

/// Start axum http server
pub async fn start_server() {
    let app = get_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on port 3001");
    axum::serve(listener, app).await.unwrap();
}

fn get_router() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> impl IntoResponse {
    println!("Hello, World!");
    "Hello, World!"
}
