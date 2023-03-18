use crate::entity::signing_request;
use shared::model::signing_request_dto::SigningRequestDto;

pub trait FromModel<T> {
    fn from_model(model: T) -> Self;
}

impl FromModel<signing_request::Model> for SigningRequestDto {
    fn from_model(model: signing_request::Model) -> Self {
        SigningRequestDto {
            client_id: model.client_id.to_string(),
            hash: model.hash,
            issued_at: model.issued_at.to_rfc3339(),
            certificate: None,
            serial_number: model.serial_number,
            subject_name: model.subject_name,
        }
    }
}
