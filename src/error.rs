use std::fmt::Display;

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

pub struct WitError {
    error: WitErrorType,
    message: String
}

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

pub mod WitErrorBuilder {
    use std::error::Error;
    use super::{WitError, WitErrorType::*};
    // For debug use only
    pub fn generic_error() -> WitError {
        WitError {
            error: DebugError,
            message: String::from("Generic debug error.")
        }
    }

    pub fn io_error(message: String) -> WitError {
        WitError {
            error: IOError,
            message
        }
    }

    pub fn repo_creation_error(message: String) -> WitError {
        WitError {
            error: RepoCreationError,
            message
        }
    }

    pub fn from_error(error: &dyn Error) -> String {
        //format!("{}", error)
        println!("{}", error);
        error.to_string()
    }
}