use axum::{http::Method, Router};
use sea_orm::DbConn;
use tower_http::cors::{Any, CorsLayer};
use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

use crate::api::state::AppState;

use super::{
    auth::auth_router::get_auth_router, club::club_router::get_clubs_router,
    slot::slot_router::get_slot_router, users::users_router::get_users_router,
    zone::zone_router::get_zone_router,
};

/// Start axum http server
pub async fn start_server(db: DbConn) {
    let state = AppState { db };
    let app = get_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on port 3001");
    println!("You can access the API documentation at http://localhost:3001/swagger-ui");
    axum::serve(listener, app).await.unwrap();
}

fn get_router() -> Router<AppState> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .merge(get_users_router())
        .merge(get_auth_router())
        .merge(get_clubs_router())
        .merge(get_zone_router())
        .merge(get_slot_router())
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDocs::openapi()))
}

/// Swagger OpenApi documentation of the API
#[utoipauto]
#[derive(OpenApi)]
#[openapi(info(title = "Open API", version = "1.0.0"), modifiers(&BearerAuth))]
pub struct ApiDocs;

pub struct BearerAuth;

impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "BrearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
