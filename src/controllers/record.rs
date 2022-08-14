use std::sync::Arc;

use axum::{Extension, Json};

use crate::{
    dtos::record::RecordDTO, error::DomainError, services::postgres_service::PostgresService,
};

pub(crate) struct RecordController;

impl RecordController {
    pub async fn get_all(
        Extension(postgres_service): Extension<Arc<PostgresService>>,
    ) -> Result<Json<Vec<RecordDTO>>, DomainError> {
        let dtos = postgres_service.get_all().await?;

        Ok(Json(dtos))
    }
}
