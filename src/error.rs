use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum DomainError {
    #[error("Error while excuting query")]
    QueryError(#[from] sqlx::Error),
    #[error("Error while encoding/decoding UUID")]
    UuidError(#[from] sqlx::types::uuid::Error),
}
