use std::path::PathBuf;

use crate::{object::{Find, Object}, error::{WitError, builder::*}};

pub struct Tree {
    leaves: Vec<Leaf>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            leaves: Vec::new(),
        }
    }

    pub fn from(raw: &Vec<u8>) -> Result<Self, Box<WitError>> {
        let mut tree = Self::new();
        let mut pos = 0;

        while pos < raw.len() {
            let (end, leaf) = Self::parse_one(raw, pos)?;
            tree.add_leaf(leaf);
            pos = end;
        }

        Ok(tree)
    }

    pub fn add_leaf(&mut self, leaf: Leaf) {
        self.leaves.push(leaf);
    }

    pub fn leaves(&self) -> &Vec<Leaf> {
        &self.leaves
    }

    pub fn parse_one(raw: &Vec<u8>, start: usize) -> Result<(usize, Leaf), Box<WitError>> {
        let mode_end = raw.find_from(b'\n', start)?;
        if mode_end - start != 5 && mode_end - start != 6 {
            return Err(mode_err(mode_end - start));
        }
        let mode = String::from_utf8(raw[start..mode_end].to_vec())?;

        let path_end = raw.find_from(b'\x00', mode_end)?;
        let path = PathBuf::from(String::from_utf8(raw[mode_end+1..path_end].to_vec())?);

        let sha = Self::sha(&raw[path_end+1..path_end+21].to_vec());

        Ok((path_end + 21, Leaf::new(mode, path, sha)))
    }

    fn sha(vec: &Vec<u8>) -> String {
        let mut sha = 0;
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        vec.iter().enumerate().for_each(|(i, x)| {
            if i % 4 == 0 {
                // https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
                sha +=
                ((buf[0] as u32) << 24) +
                ((buf[1] as u32) << 16) +
                ((buf[2] as u32) <<  8) +
                ((buf[3] as u32) <<  0);
            }
            buf[i % 4] = *x;
        });
        format!("{:x}", &sha)
    }
}

impl Object for Tree {
    fn serialize(&self) -> Result<Vec<u8>, Box<WitError>> {
        let mut bytes = Vec::<u8>::new();
        bytes.extend(b"");

        for leaf in self.leaves() {
            bytes.extend(leaf.mode().as_bytes());
            bytes.push(b' ');
            bytes.extend(leaf.path().to_str().ok_or(
                utf8_err(format!("Could not convert {} to str.", leaf.path().to_str().unwrap_or("")))
            )?.as_bytes());
            bytes.push(b'\x00');
            bytes.extend(
                u32::from_str_radix(leaf.sha(), 16)?.to_be_bytes().to_vec()
            );
        }

        Ok(bytes)
    }

    fn deserialize(&mut self, data: Vec<u8>) -> Result<(), Box<WitError>> {
        let mut pos = 0;

        while pos < data.len() {
            let (end, leaf) = Self::parse_one(&data, pos)?;
            self.add_leaf(leaf);
            pos = end;
        }
        Ok(())
    }

    fn fmt(&self) -> Vec<u8> {
        todo!()
    }

    fn repo(&self) -> Option<&crate::repository::Repository> {
        todo!()
    }
}

pub struct Leaf {
    mode: String,
    path: PathBuf,
    sha: String,
}

impl Leaf {
    pub fn new(mode: String, path: PathBuf, sha: String) -> Leaf {
        Leaf {
            mode,
            path,
            sha,
        }
    }

    pub fn mode(&self) -> &str {
        &self.mode
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn sha(&self) -> &str {
        &self.sha
    }
}

