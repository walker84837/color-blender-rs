use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("failed to parse hex pair: {0}")]
    ParseError(#[from] ParseIntError),

    #[error("wrong hex format: {0}")]
    WrongFormat(String),
}

pub type ColorResult<T> = Result<T, ColorError>;
