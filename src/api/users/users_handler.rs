use axum::response::IntoResponse;

pub async fn get_users() -> impl IntoResponse {
    println!("Get users");
    "Get users"
}
