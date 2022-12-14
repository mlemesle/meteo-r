use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPoolOptions, types::Uuid, PgPool, QueryBuilder};

use crate::{entities::record::RecordEntity, error::DomainError};

#[derive(Clone)]
pub(crate) struct RecordRepository {
    pool: PgPool,
}

impl RecordRepository {
    pub async fn try_new(url: &str) -> Result<Self, DomainError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn get_all(&self) -> Result<Vec<RecordEntity>, DomainError> {
        sqlx::query_as(
            r#"
            SELECT * FROM records;
        "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|sql_err| {
            tracing::error!("{}", sql_err);
            DomainError::QueryError(sql_err)
        })
    }

    pub async fn insert_all(&self, datas: Vec<RecordEntity>) -> Result<u64, DomainError> {
        let datas_len = datas.len();
        if datas_len < 1 {
            return Ok(0);
        }

        QueryBuilder::new("INSERT INTO records (temperature, pressure, humidity, date) ")
            .push_values(datas.iter(), |mut b, data| {
                b.push_bind(data.temperature)
                    .push_bind(data.pressure)
                    .push_bind(data.humidity)
                    .push_bind(data.date);
            })
            .build()
            .execute(&self.pool)
            .await
            .map(|pg_result| pg_result.rows_affected())
            .map_err(|sql_err| {
                tracing::error!("{}", sql_err);
                DomainError::QueryError(sql_err)
            })
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<RecordEntity>, DomainError> {
        sqlx::query_as(
            r#"
                SELECT * FROM records WHERE id = $1;
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|sql_err| {
            tracing::error!("{}", sql_err);
            DomainError::QueryError(sql_err)
        })
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        sqlx::query(
            r#"
            DELETE FROM records WHERE id = $1;
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map(|pg_result| pg_result.rows_affected() == 1)
        .map_err(|sql_err| {
            tracing::error!("{}", sql_err);
            DomainError::QueryError(sql_err)
        })
    }

    pub async fn get_with_params(
        &self,
        start_date_opt: Option<DateTime<Utc>>,
        end_date_opt: Option<DateTime<Utc>>,
    ) -> Result<Vec<RecordEntity>, DomainError> {
        let mut qb = QueryBuilder::new("SELECT * FROM records WHERE 1 = 1");
        if let Some(start_date) = start_date_opt {
            qb.push(" AND date >= ");
            qb.push_bind(start_date);
        }

        if let Some(end_date) = end_date_opt {
            qb.push(" AND date <= ");
            qb.push_bind(end_date);
        }

        qb.push(";");
        let query = qb.build_query_as();

        query.fetch_all(&self.pool).await.map_err(|sql_err| {
            tracing::error!("{}", sql_err);
            DomainError::QueryError(sql_err)
        })
    }
}
