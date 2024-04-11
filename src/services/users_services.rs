extern crate bcrypt;

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use entity::app_user::{self};
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::DbErr;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{DbConn, EntityTrait, Set};

use crate::dto::app_user::{CreateUserDto, LoginUserDto};
use crate::services::session_services;

pub async fn get_users(db: &DbConn) -> Result<Vec<app_user::Model>, anyhow::Error> {
    app_user::Entity::find().all(db).await.map_err(|e| e.into())
}

pub async fn create_user(
    db: &DbConn,
    create_user_dto: CreateUserDto,
) -> Result<app_user::Model, anyhow::Error> {
    let password_hash = hash_password(create_user_dto.password)?;
    let usr = app_user::ActiveModel {
        username: Set(create_user_dto.username),
        email: Set(create_user_dto.email),
        password: Set(password_hash),
        ..Default::default()
    };
    let active = usr.save(db).await?;
    let model: app_user::Model = active.try_into_model()?;
    Ok(model)
}

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub async fn seach_user(
    db: &DbConn,
    login_user_dto: LoginUserDto,
) -> Result<app_user::Model, DbErr> {
    let searched_user = app_user::Entity::find()
        .filter(
            Condition::any()
                .add(app_user::Column::Username.eq(&login_user_dto.user_login_input))
                .add(app_user::Column::Email.eq(&login_user_dto.user_login_input)),
        )
        .one(db)
        .await?;
    let user = searched_user.ok_or(DbErr::RecordNotFound(
        "Aucun Utilisateur trouvé".to_string(),
    ))?;
    match verify(&login_user_dto.password, &user.password) {
        Ok(true) => {
            if let Ok(_) = session_services::create_session(&db, user.id).await {
                Ok(user)
            } else {
                Err(DbErr::RecordNotInserted)
            }
        }
        Ok(false) => Err(DbErr::RecordNotFound(login_user_dto.password)),
        Err(_) => Err(DbErr::RecordNotFound(login_user_dto.password)),
    }
}
