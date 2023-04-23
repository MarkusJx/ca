use crate::model::error_dto::ErrorDto;
use crate::util::types::WebResult;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::Display;
use keycloak::KeycloakError;
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
    pub message: String,
}

impl HttpResponseError {
    pub fn new(error: HttpResponseErrorCode, message: String) -> Self {
        Self { error, message }
    }

    pub fn internal_error<T: Into<String>>(message: T) -> Self {
        Self::new(HttpResponseErrorCode::InternalError, message.into())
    }

    pub fn bad_request<T: Into<String>>(message: T) -> Self {
        Self::new(HttpResponseErrorCode::BadRequest, message.into())
    }

    pub fn not_found<T: Into<String>>(message: T) -> Self {
        Self::new(HttpResponseErrorCode::NotFound, message.into())
    }

    pub fn unauthorized<T: Into<String>>(message: T) -> Self {
        Self::new(HttpResponseErrorCode::Unauthorized, message.into())
    }

    pub fn failed_dependency<T: Into<String>>(message: T) -> Self {
        Self::new(HttpResponseErrorCode::FailedDependency, message.into())
    }
}

impl Display for HttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

pub trait MapHttpResponseError<T> {
    fn map_internal_error(self, message: &str) -> Result<T, HttpResponseError>;

    fn map_bad_request(self, message: &str) -> Result<T, HttpResponseError>;

    fn map_not_found(self, message: &str) -> Result<T, HttpResponseError>;

    fn map_failed_dependency(self, message: &str) -> Result<T, HttpResponseError>;

    fn map_unauthorized(self, message: &str) -> Result<T, HttpResponseError>;
}

impl<T, E> MapHttpResponseError<T> for Result<T, E>
where
    E: Display,
{
    fn map_internal_error(self, message: &str) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Internal server error: {}", e);
            HttpResponseError::internal_error(message).into()
        })
    }

    fn map_bad_request(self, message: &str) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Bad request: {}", e);
            HttpResponseError::bad_request(message).into()
        })
    }

    fn map_not_found(self, message: &str) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Not found: {}", e);
            HttpResponseError::not_found(message).into()
        })
    }

    fn map_failed_dependency(self, message: &str) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Failed dependency: {}", e);
            HttpResponseError::failed_dependency(message).into()
        })
    }

    fn map_unauthorized(self, message: &str) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            error!("Unauthorized: {}", e);
            HttpResponseError::unauthorized(message).into()
        })
    }
}

pub trait MapKeycloakError<T> {
    fn map_keycloak_error(self, message: &str) -> Result<T, HttpResponseError>;
}

impl<T> MapKeycloakError<T> for Result<T, KeycloakError> {
    fn map_keycloak_error(self, message: &str) -> Result<T, HttpResponseError> {
        self.map_err(|e| {
            match e {
                KeycloakError::HttpFailure { status, body, text } => {
                    error!("Keycloak error: {} {:?} {}", status, body, text);
                }
                KeycloakError::ReqwestFailure(e) => {
                    error!("Keycloak error: {}", e);
                }
            }

            HttpResponseError::failed_dependency(message).into()
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
