extern crate bcrypt;

use bcrypt::{hash, DEFAULT_COST};
use entity::app_user::{self};
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{DbConn, EntityTrait, Set};



use crate::dto::app_user::CreateUserDto;

pub async fn get_users(db: &DbConn) -> Result<Vec<app_user::Model>, anyhow::Error> {
    app_user::Entity::find().all(db).await.map_err(|e| e.into())
}

pub async fn create_user(
    db: &DbConn,
    create_user_dto: CreateUserDto,
) -> Result<app_user::Model, anyhow::Error> {
    let hashed = hash(create_user_dto.password, DEFAULT_COST).unwrap();
    let usr = app_user::ActiveModel {
        username: Set(create_user_dto.username),
        email: Set(create_user_dto.email),
        password: Set(hashed),
        ..Default::default()
    };
    let active = usr.save(db).await?;
    let model: app_user::Model = active.try_into_model()?;
    Ok(model)
}
