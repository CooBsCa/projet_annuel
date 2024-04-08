use axum::{http::StatusCode, response::IntoResponse};
pub enum ApiError {
    NotFound,
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "something went wrong"),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something else went wrong",
            ),
        }
        .into_response()
    }
}
