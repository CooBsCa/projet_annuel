use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::app_user;
use sea_orm::{DbConn, EntityTrait};

pub async fn get_users(State(db): State<DbConn>) -> impl IntoResponse {
    println!("Get users");
    (
        StatusCode::OK,
        Json(app_user::Entity::find().into_json().all(&db).await.unwrap()),
    )
}
