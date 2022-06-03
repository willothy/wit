use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::str::from_utf8;

use crypto::digest::Digest;
use flate2::Compression;
use flate2::write::{ZlibEncoder};
use flate2::read::ZlibDecoder;
use crypto;

use crate::blob::Blob;
use crate::commit::Commit;
use crate::error::{WitError, builder::*};
use crate::repository::Repository;
use crate::tag::Tag;
use crate::tree::Tree;

pub trait Find<T> {
    fn find(&self, element: T) -> Result<usize, Box<WitError>> { self.find_from(element, 0) }

    fn find_from(&self, element: T, start: usize) -> Result<usize, Box<WitError>>;

    fn find_some(&self, element: T, start: usize) -> Option<usize>;

    #[allow(unused_variables)]
    fn find_signed(&self, element: T, start: usize) -> isize {0}

    #[allow(unused_variables)]
    fn find_exact(&self, element: T, start: usize) -> usize {0}
}

pub trait Replace {
    fn replace(&mut self, from: &str, to: &str) -> Vec<u8>;
}

impl Replace for Vec<u8> {
    fn replace(&mut self, from: &str, to: &str) -> Vec<u8> {
        let mut result = Vec::new();
        let mut start = 0;
        loop {
            let end = match self.find_some(from.as_bytes()[0], start) {
                Some(val) => val,
                None => break
            };

            result.extend_from_slice(&self[start..end]);
            result.extend_from_slice(to.as_bytes());
            start = end + from.len();
        }
        result.extend_from_slice(&self[start..]);
        result
    }
}

impl<T: PartialEq + std::fmt::Debug> Find<T> for Vec<T> {
    fn find_from(&self, element: T, start: usize) -> Result<usize, Box<WitError>> {
        self.iter().skip(start).position(|el| *el == element).ok_or(
            io_error(format!("{:?} not found.", element))
        )
    }

    fn find_some(&self, element: T, start: usize) -> Option<usize> {
        self.iter().skip(start).position(|el| *el == element)
    }

    fn find_signed(&self, element: T, start: usize) -> isize {
        match self.iter().skip(start).position(|el| *el == element) {
            Some(idx) => idx as isize,
            None => -1
        }
    }

    fn find_exact(&self, element: T, start: usize) -> usize {
        self.iter().skip(start).position(|el| *el == element).unwrap()
    }
}

pub enum WitObject<'a> {
    BlobObject(Blob<'a>),
    CommitObject(Commit<'a>),
    TreeObject(Tree),
    TagObject(Tag<'a>)
}

impl<'a> WitObject<'a> {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            WitObject::BlobObject(blob) => blob.serialize(),
            WitObject::CommitObject(commit) => commit.serialize(),
            WitObject::TreeObject(tree) => tree.serialize(),
            WitObject::TagObject(tag) => tag.serialize()
        }
    }

    pub fn deserialize(&mut self, data: Vec<u8>) -> Result<(), Box<WitError>> {
        match self {
            WitObject::BlobObject(blob) => blob.deserialize(data),
            WitObject::CommitObject(commit) => commit.deserialize(data),
            WitObject::TreeObject(tree) => tree.deserialize(data),
            WitObject::TagObject(tag) => tag.deserialize(data)
        }
    }

    pub fn fmt(&self) -> Vec<u8> {
        match self {
            WitObject::BlobObject(blob) => blob.fmt(),
            WitObject::CommitObject(commit) => commit.fmt(),
            WitObject::TreeObject(tree) => tree.fmt(),
            WitObject::TagObject(tag) => tag.fmt()
        }
    }

    pub fn repo(&self) -> Option<& Repository> {
        match self {
            WitObject::BlobObject(blob) => blob.repo(),
            WitObject::CommitObject(commit) => commit.repo(),
            WitObject::TreeObject(tree) => tree.repo(),
            WitObject::TagObject(tag) => tag.repo()
        }
    }
}

pub trait Object {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: Vec<u8>) -> Result<(), Box<WitError>>;

    fn fmt(&self) -> Vec<u8>;
    fn repo(&self) -> Option<&Repository>;
}

