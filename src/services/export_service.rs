use crate::{
    dtos::{export::ExportFilter, record::RecordDTO},
    error::DomainError,
    repositories::record_repository::RecordRepository,
};

pub(crate) struct ExportService {
    record_repository: RecordRepository,
}

impl ExportService {
    pub async fn try_new(url: &str) -> Result<Self, DomainError> {
        let record_repository = RecordRepository::try_new(url).await?;
        Ok(Self { record_repository })
    }

    pub async fn export(&self, export_filter: ExportFilter) -> Result<Vec<RecordDTO>, DomainError> {
        let dtos = self
            .record_repository
            .get_with_params(export_filter.start_date, export_filter.end_date)
            .await?
            .into_iter()
            .map(From::from)
            .collect();

        Ok(dtos)
    }
}
