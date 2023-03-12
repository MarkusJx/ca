use crate::error::http_response_error::HttpResponseError;
use sea_orm::DbErr;
use std::error::Error;

pub type BasicResult<T> = Result<T, Box<dyn Error>>;
pub type DbResult<T> = Result<T, DbErr>;
pub type WebResult<T> = Result<T, HttpResponseError>;
