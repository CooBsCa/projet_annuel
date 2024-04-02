use entity::app_user::{self};
use sea_orm::{DbConn, EntityTrait};

pub async fn get_users(db: &DbConn) -> Result<Vec<app_user::Model>, anyhow::Error> {
    app_user::Entity::find().all(db).await.map_err(|e| e.into())
}
