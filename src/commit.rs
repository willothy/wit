use crate::{
    object::Object, 
    repository::Repository, 
    error::WitError
};
use crate::kvlm::{ KVLMExt, KVLM };

pub struct Commit<'a> {
    repo: Option<&'a Repository>,
    kvlm: KVLM,
}

impl<'a> Commit<'a> {
    pub fn new(repo: Option<&'a Repository>) -> Self {
        Self {
            repo: repo,
            kvlm: KVLM::new(),
        }
    }

    pub fn kvlm(&self) -> &KVLM {
        &self.kvlm
    }
}

impl<'a> Object for Commit<'a> {
    fn serialize(&self) -> Result<Vec<u8>, Box<WitError>> {
        Ok(self.kvlm.serialize().as_bytes().to_vec())
    }

    fn deserialize(&mut self, data: Vec<u8>) -> Result<(), Box<WitError>> {
        self.kvlm = KVLM::create(data, 0);
        Ok(())
    }

    fn fmt(&self) -> Vec<u8> {
        b"commit".to_vec()
    }

    fn repo(&self) -> Option<&Repository> {
        self.repo
    }
}
