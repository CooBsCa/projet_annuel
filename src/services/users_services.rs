extern crate bcrypt;
extern crate rand;

use bcrypt::{hash, verify, DEFAULT_COST};
use entity::app_user::{self};
use entity::club::{self};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::DbErr;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{DbConn, EntityTrait, Set};

use crate::dto::app_user::{
    AppUserDto, CreateUserDto, LoginUserDto, UpdateEmailDto, UpdatePasswordDto,
};
use crate::dto::session::SessionUuidDto;
use crate::services::session_services;

pub async fn get_users(db: &DbConn) -> Result<Vec<app_user::Model>, anyhow::Error> {
    app_user::Entity::find().all(db).await.map_err(|e| e.into())
}

pub async fn get_user_by_id(db: &DbConn, id: i32) -> Result<app_user::Model, anyhow::Error> {
    app_user::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(anyhow::Error::msg("User not found"))?
        .try_into_model()
        .map_err(|e| e.into())
}

pub async fn create_user(
    db: &DbConn,
    create_user_dto: CreateUserDto,
) -> Result<SessionUuidDto, anyhow::Error> {
    let user_exists = app_user::Entity::find()
        .filter(
            Condition::any()
                .add(app_user::Column::Username.eq(&create_user_dto.username))
                .add(app_user::Column::Email.eq(&create_user_dto.email)),
        )
        .one(db)
        .await?;
    if user_exists.is_some() {
        return Err(
            DbErr::Custom("Un utilisateur avec ce nom ou email existe déjà".to_string()).into(),
        );
    }
    let password_hash = hash(create_user_dto.password, DEFAULT_COST)?;
    let searched_club = club::Entity::find_by_id(create_user_dto.id_club)
        .one(db)
        .await?;
    searched_club.ok_or(DbErr::RecordNotFound(
        "Aucun club conrrespondant a cette id".to_string(),
    ))?;

    let usr = app_user::ActiveModel {
        username: Set(create_user_dto.username),
        email: Set(create_user_dto.email),
        password: Set(password_hash),
        id_club: Set(create_user_dto.id_club),
        ..Default::default()
    };
    let active: app_user::ActiveModel = usr.save(db).await?;
    let model: app_user::Model = active.try_into_model()?;
    // Ok(model)
    let session = session_services::create_session(db, model.id).await?;
    Ok(SessionUuidDto {
        uuid: session.uuid,
        is_admin: model.is_admin,
        username: model.username,
    })
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

pub async fn user_exists_by_email(db: &DbConn, email: &str) -> Result<app_user::Model, DbErr> {
    let user = app_user::Entity::find()
        .filter(app_user::Column::Email.eq(email))
        .one(db)
        .await?;

    user.ok_or(DbErr::RecordNotFound("Aucun utilisateur trouvé".to_string()).into())
}

pub async fn reset_password(db: &DbConn, user: app_user::Model) -> Result<String, anyhow::Error> {
    let random_password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let password_hash = hash(random_password.clone(), DEFAULT_COST)?;
    let user = app_user::Entity::find_by_id(user.id).one(db).await?;
    let mut user: app_user::ActiveModel = user.ok_or(anyhow::Error::msg("User not found"))?.into();
    user.password = Set(password_hash.clone());
    user.update(db).await?;
    Ok(random_password)
}

pub async fn update_user_email(
    db: &DbConn,
    data: UpdateEmailDto,
) -> Result<app_user::Model, anyhow::Error> {
    let user = app_user::Entity::find_by_id(data.id).one(db).await?;
    let mut user: app_user::ActiveModel = user.ok_or(anyhow::Error::msg("User not found"))?.into();
    user.email = Set(data.email);
    let active = user.update(db).await?;
    Ok(active.try_into_model()?)
}

pub async fn update_user_password(
    db: &DbConn,
    data: UpdatePasswordDto,
) -> Result<app_user::Model, anyhow::Error> {
    let search_user = app_user::Entity::find_by_id(data.id).one(db).await?;
    let user = search_user.ok_or(DbErr::RecordNotFound(
        "Aucun Utilisateur trouvé".to_string(),
    ))?;

    if !verify(&data.password.clone(), &user.password)? {
        return Err(anyhow::Error::msg("Mot de passe incorrect"));
    }

    let new_hash_password = hash(data.new_password.clone(), DEFAULT_COST)?;
    let mut user: app_user::ActiveModel = user.into();
    user.password = Set(new_hash_password);
    let active = user.update(db).await?;
    Ok(active.try_into_model()?)
}
