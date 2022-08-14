use sqlx::{postgres::PgPoolOptions, types::Uuid, PgPool};

use crate::{entities::record::RecordEntity, error::DomainError};

pub(crate) struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub async fn try_new(url: &str) -> Result<Self, DomainError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn get_all(&self) -> Result<Vec<RecordEntity>, DomainError> {
        let query_result = sqlx::query_as(
            r#"
            SELECT * FROM records;
        "#,
        )
        .fetch_all(&self.pool)
        .await;

        match query_result {
            Ok(result) => Ok(result),
            Err(sql_err) => {
                tracing::error!("{}", sql_err);
                Err(DomainError::QueryError(sql_err))
            }
        }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Option<RecordEntity> {
        todo!()
    }

    pub async fn insert(&self, data: RecordEntity) {
        todo!()
    }

    pub async fn insert_all(&self, datas: Vec<RecordEntity>) {
        todo!()
    }
}