pub fn read<'a>(repo: &'a Repository, sha: &'a str) -> Result<WitObject<'a>, Box<WitError>> {
    let path = Repository::file(&repo, vec!["objects", &sha[..2], &sha[2..]], false)?;

    let raw = fs::read(path)?;
    let mut decoded = Vec::<u8>::new();
    ZlibDecoder::new(&raw[..]).read_exact(&mut decoded)?;

    let x = decoded.find(b' ') ?;
    let fmt = &decoded[..x];

    let y = decoded[x..].to_vec().find(b'\x00')?;

    let size = from_utf8(&decoded[x..y])?.parse::<usize>()?;
    if size != decoded.len() - y - 1 {
        Err(malformed_object_error(format!("Malformed object {}: bad length", sha)))?
    }

    build(from_utf8(&fmt)?, Some(repo), Some(raw[y+1..].to_vec()))
}

pub fn find<'a>(repo: &'a Repository, name: &str, fmt: Option<&str>, follow: bool) -> Result<String, Box<WitError>> {
    Ok(String::from(name))
}

pub fn write(obj: WitObject, actually_write: bool) -> Result<String, Box<WitError>> {
    let data = obj.serialize();
    let mut result = Vec::new();
    result.extend(obj.fmt());
    result.extend(vec![b' ']);
    result.extend(data.len().to_string().as_bytes().to_vec());
    result.extend(vec![b'\x00']);
    result.extend(data);

    let mut sha = crypto::sha1::Sha1::new();
    sha.input(&result);
    let sha = sha.result_str();

    if actually_write {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&result)?;

        fs::write(
            Repository::file(obj.repo().ok_or(debug_error())?, vec!["objects"], actually_write)?, // Path
            encoder.finish()? // Data
        )?;
    }

    Ok(sha)
}

fn build<'a>(fmt: &str, repo: Option<&'a Repository>, data: Option<Vec<u8>>) -> Result<WitObject<'a>, Box<WitError>> {
    match fmt {
        "blob" => Ok(WitObject::BlobObject(Blob::new(repo, data.ok_or(debug_error())?))),
        "commit" => Ok(WitObject::CommitObject(Commit::new(repo))),
        "tree" => Ok(WitObject::TreeObject(Tree::from(&data.ok_or(debug_error())?)?)),
        "tag" => Ok(WitObject::TagObject(Tag::new(repo))),
        _ => Err(unknown_object_error(format!("Unknown object type {}", fmt)))
    }
}

pub fn hash<'a>(fd: &str, fmt: &str, repo: Option<&'a Repository>) -> Result<String, Box<WitError>>{
    write(build(fmt, repo, Some(fs::read(fd)?))?, repo.is_none())
}

pub fn graphviz(repo: &Repository, sha: String, seen: &mut Vec<String>) -> Result<(), Box<WitError>> {
    if seen.contains(&sha) {
        return Ok(())
    }

    seen.push(sha.clone());
    let commit = match self::read(repo, &sha)? {
        WitObject::CommitObject(commit) => {
            match commit.fmt().as_slice() {
                b"commit" => commit,
                _ => return Err(malformed_object_error(format!("Malformed commit {}", sha)))
            }
        },
        _ => Err(debug_error())?
    };

    if !commit.kvlm().contains_key("parent") {
        return Ok(())
    }

    let parents = commit.kvlm().get("parent").ok_or(malformed_object_error("No parent".to_owned()))?;
    for parent in parents {
        println!("c_{} -> c_{}", sha, parent);
        graphviz(repo, parent.clone(), seen)?;
    }
    Ok(())
}

pub fn checkout<'a>(repo: &'a Repository, tree: &Tree, path: &PathBuf) -> Result<(), Box<WitError>> {
    let mut obj: WitObject;
    let mut dest: PathBuf;
    for leaf in tree.leaves() {
        obj = read(repo, &leaf.sha())?;
        dest = PathBuf::from(path).join(&leaf.path());

        match obj {
            WitObject::BlobObject(blob) => {
                fs::write(&dest, blob.data())?;
            },
            WitObject::TreeObject(tree) => {
                fs::create_dir_all(&dest)?;
                checkout(repo, &tree, &dest)?;
            },
            _ => return Err(debug_error())?
        }
    }
    Ok(())
}