use entity::club;
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{DbConn, EntityTrait, Set};

use crate::dto::club::{ClubDto, CreateClubDto};

pub async fn get_clubs(db: &DbConn) -> Result<Vec<club::Model>, anyhow::Error> {
    club::Entity::find().all(db).await.map_err(|e| e.into())
}

pub async fn get_club_id(db: &DbConn, id: i32) -> Result<club::Model, anyhow::Error> {
    club::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(anyhow::Error::msg("Club not found"))?
        .try_into_model()
        .map_err(|e| e.into())
}

pub async fn create_club(db: &DbConn, data: CreateClubDto) -> Result<club::Model, anyhow::Error> {
    let club = club::ActiveModel {
        name: Set(data.name),
        ..Default::default()
    };
    let active = club.save(db).await?;
    Ok(active.try_into_model()?)
}

pub async fn update_club(db: &DbConn, data: ClubDto) -> Result<club::Model, anyhow::Error> {
    let club = club::Entity::find_by_id(data.id).one(db).await?;
    let mut club: club::ActiveModel = club.ok_or(anyhow::Error::msg("Club not found"))?.into();
    club.name = Set(data.name);
    let active = club.update(db).await?;
    Ok(active.try_into_model()?)
}
