use std::sync::Arc;

use axum::{extract::Query, Extension, Json};

use crate::{
    dtos::{export::ExportFilter, record::RecordDTO},
    error::DomainError,
    services::export_service::ExportService,
};

pub(crate) struct ExportController;

impl ExportController {
    pub async fn export(
        Extension(export_service): Extension<Arc<ExportService>>,
        Query(export_filter): Query<ExportFilter>,
    ) -> Result<Json<Vec<RecordDTO>>, DomainError> {
        export_service.export(export_filter).await.map(Json)
    }
}
