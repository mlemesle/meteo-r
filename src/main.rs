use axum::routing::get;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{Extension, Router};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::controllers::record::RecordController;
use crate::error::DomainError;
use crate::services::postgres_service::PostgresService;

mod controllers;
mod dtos;
mod entities;
mod error;
mod repositories;
mod services;

#[tokio::main]
async fn main() -> Result<(), DomainError> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let postgres_service =
        Arc::new(PostgresService::try_new("postgres://meteor:passw0rd@localhost/meteor").await?);

    let app = Router::new()
        .route("/records", get(RecordController::get_all))
        .layer(Extension(postgres_service));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

impl IntoResponse for DomainError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            DomainError::QueryError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Error while querying")
            }
            DomainError::UuidError(err) => (StatusCode::INTERNAL_SERVER_ERROR, "Error UUID"),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
