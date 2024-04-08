use entity::club;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, serde::Deserialize, ToSchema)]
pub struct ClubDto {
    pub id: i32,
    pub name: String,
}

impl From<club::Model> for ClubDto {
    fn from(model: club::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}

#[derive(Serialize, serde::Deserialize, ToSchema)]
pub struct CreateClubDto {
    pub name: String,
}
