use serde::{Deserialize, Serialize};

use crate::entities::record::RecordEntity;

#[derive(Serialize, Deserialize)]
pub(crate) struct RecordDTO {
    pub id: String,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
}

impl From<RecordEntity> for RecordDTO {
    fn from(value: RecordEntity) -> Self {
        Self {
            id: value.id.to_string(),
            temperature: value.temperature,
            pressure: value.pressure,
            humidity: value.humidity,
        }
    }
}
