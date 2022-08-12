use crate::error::DomainError;

pub(crate) mod record;

pub(crate) trait TryToEntity {
    type OutputEntity;

    fn try_to_entity(self) -> Result<Self::OutputEntity, DomainError>;
}
