use std::path::{PathBuf, Path};
use std::fs;

use ini::configparser::ini::Ini;
use crate::error::{builder::*, WitError};

pub struct Repository {
    pub worktree: PathBuf,
    pub git_dir: PathBuf,
    pub conf: Ini
}

impl Repository {
    pub fn new(path: &str, force: bool) -> Result<Repository, WitError> {
        let git_dir = Path::new(path).join(".git");
        let mut config = Ini::new();
        let config_path = git_dir.join("config");

        if !(force || git_dir.is_dir()) {
            return Err(repo_creation_error(format!("{} is not a git repository", path)))
        }

        if config_path.exists() && config_path.is_file() {
            config.load(match config_path.to_str() {
                Some(string) => string,
                None => return Err(repo_creation_error(format!("Could not load config from {:?}", config_path)))
            }).unwrap();
            Ok(Repository {
                worktree: PathBuf::from(path),
                git_dir: git_dir,
                conf: config
            })
        } else if !force {
            return Err(repo_creation_error(format!("Could not create repository in {}", path)))
        } else {
            Ok(Repository {
                worktree: PathBuf::from(path),
                git_dir: git_dir,
                conf: config
            })
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
            paths[0..paths.len()-1].to_vec()
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
                Err(io_error(format!("{:?} is not a directory.", path)))
            }
        } else if mkdir {
            match fs::create_dir_all(&path) {
                Ok(_) => Ok(path),
                Err(e) => Err(io_error(from_error(&e)))
            }
        } else {
            Err(io_error(format!("Failed to create {:?}", path)))
        }
    }

    pub fn repo_create(path: &str) -> Result<Self, WitError> {
        let repo = Self::new(path, true)?;

        if repo.worktree.exists() {
            if !repo.worktree.is_dir() {
                return Err(repo_creation_error(format!("{} is not a directory.", path)))
            }
            if !repo.worktree.read_dir().map(|mut i| i.next().is_none()).unwrap_or(false) {
                return Err(repo_creation_error(format!("Directory {} is not empty.", path)))
            }
        } else {
            if let Err(e) = fs::create_dir_all(&repo.worktree) {
                return Err(repo_creation_error(from_error(&e)))
            }
        }

        Self::repo_dir(&repo, vec!["branches"], true)?;
        Self::repo_dir(&repo, vec!["objects"], true)?;
        Self::repo_dir(&repo, vec!["refs", "tags"], true)?;
        Self::repo_dir(&repo, vec!["refs", "heads"], true)?;

        // .git/description
        if let Err(err) = fs::write(Self::repo_file(&repo, vec!["description"], true)?, "Unnamed repository; edit this file 'description' to name the repository.\n") {
            return Err(repo_creation_error(from_error(&err)))
        }

        // .git/HEAD
        if let Err(err) = fs::write(Self::repo_file(&repo, vec!["HEAD"], true)?, "ref: refs/heads/main\n") {
            return Err(repo_creation_error(from_error(&err)))
        }

        // .git/config
        if let Err(err) = Self::repo_default_config().write(
            match Self::repo_file(&repo, vec!["config"], true)?.to_str() {
                Some(path) => path,
                None => return Err(repo_creation_error(format!("Error opening config file.")))
            }
        ) {
            return Err(repo_creation_error(from_error(&err)))
        }

        Ok(repo)
    }

    fn repo_default_config() -> Ini {
        let mut config = Ini::new();
        config.set("core", "repositoryformatversion", Some(String::from("0")));
        config.set("core", "filemode", Some(String::from("false")));
        config.set("core", "bare", Some(String::from("false")));
        config
    }
}