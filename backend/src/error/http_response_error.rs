use crate::model::error_dto::ErrorDto;
use crate::util::types::WebResult;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::Display;
use log::error;
use shared::util::types::BasicResult;
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
    #[display(fmt = "FailedDependency")]
    FailedDependency,
}

#[derive(Debug, Clone)]
pub struct HttpResponseError {
    pub error: HttpResponseErrorCode,
    pub message: Option<String>,
}

impl HttpResponseError {
    pub fn new(error: HttpResponseErrorCode, message: Option<String>) -> Self {
        Self { error, message }
    }

    pub fn internal_error<T: Into<String>>(message: Option<T>) -> Self {
        Self::new(
            HttpResponseErrorCode::InternalError,
            message.map(|m| m.into()),
        )
    }

    pub fn bad_request<T: Into<String>>(message: Option<T>) -> Self {
        Self::new(HttpResponseErrorCode::BadRequest, message.map(|m| m.into()))
    }

    pub fn not_found<T: Into<String>>(message: Option<T>) -> Self {
        Self::new(HttpResponseErrorCode::NotFound, message.map(|m| m.into()))
    }

    pub fn unauthorized<T: Into<String>>(message: Option<T>) -> Self {
        Self::new(
            HttpResponseErrorCode::Unauthorized,
            message.map(|m| m.into()),
        )
    }

    pub fn failed_dependency<T: Into<String>>(message: Option<T>) -> Self {
        Self::new(
            HttpResponseErrorCode::FailedDependency,
            message.map(|m| m.into()),
        )
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
    fn map_internal_error(self, message: Option<&str>) -> Result<T, HttpResponseError>;

    fn map_bad_request(self, message: Option<&str>) -> Result<T, HttpResponseError>;

    fn map_not_found(self, message: Option<&str>) -> Result<T, HttpResponseError>;

    fn map_failed_dependency(self, message: Option<&str>) -> Result<T, HttpResponseError>;

    fn map_unauthorized(self, message: Option<&str>) -> Result<T, HttpResponseError>;
}

impl<T, E> MapHttpResponseError<T> for Result<T, E>
where
    E: Display,
{
    fn map_internal_error(self, message: Option<&str>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Internal server error: {}", e);
            HttpResponseError::internal_error(message).into()
        })
    }

    fn map_bad_request(self, message: Option<&str>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Bad request: {}", e);
            HttpResponseError::bad_request(message).into()
        })
    }

    fn map_not_found(self, message: Option<&str>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Not found: {}", e);
            HttpResponseError::not_found(message).into()
        })
    }

    fn map_failed_dependency(self, message: Option<&str>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Failed dependency: {}", e);
            HttpResponseError::failed_dependency(message).into()
        })
    }

    fn map_unauthorized(self, message: Option<&str>) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Unauthorized: {}", e);
            HttpResponseError::unauthorized(message).into()
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
            HttpResponseErrorCode::FailedDependency => StatusCode::FAILED_DEPENDENCY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&ErrorDto::from(self.clone())).unwrap())
    }
}

pub trait MapToBasicResult<T> {
    fn map_error_to_basic(self) -> BasicResult<T>;
}

impl<T> MapToBasicResult<T> for WebResult<T> {
    fn map_error_to_basic(self) -> BasicResult<T> {
        self.map_err(|e| e.to_string().into())
    }
}
