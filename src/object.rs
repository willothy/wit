use std::io::prelude::*;
use std::fs;
use std::str::from_utf8;

use crypto::digest::Digest;
use flate2::Compression;
use flate2::write::{ZlibEncoder};
use flate2::read::ZlibDecoder;
use crypto;

use crate::error::{WitError, builder::*};
use crate::repository::Repository;

trait Find<T> {
    fn find(&self, element: T) -> Result<usize, Box<WitError>>;
}

impl<T: PartialEq + std::fmt::Debug> Find<T> for Vec<T> {
    fn find(&self, element: T) -> Result<usize, Box<WitError>> {
        self.iter().position(|el| *el == element).ok_or(
            io_error(format!("{:?} not found.", element))
        )
    }
}

pub trait Object {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: Vec<u8>);

    fn fmt(&self) -> Vec<u8>;
    fn repo(&self) -> &Repository;
}

pub fn read(repo: &Repository, sha: &str) -> Result<i32, Box<WitError>> {
    let path = Repository::repo_file(repo, vec!["objects", &sha[..2], &sha[2..]], false)?;

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

    let object_builder = match from_utf8(&fmt)? {
        "commit" => || {},
        "tree" => || {},
        "tag" => || {},
        "blob" => || {},
        _ => return Err(debug_error())
    };
    todo!()
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
            Repository::repo_file(obj.repo(), vec!["objects"], actually_write)?, // Path
            encoder.finish()? // Data
        )?;
    }

    Ok(sha)
}