use entity::club;
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{DbConn, EntityTrait, Set};

use crate::dto::club::CreateClubDto;

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
