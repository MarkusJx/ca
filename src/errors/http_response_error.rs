use crate::models::error_dto::ErrorDto;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::Display;
use log::error;
use paperclip::actix::api_v2_errors;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Display)]
pub enum HttpResponseErrorCode {
    #[display(fmt = "Internal server error")]
    InternalError,
    #[display(fmt = "Bad request")]
    BadRequest,
    #[display(fmt = "Not found")]
    NotFound,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

#[derive(Debug, Clone)]
#[api_v2_errors(
    code = 500,
    description = "Internal server error",
    schema = "ErrorDto",
    code = 404,
    description = "Not found",
    schema = "ErrorDto",
    code = 401,
    description = "Unauthorized"
)]
pub struct HttpResponseError {
    pub error: HttpResponseErrorCode,
    pub message: Option<String>,
}

impl HttpResponseError {
    pub fn new(error: HttpResponseErrorCode, message: Option<String>) -> Self {
        Self { error, message }
    }

    pub fn internal_error(message: Option<String>) -> Self {
        Self::new(HttpResponseErrorCode::InternalError, message)
    }

    pub fn bad_request(message: Option<String>) -> Self {
        Self::new(HttpResponseErrorCode::BadRequest, message)
    }

    pub fn not_found(message: Option<String>) -> Self {
        Self::new(HttpResponseErrorCode::NotFound, message)
    }

    pub fn unauthorized(message: Option<String>) -> Self {
        Self::new(HttpResponseErrorCode::Unauthorized, message)
    }
}

impl Display for HttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}: {}", self.error, message),
            None => write!(f, "{}", self.error),
        }
    }
}

pub trait MapHttpResponseError<T> {
    fn map_internal_error(self, message: Option<String>) -> Result<T, HttpResponseError>;

    fn map_bad_request(self, message: Option<String>) -> Result<T, HttpResponseError>;

    fn map_not_found(self, message: Option<String>) -> Result<T, HttpResponseError>;
}

impl<T, E> MapHttpResponseError<T> for Result<T, E>
where
    E: Display,
{
    fn map_internal_error(self, message: Option<String>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Internal server error: {}", e);
            HttpResponseError::internal_error(message).into()
        })
    }

    fn map_bad_request(self, message: Option<String>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Bad request: {}", e);
            HttpResponseError::bad_request(message).into()
        })
    }

    fn map_not_found(self, message: Option<String>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Not found: {}", e);
            HttpResponseError::new(HttpResponseErrorCode::NotFound, message).into()
        })
    }
}

impl error::ResponseError for HttpResponseError {
    fn status_code(&self) -> StatusCode {
        match self.error {
            HttpResponseErrorCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpResponseErrorCode::BadRequest => StatusCode::BAD_REQUEST,
            HttpResponseErrorCode::NotFound => StatusCode::NOT_FOUND,
            HttpResponseErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&ErrorDto::from(self.clone())).unwrap())
    }
}
