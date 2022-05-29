use crate::{object::Object, repository::Repository};


pub struct Blob {
    repo: Option<Repository>,
    blobdata: Vec<u8>,
}

impl Blob {
    pub fn new(repo: Option<Repository>, data: Vec<u8>) -> Self {
        Self {
            repo: repo,
            blobdata: data,
        }
    }
}

impl Object for Blob {
    fn serialize(&self) -> Vec<u8> {
        self.blobdata.clone()
    }

    fn deserialize(&mut self, data: Vec<u8>) {
        self.blobdata = data;
    }

    fn fmt(&self) -> Vec<u8> {
        b"blob".to_vec()
    }

    fn repo(&self) -> Option<Repository> {
        self.repo.clone()
    }
}