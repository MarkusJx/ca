use crate::error::http_response_error::HttpResponseError;
use sea_orm::DbErr;

pub type DbResult<T> = Result<T, DbErr>;
pub type WebResult<T> = Result<T, HttpResponseError>;
