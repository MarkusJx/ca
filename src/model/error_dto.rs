use crate::error::http_response_error::HttpResponseError;
use actix_web::ResponseError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorDto {
    /// The error code
    #[schema(example = 500)]
    pub code: u16,
    /// The error as string
    #[schema(example = "Internal Server Error")]
    pub error: String,
    /// The error message
    #[schema(example = "Something went wrong")]
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
