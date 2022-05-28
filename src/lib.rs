use std::path::{ Path, PathBuf };
use std::fs;

use ini::configparser::ini::Ini;

use self::WitErrorType::{
    DebugError,
    IOError,
    RepoCreationError
};

pub enum WitErrorType {
    DebugError,
    IOError,
    RepoCreationError
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

    // For debug use only
    pub fn generic() -> Self {
        WitError {
            error: DebugError,
            message: String::from("Generic debug error.")
        }
    }

    pub fn io_error(message: &str) -> Self {
        WitError {
            error: IOError,
            message: String::from(message)
        }
    }
}

pub struct Repository<'repo> {
    pub worktree: &'repo str,
    pub git_dir: PathBuf,
    pub conf: Ini
}

impl<'repo> Repository<'repo> {
    pub fn new(path: &'repo str, force: bool) -> Result<Repository, WitError> {
        let git_dir = Path::new(path).join(".git");
        let mut config = Ini::new();
        let config_path = git_dir.join("config");

        if config_path.exists() && config_path.is_file() {
            config.load(config_path.to_str().unwrap()).unwrap();
            Ok(Repository {
                worktree: path,
                git_dir: git_dir,
                conf: config
            })
        } else {
            Err(WitError::new(RepoCreationError, format!("Could not create repository in {}", path)))
        }
    }

    fn repo_path(base: &Repository, paths: Vec<&str>) -> PathBuf {
        let mut result = base.git_dir.clone();
        for fragment in paths {
            result = result.join(fragment);
        }
        result
    }

    fn repo_file(repo: &Repository, paths: Vec<&str>, mkdir: bool) -> Result<PathBuf, WitError> {
        let dirs = if paths.len() > 0 {
            paths[..paths.len()-2].to_vec()
        } else {
            Vec::new()
        };

        match Self::repo_dir(repo, dirs, mkdir) {
            Ok(_) => Ok(Self::repo_path(repo, paths)),
            Err(e) => Err(e)
        }
    }

    fn repo_dir(repo: &Repository, paths: Vec<&str>, mkdir: bool) -> Result<PathBuf, WitError> {
        let path = Self::repo_path(repo, paths);
        if path.exists() {
            if path.is_dir() {
                Ok(path)
            } else  {
                Err(WitError::io_error(""))
            }
        } else if mkdir {
            match fs::create_dir_all(&path) {
                Ok(_) => Ok(path),
                Err(e) => Err(WitError::io_error(e.to_string().as_str()))
            }
        } else {
            Err(WitError::io_error(format!("Failed to create {}", path.display()).as_str()))
        }
    }
}