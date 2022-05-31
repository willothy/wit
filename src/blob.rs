use crate::{object::Object, repository::Repository};


pub struct Blob<'a> {
    repo: Option<&'a Repository>,
    blobdata: Vec<u8>,
}

impl<'a> Blob<'a> {
    pub fn new(repo: Option<&'a Repository>, data: Vec<u8>) -> Self {
        Self {
            repo: repo,
            blobdata: data,
        }
    }
}

impl<'a> Object for Blob<'a> {
    fn serialize(&self) -> Vec<u8> {
        self.blobdata.clone()
    }

    fn deserialize(&mut self, data: Vec<u8>) {
        self.blobdata = data;
    }

    fn fmt(&self) -> Vec<u8> {
        b"blob".to_vec()
    }

    fn repo(&self) -> Option<&'a Repository> {
        self.repo
    }
}