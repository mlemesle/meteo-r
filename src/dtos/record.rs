use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::record::RecordEntity;

#[derive(Serialize, Deserialize)]
pub(crate) struct RecordDTO {
    pub id: String,
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub date: DateTime<Utc>,
}

impl From<RecordEntity> for RecordDTO {
    fn from(value: RecordEntity) -> Self {
        Self {
            id: value.id.unwrap_or_default().to_string(),
            temperature: value.temperature,
            pressure: value.pressure,
            humidity: value.humidity,
            date: value.date,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct RecordDTOWithoutID {
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub date: DateTime<Utc>,
}
