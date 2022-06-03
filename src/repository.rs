use std::path::{PathBuf, Path};
use std::fs;

use ini::configparser::ini::Ini;
use crate::error::{builder::*, WitError};

#[derive(Debug, Clone)]
pub struct Repository {
    pub worktree: PathBuf,
    pub git_dir: PathBuf,
    pub conf: Ini
}

impl Repository {
    pub fn new(path: &str, force: bool) -> Result<Repository, Box<WitError>> {
        let git_dir = Path::new(path).join(".git");
        let mut config = Ini::new();
        let config_path = git_dir.join("config");

        if !(force || git_dir.is_dir()) {
            Err(repo_creation_error(format!("{} is not a git repository", path)))?
        }

        if config_path.exists() && config_path.is_file() {
            config.load(match config_path.to_str() {
                Some(string) => string,
                None => Err(repo_creation_error(format!("Could not load config from {:?}", config_path)))?
            }).unwrap();
        } else if !force {
            Err(repo_creation_error(format!("Could not create repository in {}", path)))?
        }

        if !force {
            let version = config
                .get("core", "repositoryformatversion")
                .ok_or(
                    version_mismatch_error(format!("Could not read repository format version from config."))
                )?.parse::<i32>()?;
            if version != 0 {
                Err(version_mismatch_error(format!("Unsupported repositoryformatversion {}", version)))?
            }
        }
        Ok(Repository {
            worktree: PathBuf::from(path),
            git_dir: git_dir,
            conf: config
        })
    }

    pub fn find(path: &str, required: bool) -> Result<Option<Repository>, Box<WitError>> {
        let path = fs::canonicalize(path)?;

        if path.join(".git").is_dir() {
            return Ok(
                Some(
                    Repository::new(
                        path
                            .to_str()
                            .ok_or(path_conversion_error())?,
                        false
                    )?
                )
            )
        }

        let parent = fs::canonicalize(path.join(".."))?;

        if parent == path {
            return if required {
                Err(io_error(format!("No git directory in {:?}", path)))?
            } else {
                Ok(None)
            }
        }

        Self::find(
            parent.to_str().ok_or(path_conversion_error())?,
            required
        )
    }

    pub fn path(base: &Repository, paths: Vec<&str>) -> PathBuf {
        let mut result = base.git_dir.clone();
        for fragment in paths {
            result = result.join(fragment);
        }
        result
    }

    pub fn file(repo: &Repository, paths: Vec<&str>, mkdir: bool) -> Result<PathBuf, Box<WitError>> {
        let dirs = if paths.len() > 0 {
            paths[0..paths.len()-1].to_vec()
        } else {
            Vec::new()
        };

        match Self::dir(repo, dirs, mkdir) {
            Ok(_) => Ok(Self::path(repo, paths)),
            Err(e) => Err(e)
        }
    }

    pub fn dir(repo: &Repository, paths: Vec<&str>, mkdir: bool) -> Result<PathBuf, Box<WitError>> {
        let path = Self::path(repo, paths);
        if path.exists() {
            if path.is_dir() {
                Ok(path)
            } else  {
                Err(io_error(format!("{:?} is not a directory.", path)))
            }
        } else if mkdir {
            return match fs::create_dir_all(&path) {
                Ok(_) => Ok(path),
                Err(err) => Err(Box::<WitError>::from(err))
            }
        } else {
            return Err(io_error(format!("Failed to create {:?}", path)))
        }
    }

    pub fn create(path: &str) -> Result<Self, Box<WitError>> {
        let repo = Self::new(path, true)?;

        if repo.worktree.exists() {
            if !repo.worktree.is_dir() {
                Err(repo_creation_error(format!("{} is not a directory.", path)))?
            }
            if !repo.worktree.read_dir().map(|mut i| i.next().is_none()).unwrap_or(false) {
                Err(repo_creation_error(format!("Directory {} is not empty.", path)))?
            }
        } else {
            if let Err(e) = fs::create_dir_all(&repo.worktree) {
                Err(Box::<WitError>::from(e))?
            }
        }

        Self::dir(&repo, vec!["branches"], true)?;
        Self::dir(&repo, vec!["objects"], true)?;
        Self::dir(&repo, vec!["refs", "tags"], true)?;
        Self::dir(&repo, vec!["refs", "heads"], true)?;

        // .git/description
        if let Err(err) = fs::write(Self::file(&repo, vec!["description"], true)?, "Unnamed repository; edit this file 'description' to name the repository.\n") {
            Err(Box::<WitError>::from(err))?
        }

        // .git/HEAD
        if let Err(err) = fs::write(Self::file(&repo, vec!["HEAD"], true)?, "ref: refs/heads/main\n") {
            Err(Box::<WitError>::from(err))?
        }

        // .git/config
        if let Err(err) = Self::default_config().write(
            Self::file(&repo, vec!["config"], true)?
                .to_str()
                .ok_or(repo_creation_error(format!("Error opening config file.")))?
        ) {
            Err(Box::<WitError>::from(err))?
        }

        Ok(repo)
    }

    fn default_config() -> Ini {
        let mut config = Ini::new();
        config.set("core", "repositoryformatversion", Some(String::from("0")));
        config.set("core", "filemode", Some(String::from("false")));
        config.set("core", "bare", Some(String::from("false")));
        config
    }
}