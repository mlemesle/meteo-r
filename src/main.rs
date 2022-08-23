use axum::routing::get;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{Extension, Router};
use clap::{Arg, Command};
use serde_json::json;
use std::net::SocketAddr;

use crate::controllers::{export::ExportController, record::RecordController};
use crate::error::DomainError;
use crate::services::{export_service::ExportService, record_service::RecordService};

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

    // Retrieving user args
    let matches = Command::new("meteo-r")
        .about("A web server to interact with meteorological datas and export them to graph")
        .version("0.0.1")
        .author("mlemesle")
        .arg(
            Arg::new("db-url")
                .long("db-url")
                .short('d')
                .required(true)
                .takes_value(true)
                .help("URL to the postgres database"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .takes_value(true)
                .value_parser(clap::value_parser!(u16).range(3000..))
                .default_value("4444")
                .help("Port listened by the server"),
        )
        .get_matches();

    let db_url = matches
        .get_one::<String>("db-url")
        .expect("mandatory field");
    let record_service = RecordService::try_new(&db_url).await?;
    let export_service = ExportService::try_new(&db_url).await?;

    let app = Router::new()
        .route(
            "/records",
            get(RecordController::get_all).post(RecordController::insert_all),
        )
        .route(
            "/records/:id",
            get(RecordController::get_by_id).delete(RecordController::delete_by_id),
        )
        .route("/export", get(ExportController::export))
        .layer(Extension(record_service))
        .layer(Extension(export_service));

    let port = *matches.get_one("port").expect("defaulted by clap");
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

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
            DomainError::QueryError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Error while querying")
            }
            DomainError::UuidError(_err) => (StatusCode::INTERNAL_SERVER_ERROR, "Error UUID"),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
