use thiserror::Error as ThisError;
use crate::{ReturnCode, StatusLine};

/// Client error, either I/O error or SSIP error.
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Not ready")]
    NotReady,
    #[error("SSIP: {0}")]
    Ssip(StatusLine),
    #[error("Too few lines")]
    TooFewLines,
    #[error("Too many lines")]
    TooManyLines,
    #[error("Unexpected status: {0}")]
    UnexpectedStatus(ReturnCode),
    #[error("Unexpected EOF: {0}")]
    UnexpectedEof(&'static str),
    #[error("Invalid data: {0}")]
    InvalidData(&'static str),
}

impl Error {
    /// Invalid data I/O error
    pub fn invalid_data(msg: &'static str) -> Self {
        Error::InvalidData(msg)
    }

    /// Unexpected EOF I/O error
    pub fn unexpected_eof(msg: &'static str) -> Self {
        Error::UnexpectedEof(msg)
    }
}

