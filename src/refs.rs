use std::path::{PathBuf, Path};
use std::fs;
use linked_hash_map::LinkedHashMap;
use crate::{
    repository::Repository,
    error::{
        WitError,
        builder::utf8_error
    }
};

pub enum Ref {
    Direct(DirectRef),
    Indirect(IndirectRef)
}

pub type IndirectRef = LinkedHashMap<String, Ref>;
pub type DirectRef = String;

pub fn resolve(repo: &Repository, ref_path: &str) -> Result<String, Box<WitError>> {
    let paths = ref_path.split('/').collect::<Vec<&str>>();
    let data = std::fs::read_to_string(Repository::file(repo, paths, false)?)?;
    if data.starts_with("ref: ") {
        self::resolve(repo, &data[5..])
    } else {
        Ok(data)
    }
}

pub fn list(repo: &Repository, path: Option<PathBuf>) -> Result<IndirectRef, Box<WitError>> {
    let path = if let None = path {
        Repository::dir(repo, vec!["refs"], false)?
    } else {
        path.unwrap()
    };

    let mut ret: IndirectRef = IndirectRef::new();

    for file in std::fs::read_dir(path)? {
        let can = file?;

        let name = String::from(
            can
            .file_name()
            .to_str().ok_or(utf8_error(format!("Could not read file name.")))?
        );
        if can.path().is_dir() {
            ret.insert(
                name,
                Ref::Indirect(
                    self::list(
                        repo,
                        Some(can.path())
                    )?
                )
            );
        } else {
            ret.insert(name.clone(), Ref::Direct(resolve(repo, &name)?));
        }
    }

    Ok(ret)
}

pub fn show(repo: &Repository, refs: &IndirectRef, with_hash: bool, prefix: &str) -> Result<(), Box<WitError>> {
    for (k, v) in refs.iter() {
        match v {
            Ref::Direct(ref_path) => {
                println!("{}{}{}",
                    ref_path.to_owned() + (if with_hash { " " } else { "" }),
                    prefix.to_owned() + (if prefix.is_empty() { "" } else { "/" }),
                    k
                );
            },
            Ref::Indirect(refs) => {
                self::show(repo, refs, with_hash, format!("{}{}{}", prefix, if prefix.is_empty() { "" } else { "/" }, k).as_str())?
            }
        }
    }
    Ok(())
}

pub fn create(repo: &Repository, ref_name: String, sha: String) -> Result<(), Box<WitError>> {
    fs::write(
        Repository::file(
            repo,
            Path::new("refs/")
                .join(&ref_name)
                .to_str()
                .ok_or(utf8_error(format!("Could not read file name.")))?
                .split('/')
                .collect::<Vec<&str>>(),
            true
        )?,
        sha + "\n"
    )?;
    Ok(())
}