use crate::errors::http_response_error::HttpResponseError;
use actix_web::ResponseError;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct ErrorDto {
    pub code: u16,
    pub error: String,
    pub message: Option<String>,
}

impl From<HttpResponseError> for ErrorDto {
    fn from(error: HttpResponseError) -> Self {
        Self {
            code: error.status_code().as_u16(),
            error: format!("{}", error.error),
            message: error.message,
        }
    }
}
