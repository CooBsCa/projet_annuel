use crate::services::users_services;
use axum::{extract::State, Json};
use entity::app_user;
use sea_orm::DbConn;

pub async fn get_users(State(db): State<DbConn>) -> Json<Vec<app_user::Model>> {
    Json(users_services::get_users(&db).await.unwrap())
}
