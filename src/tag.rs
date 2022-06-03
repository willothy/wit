use crate::{
    object::{Object, WitObject::*, self},
    repository::Repository,
    error::WitError,
    kvlm::{ KVLMExt, KVLM }, refs
};

pub struct Tag<'a> {
    repo: Option<&'a Repository>,
    kvlm: KVLM,
}

impl<'a> Tag<'a> {
    pub fn new(repo: Option<&'a Repository>) -> Self {
        Self {
            repo,
            kvlm: KVLM::new(),
        }
    }

    pub fn kvlm(&mut self) -> &mut KVLM {
        &mut self.kvlm
    }
}

impl<'a> Object for Tag<'a> {
    fn serialize(&self) -> Vec<u8> {
        self.kvlm.serialize().as_bytes().to_vec()
    }

    fn deserialize(&mut self, data: Vec<u8>) -> Result<(), Box<WitError>> {
        self.kvlm = KVLM::create(data, 0);
        Ok(())
    }

    fn fmt(&self) -> Vec<u8> {
        b"tag".to_vec()
    }

    fn repo(&self) -> Option<&Repository> {
        self.repo
    }
}

pub fn create(repo: &Repository, name: &str, reference: &str, create_object: bool) -> Result<(), Box<WitError>> {
    let sha = object::find(repo, reference, None, true)?;

    if create_object {
        let mut tag = Tag::new(Some(repo));
        let kvlm = tag.kvlm();
        kvlm.insert("object".to_owned(), vec![sha]);
        kvlm.insert("type".to_owned(), vec!["commit".to_owned()]);
        kvlm.insert("tag".to_owned(), vec![name.to_owned()]);
        kvlm.insert("tagger".to_owned(), vec![format!("{} <{}>", "Will Hopkins", "willothyh@gmail.com")]);
        // Commit message
        kvlm.insert("".to_owned(), vec![format!("{}", "Created by wit")]);

        // Create the tag object
        let tag_sha = object::write(TagObject(tag), true)?;
        // Create the ref
        refs::create(&repo, "tags/".to_owned() + name, tag_sha)
    } else {
        // Create lightweight tag
        refs::create(&repo, "tags/".to_owned() + name, sha)
    }
}