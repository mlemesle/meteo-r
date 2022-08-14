use std::str::FromStr;

use sqlx::types::Uuid;

use crate::{
    dtos::record::RecordDTO, dtos::record::RecordDTOWithoutID, error::DomainError,
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

    pub async fn insert_all(&self, datas: Vec<RecordDTOWithoutID>) -> Result<u64, DomainError> {
        let entities = datas
            .into_iter()
            .map(TryFrom::try_from)
            // There we can allow ourselves to unwrap directly since the implementation can't fail
            .map(Result::unwrap)
            .collect();

        self.postgres_repository.insert_all(entities).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<RecordDTO>, DomainError> {
        let id = Uuid::from_str(id)?;

        self.postgres_repository
            .get_by_id(id)
            .await
            .map(|entity_opt| entity_opt.map(From::from))
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<bool, DomainError> {
        let id = Uuid::from_str(id)?;

        self.postgres_repository.delete_by_id(id).await
    }
}
