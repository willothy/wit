use std::fmt::Display;
use std::error::Error;
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
pub enum WitErrorType {
    DebugError,
    IOError,
    RepoCreationError,
    InheritedError,
    VersionMismatchError,
    MalformedObjectError,
    UnknownObjectTypeError,
    PathConversionError,
    CLIArgumentError,
    UTF8ConversionError,
    InvalidModeLengthError,
    RepoNotFoundError
}

impl Display for WitErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
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

// Conversions for WitError
impl From<std::io::Error> for WitError {
    fn from(err: std::io::Error) -> Self {
        WitError::new(WitErrorType::InheritedError, err.to_string())
    }
}

impl From<std::io::Error> for Box<WitError> {
    fn from(err: std::io::Error) -> Self {
        Box::new(WitError::new(WitErrorType::InheritedError, err.to_string()))
    }
}

impl From<Box<std::io::Error>> for Box<WitError> {
    fn from(err: Box<std::io::Error>) -> Self {
        Box::new(WitError::from(*err))
    }
}

impl From<ParseIntError> for Box<WitError> {
    fn from(err: ParseIntError) -> Self {
        Box::new(WitError::new(WitErrorType::InheritedError, err.to_string()))
    }
}

impl From<Utf8Error> for Box<WitError> {
    fn from(err: Utf8Error) -> Self {
        Box::new(WitError::new(WitErrorType::InheritedError, err.to_string()))
    }
}

impl From<FromUtf8Error> for Box<WitError> {
    fn from(err: FromUtf8Error) -> Self {
        Box::new(WitError::new(WitErrorType::UTF8ConversionError, err.to_string()))
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

    pub fn version_mismatch_error(message: String) -> Box<WitError> {
        Box::new(WitError::new(VersionMismatchError, message))
    }

    pub fn malformed_object_error(message: String) -> Box<WitError> {
        Box::new(WitError::new(MalformedObjectError, message))
    }

    pub fn path_conversion_error() -> Box<WitError> {
        Box::new(WitError::new(MalformedObjectError, format!("Could not convert path to string")))
    }

    pub fn cli_argument_error(message: String) -> Box<WitError> {
        Box::new(WitError::new(CLIArgumentError, message))
    }

    pub fn unknown_object_error(message: String) -> Box<WitError> {
        Box::new(WitError::new(UnknownObjectTypeError, message))
    }

    pub fn utf8_error(message: String) -> Box<WitError> {
        Box::new(WitError::new(UTF8ConversionError, message))
    }

    pub fn mode_error(length: usize) -> Box<WitError> {
        Box::new(WitError::new(InvalidModeLengthError, format!("Invalid mode length: {}", length)))
    }
}