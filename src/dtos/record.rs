use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::{
    entities::{record::RecordEntity, TryToEntity},
    error::DomainError,
};

#[derive(Serialize, Deserialize)]
pub(crate) struct RecordDTO {
    pub id: String,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
}

impl TryToEntity for RecordDTO {
    type OutputEntity = RecordEntity;

    fn try_to_entity(self) -> Result<Self::OutputEntity, DomainError> {
        let output_entity = Self::OutputEntity {
            id: Uuid::parse_str(&self.id)?,
            temperature: self.temperature,
            pressure: self.pressure,
            humidity: self.humidity,
        };

        Ok(output_entity)
    }
}
