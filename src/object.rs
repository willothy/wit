use std::io::prelude::*;
use std::fs;
use std::str::from_utf8;

use crypto::digest::Digest;
use flate2::Compression;
use flate2::write::{ZlibEncoder};
use flate2::read::ZlibDecoder;
use crypto;

use crate::blob::Blob;
use crate::error::{WitError, builder::*};
use crate::repository::Repository;

pub trait Find<T> {
    fn find(&self, element: T) -> Result<usize, Box<WitError>>;

    fn find_some(&self, start: usize, element: T) -> Option<usize> {None}

    fn find_signed(&self, start: usize, element: T) -> isize {0}

    fn find_exact(&self, start: usize, element: T) -> usize {0}
}

pub trait Replace {
    fn replace(&mut self, from: &str, to: &str) -> Vec<u8>;
}

impl Replace for Vec<u8> {
    fn replace(&mut self, from: &str, to: &str) -> Vec<u8> {
        from_utf8(self).unwrap().replace(from, to).as_bytes().to_vec()
    }
}

impl<T: PartialEq + std::fmt::Debug + Default> Find<T> for Vec<T> {
    fn find(&self, element: T) -> Result<usize, Box<WitError>> {
        self.iter().position(|el| *el == element).ok_or(
            io_error(format!("{:?} not found.", element))
        )
    }

    fn find_signed(&self, start: usize, element: T) -> isize {
        match self.iter().skip(start).position(|el| *el == element) {
            Some(idx) => idx as isize,
            None => -1
        }
    }

    fn find_exact(&self, start: usize, element: T) -> usize {
        self.iter().skip(start).position(|el| *el == element).unwrap()
    }

    fn find_some(&self, start: usize, element: T) -> Option<usize> {
        self.iter().skip(start).position(|el| *el == element)
    }
}

pub trait Object {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: Vec<u8>);

    fn fmt(&self) -> Vec<u8>;
    fn repo(&self) -> Option<Repository>;
}



pub fn read(repo: Repository, sha: &str) -> Result<Box<dyn Object>, Box<WitError>> {
    let path = Repository::repo_file(&repo, vec!["objects", &sha[..2], &sha[2..]], false)?;

    let raw = fs::read(path)?;
    let mut decoded = Vec::<u8>::new();
    ZlibDecoder::new(&raw[..]).read_exact(&mut decoded)?;

    let x = decoded.find(b' ') ?;
    let fmt = &decoded[..x];

    let y = decoded[x..].to_vec().find(b'\x00')?;

    let size = from_utf8(&decoded[x..y])?.parse::<usize>()?;
    if size != decoded.len() - y - 1 {
        return Err(malformed_object_error(format!("Malformed object {}: bad length", sha)))
    }

    build(from_utf8(&fmt)?, Some(repo), raw[y+1..].to_vec())
}


pub fn find(/*repo: Repository, */name: &str, fmt: &str, follow: bool) -> String {
    String::from(name)
}

pub fn write(obj: Box<dyn Object>, actually_write: bool) -> Result<String, Box<WitError>> {
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
            Repository::repo_file(&obj.repo().ok_or(debug_error())?, vec!["objects"], actually_write)?, // Path
            encoder.finish()? // Data
        )?;
    }

    Ok(sha)
}

fn build<'a>(fmt: &str, repo: Option<Repository>, data: Vec<u8>) -> Result<Box<dyn Object>, Box<WitError>> {
    let builder = match fmt {
        "commit" => Blob::new, // TODO: replace
        "tree" => Blob::new, // TODO: replace
        "tag" => Blob::new, // TODO: replace
        "blob" => Blob::new,
        _ => return Err(debug_error())
    };
    Ok(Box::new(builder(repo, data)))
}

pub fn hash(fd: &str, fmt: &str, repo: Option<Repository>) -> Result<String, Box<WitError>>{
    let data = fs::read(fd)?;
    let builder = match fmt {
        "commit" => Blob::new, // TODO: replace
        "tree" => Blob::new, // TODO: replace
        "tag" => Blob::new, // TODO: replace
        "blob" => Blob::new,
        _ => return Err(debug_error())
    };

    let is_none = repo.is_none();
    let obj = builder(repo, data);

    write(Box::new(obj), is_none)
}