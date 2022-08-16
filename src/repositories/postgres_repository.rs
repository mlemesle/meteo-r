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

        let mut temperatures = Vec::with_capacity(datas_len);
        let mut pressures = Vec::with_capacity(datas_len);
        let mut humidities = Vec::with_capacity(datas_len);

        datas.iter().for_each(|data| {
            temperatures.push(data.temperature);
            pressures.push(data.pressure);
            humidities.push(data.humidity);
        });

        sqlx::query(
            r#"
            INSERT INTO records (temperature, pressure, humidity) 
            SELECT * FROM UNNEST ($1,$2,$3);
        "#,
        )
        .bind(temperatures)
        .bind(pressures)
        .bind(humidities)
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
}
