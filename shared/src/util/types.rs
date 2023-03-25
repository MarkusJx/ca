use std::error::Error;

pub type BasicResult<T> = Result<T, Box<dyn Error>>;
