use crate::{
    dtos::record::RecordDTO, error::DomainError,
    repositories::postgres_repository::PostgresRepository,
};

pub(crate) struct PostgresService {
    postgres_repository: PostgresRepository,
}

impl PostgresService {
    pub async fn try_new(url: &str) -> Result<Self, DomainError> {
        let postgres_repository = PostgresRepository::try_new(url).await?;
        Ok(Self {
            postgres_repository,
        })
    }

    pub async fn get_all(&self) -> Result<Vec<RecordDTO>, DomainError> {
        let dtos = self
            .postgres_repository
            .get_all()
            .await?
            .into_iter()
            .map(From::from)
            .collect::<Vec<_>>();
        Ok(dtos)
    }

    pub async fn get_by_id(&self, id: sqlx::types::Uuid) -> Option<RecordDTO> {
        todo!()
    }

    pub async fn insert_all(&self, datas: Vec<RecordDTO>) {
        todo!()
    }
}
