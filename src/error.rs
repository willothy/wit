use std::fmt::Display;
use std::error::Error;
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use self::WitErrorType::*;

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
    CLIUnknownCommandError,
    CLINoCommandError,
    UTF8ConversionError,
    InvalidModeLengthError,
    RepoNotFoundError,
    AmbiguousReferenceError,
    UnknownReferenceError,
    MissingDataError,
    NotADirectoryError,
    DirectoryNotEmptyError,
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
        WitError::new(InheritedError, err.to_string())
    }
}

impl From<regex::Error> for Box<WitError> {
    fn from(err: regex::Error) -> Self {
        Box::new(WitError::new(InheritedError, err.to_string()))
    }
}

impl From<std::io::Error> for Box<WitError> {
    fn from(err: std::io::Error) -> Self {
        Box::new(WitError::new(InheritedError, err.to_string()))
    }
}

impl From<Box<std::io::Error>> for Box<WitError> {
    fn from(err: Box<std::io::Error>) -> Self {
        Box::new(WitError::from(*err))
    }
}

impl From<ParseIntError> for Box<WitError> {
    fn from(err: ParseIntError) -> Self {
        Box::new(WitError::new(InheritedError, err.to_string()))
    }
}

impl From<Utf8Error> for Box<WitError> {
    fn from(err: Utf8Error) -> Self {
        Box::new(WitError::new(InheritedError, err.to_string()))
    }
}

impl From<FromUtf8Error> for Box<WitError> {
    fn from(err: FromUtf8Error) -> Self {
        Box::new(WitError::new(UTF8ConversionError, err.to_string()))
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
    pub fn debug_err() -> Box<WitError> {
        Box::new(WitError::new(DebugError, String::from("Generic debug error.")))
    }

    pub fn io_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(IOError, message))
    }

    pub fn repo_creation_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(RepoCreationError, message))
    }

    pub fn version_mismatch_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(VersionMismatchError, message))
    }

    pub fn malformed_object_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(MalformedObjectError, message))
    }

    pub fn path_conversion_err() -> Box<WitError> {
        Box::new(WitError::new(MalformedObjectError, format!("Could not convert path to string")))
    }

    pub fn cli_argument_err(arg: &str) -> Box<WitError> {
        Box::new(WitError::new(
            CLIArgumentError,
            format!("Required argument '{}' was not provided.", arg)
        ))
    }

    pub fn cli_unknown_command_err(command: &str) -> Box<WitError> {
        Box::new(WitError::new(
            CLIUnknownCommandError,
            format!("Unknown command '{}'", command)
        ))
    }

    pub fn cli_no_command_err() -> Box<WitError> {
        Box::new(WitError::new(
            CLINoCommandError,
            format!("No command provided.")
        ))
    }

    pub fn unknown_object_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(UnknownObjectTypeError, message))
    }

    pub fn utf8_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(UTF8ConversionError, message))
    }

    pub fn mode_err(length: usize) -> Box<WitError> {
        Box::new(WitError::new(InvalidModeLengthError, format!("Invalid mode length: {}", length)))
    }

    pub fn repo_not_found_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(RepoNotFoundError, message))
    }

    pub fn pwd_not_repo_err() -> Box<WitError> {
        Box::new(WitError::new(
            RepoNotFoundError,
            format!("No repository found in {}", std::env::current_dir().unwrap().display())
        ))
    }

    pub fn ambiguous_reference_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(AmbiguousReferenceError, message))
    }

    pub fn unknown_reference_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(UnknownReferenceError, message))
    }

    pub fn missing_data_err(message: String) -> Box<WitError> {
        Box::new(WitError::new(MissingDataError, message))
    }

    pub fn not_a_directory_err(path: &std::path::PathBuf) -> Box<WitError> {
        Box::new(WitError::new(NotADirectoryError, path.display().to_string()))
    }

    pub fn dir_not_empty_err(path: &std::path::PathBuf) -> Box<WitError> {
        Box::new(WitError::new(DirectoryNotEmptyError, path.display().to_string()))
    }
}