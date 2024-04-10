extern crate bcrypt;

use chrono::{Duration, Local};
use entity::session::{self};
use sea_orm::DbErr;
use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{DbConn, Set};
use uuid::Uuid;

pub async fn create_session(db: &DbConn, user_id: i32) -> Result<session::Model, anyhow::Error> {
    let session = session::ActiveModel {
        user_id: Set(user_id),
        uuid: Set(Uuid::new_v4().to_string()),
        end_session_date: Set(Local::now()
            .naive_local()
            .checked_add_signed(Duration::hours(1))
            .ok_or(DbErr::RecordNotInserted)?),
        ..Default::default()
    };
    let active = session.save(db).await?;
    let model: session::Model = active.try_into_model()?;
    Ok(model)
}
