pub(crate) mod record;

pub(crate) trait ToDTO {
    type OutputDTO;

    fn to_dto(self) -> Self::OutputDTO;
}
