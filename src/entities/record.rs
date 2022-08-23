use std::str::FromStr;

use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

use crate::{
    dtos::record::{RecordDTO, RecordDTOWithoutID},
    error::DomainError,
};

#[derive(sqlx::FromRow)]
pub(crate) struct RecordEntity {
    pub id: Option<Uuid>,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub date: DateTime<Utc>,
}

impl TryFrom<RecordDTO> for RecordEntity {
    type Error = DomainError;

    fn try_from(value: RecordDTO) -> Result<Self, Self::Error> {
        let entity = Self {
            id: Some(Uuid::from_str(&value.id)?),
            temperature: value.temperature,
            pressure: value.pressure,
            humidity: value.humidity,
            date: value.date,
        };

        Ok(entity)
    }
}

impl TryFrom<RecordDTOWithoutID> for RecordEntity {
    type Error = DomainError;

    fn try_from(value: RecordDTOWithoutID) -> Result<Self, Self::Error> {
        Ok(Self {
            id: None,
            temperature: value.temperature,
            pressure: value.pressure,
            humidity: value.humidity,
            date: value.date,
        })
    }
}
