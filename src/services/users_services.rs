extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};
use entity::app_user::{self};
use entity::club::{self};
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::DbErr;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{DbConn, EntityTrait, Set};

use crate::dto::app_user::{CreateUserDto, LoginUserDto};
use crate::dto::session::SessionUuidDto;
use crate::services::session_services;

pub async fn get_users(db: &DbConn) -> Result<Vec<app_user::Model>, anyhow::Error> {
    app_user::Entity::find().all(db).await.map_err(|e| e.into())
}

pub async fn create_user(
    db: &DbConn,
    create_user_dto: CreateUserDto,
) -> Result<app_user::Model, anyhow::Error> {
    let password_hash = hash(create_user_dto.password, DEFAULT_COST)?;
    let searched_club = club::Entity::find_by_id(create_user_dto.id_club)
        .one(db)
        .await?;
    let _ = searched_club.ok_or(DbErr::RecordNotFound(
        "Aucun club conrrespondant a cette id".to_string(),
    ))?;

    let usr = app_user::ActiveModel {
        username: Set(create_user_dto.username),
        email: Set(create_user_dto.email),
        password: Set(password_hash),
        id_club: Set(create_user_dto.id_club),
        ..Default::default()
    };
    let active = usr.save(db).await?;
    let model: app_user::Model = active.try_into_model()?;
    Ok(model)
}

pub async fn search_user(
    db: &DbConn,
    login_user_dto: LoginUserDto,
) -> Result<SessionUuidDto, DbErr> {
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
            if let Ok(session) = session_services::create_session(db, user.id).await {
                Ok(SessionUuidDto {
                    uuid: session.uuid,
                    is_admin: user.is_admin,
                    username: user.username,
                })
            } else {
                Err(DbErr::RecordNotInserted)
            }
        }
        Ok(false) => Err(DbErr::RecordNotFound(login_user_dto.password)),
        Err(_) => Err(DbErr::RecordNotFound(login_user_dto.password)),
    }
}
