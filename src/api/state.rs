use axum::extract::FromRef;
use sea_orm::DbConn;

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}

// support converting an `AppState` in an `ApiState`
impl FromRef<AppState> for DbConn {
    fn from_ref(app_state: &AppState) -> DbConn {
        app_state.db.clone()
    }
}
