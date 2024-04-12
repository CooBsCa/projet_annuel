use axum::{
    extract::Request, extract::State, http::StatusCode, middleware::Next, response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Local;
use entity::{app_user, session};
use sea_orm::DbConn;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, Condition};

use crate::dto::app_user::AppUserDto;

pub async fn auth(
    State(db): State<DbConn>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match auth_header {
        Some(TypedHeader(header)) => {
            if let Some(usr) = get_user_from_token(db, header.token()).await {
                request.extensions_mut().insert(usr);
                let response = next.run(request).await;
                Ok(response)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn get_user_from_token(db: DbConn, token: &str) -> Option<AppUserDto> {
    let session = session::Entity::find()
        .filter(
            Condition::all()
                .add(session::Column::Uuid.eq(token))
                .add(session::Column::EndSessionDate.gt(Local::now().naive_local())),
        )
        .one(&db)
        .await
        .ok()??;

    app_user::Entity::find_by_id(session.user_id)
        .one(&db)
        .await
        .ok()?
        .map(AppUserDto::from)
}
