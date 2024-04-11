use axum::{
    extract::Request, extract::State, http::StatusCode, middleware::Next, response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Local;
use entity::session;
use sea_orm::DbConn;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, Condition};

pub async fn auth(
    State(db): State<DbConn>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match auth_header {
        Some(TypedHeader(header)) => {
            if token_is_valid(db, header.token()).await {
                let response = next.run(request).await;
                Ok(response)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn token_is_valid(db: DbConn, token: &str) -> bool {
    println!("{}", token);
    if let Some(session_found) = session::Entity::find()
        .filter(
            Condition::all()
                .add(session::Column::Uuid.eq(token))
                .add(session::Column::EndSessionDate.gt(Local::now().naive_local())),
        )
        .one(&db)
        .await
        .ok()
    {
        match session_found {
            Some(_) => true,
            None => false,
        }
    } else {
        false
    }
}
