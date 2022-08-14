use std::str::FromStr;

use sqlx::types::Uuid;

use crate::{dtos::record::RecordDTO, error::DomainError};

#[derive(sqlx::FromRow)]
pub(crate) struct RecordEntity {
    pub id: Uuid,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
}

impl TryFrom<RecordDTO> for RecordEntity {
    type Error = DomainError;

    fn try_from(value: RecordDTO) -> Result<Self, Self::Error> {
        let entity = Self {
            id: Uuid::from_str(&value.id)?,
            temperature: value.temperature,
            pressure: value.pressure,
            humidity: value.humidity,
        };

        Ok(entity)
    }
}
