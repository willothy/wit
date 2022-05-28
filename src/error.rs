use std::fmt::Display;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum WitErrorType {
    DebugError,
    IOError,
    RepoCreationError
}

impl Display for WitErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WitErrorType::DebugError => write!(f, "DebugError"),
            WitErrorType::IOError => write!(f, "IOError"),
            WitErrorType::RepoCreationError => write!(f, "RepoCreationError"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WitError {
    error: WitErrorType,
    message: String
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

impl Display for WitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

pub mod builder {
    use std::error::Error;
    use super::{WitError, WitErrorType::*};

    // For debug use only
    #[allow(dead_code)]
    pub fn debug_error() -> WitError {
        WitError::new(DebugError, String::from("Generic debug error."))
    }

    pub fn io_error(message: String) -> WitError {
        WitError::new(IOError, message)
    }

    pub fn repo_creation_error(message: String) -> WitError {
        WitError::new(RepoCreationError, message)
    }

    pub fn from_error(error: &dyn Error) -> String {
        error.to_string()
    }
}