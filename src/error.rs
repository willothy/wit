use std::fmt::Display;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum WitErrorType {
    DebugError,
    IOError,
    RepoCreationError,
    InheritedError
}

impl Display for WitErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WitErrorType::DebugError => write!(f, "DebugError"),
            WitErrorType::IOError => write!(f, "IOError"),
            WitErrorType::RepoCreationError => write!(f, "RepoCreationError"),
            WitErrorType::InheritedError => write!(f, "InheritedError"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WitError {
    error: WitErrorType,
    message: String,
}

impl Error for WitError {}

impl WitError {
    pub fn new(error_type: WitErrorType, message: String) -> Self {
        WitError {
            error: error_type,
            message
        }
    }
}

impl From<std::io::Error> for WitError {
    fn from(err: std::io::Error) -> Self {
        WitError::new(WitErrorType::InheritedError, err.to_string())
    }
}

impl Display for WitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

pub mod builder {
    use super::{WitError, WitErrorType::*};

    // For debug use only
    #[allow(dead_code)]
    pub fn debug_error() -> Box<WitError> {
        Box::new(WitError::new(DebugError, String::from("Generic debug error.")))
    }

    pub fn io_error(message: String) -> Box<WitError> {
        Box::new(WitError::new(IOError, message))
    }

    pub fn repo_creation_error(message: String) -> Box<WitError> {
        Box::new(WitError::new(RepoCreationError, message))
    }
}