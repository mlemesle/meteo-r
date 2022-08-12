use sqlx::types::Uuid;

use crate::dtos::{record::RecordDTO, ToDTO};

#[derive(sqlx::FromRow)]
pub(crate) struct RecordEntity {
    pub id: Uuid,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
}

impl ToDTO for RecordEntity {
    type OutputDTO = RecordDTO;

    fn to_dto(self) -> Self::OutputDTO {
        Self::OutputDTO {
            id: self.id.to_string(),
            temperature: self.temperature,
            pressure: self.pressure,
            humidity: self.humidity,
        }
    }
}
