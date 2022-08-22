use std::sync::Arc;

use axum::{extract::Path, Extension, Json};

use crate::{
    dtos::record::RecordDTO, dtos::record::RecordDTOWithoutID, error::DomainError,
    services::record_service::RecordService,
};

use super::OneOrMany;

pub(crate) struct RecordController;

impl RecordController {
    pub async fn get_all(
        Extension(postgres_service): Extension<Arc<RecordService>>,
    ) -> Result<Json<Vec<RecordDTO>>, DomainError> {
        postgres_service.get_all().await.map(Json)
    }

    pub async fn insert_all(
        Extension(postgres_service): Extension<Arc<RecordService>>,
        Json(dtos): Json<OneOrMany<RecordDTOWithoutID>>,
    ) -> Result<Json<u64>, DomainError> {
        postgres_service
            .insert_all(dtos.to_values())
            .await
            .map(Json)
    }

    pub async fn get_by_id(
        Extension(postgres_service): Extension<Arc<RecordService>>,
        Path(id): Path<String>,
    ) -> Result<Json<Option<RecordDTO>>, DomainError> {
        postgres_service.get_by_id(&id).await.map(Json)
    }

    pub async fn delete_by_id(
        Extension(postgres_service): Extension<Arc<RecordService>>,
        Path(id): Path<String>,
    ) -> Result<Json<bool>, DomainError> {
        postgres_service.delete_by_id(&id).await.map(Json)
    }
}
