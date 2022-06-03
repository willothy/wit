use crate::{
    object::Object, 
    repository::Repository, 
    error::WitError
};


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

    pub fn data(&self) -> &Vec<u8> {
        &self.blobdata
    }
}

impl<'a> Object for Blob<'a> {
    fn serialize(&self) -> Result<Vec<u8>, Box<WitError>> {
        Ok(self.blobdata.clone())
    }

    fn deserialize(&mut self, data: Vec<u8>) -> Result<(), Box<WitError>> {
        self.blobdata = data;
        Ok(())
    }

    fn fmt(&self) -> Vec<u8> {
        b"blob".to_vec()
    }

    fn repo(&self) -> Option<&'a Repository> {
        self.repo
    }
}