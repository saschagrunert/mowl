//! Basic error handling mechanisms
use std::error::Error;
use std::convert::From;
use std::{io, fmt};
use term;

/// The result type for the Parsing
pub type LogResult<'a, T> = Result<T, LogError>;

/// Representation for an error of the library
pub struct LogError {
    /// The error variant
    pub code: ErrorType,

    /// Additional description for the error
    pub description: String,

    /// The cause for this error
    pub cause: Option<Box<Error>>,
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Code: {:?}, Description: {}",
               self.code,
               self.description)
    }
}

impl fmt::Debug for LogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for LogError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// Errors not directly from the library (like OS errors)
    Other,

    /// Internal errors which should not happen at all
    Internal,
}

// Error conversion
macro_rules! from_error {
    ($($p:ty,)*) => (
        $(impl From<$p> for LogError {
            fn from(err: $p) -> LogError {
                LogError {
                    code: ErrorType::Other,
                    description: err.description().to_owned(),
                    cause: Some(Box::new(err)),
                }
            }
        })*
    )
}

from_error! {
    io::Error,
    term::Error,
}

/// Throw an internal error
pub fn bail(code: ErrorType, description: &fmt::Display) -> LogError {
    LogError {
        code: code,
        description: description.to_string(),
        cause: None,
    }
}
