use crate::{object::Object, repository::Repository};

use self::kvlm::{KVLMExt, KVLM};

pub struct Commit<'a> {
    repo: Option<&'a Repository>,
    pub kvlm: kvlm::KVLM,
}

impl<'a> Commit<'a> {
    pub fn new(repo: Option<&'a Repository>) -> Self {
        Self {
            repo: repo,
            kvlm: KVLM::new(),
        }
    }

    pub fn kvlm(&self) -> &kvlm::KVLM {
        &self.kvlm
    }
}

impl<'a> Object for Commit<'a> {
    fn serialize(&self) -> Vec<u8> {
        self.kvlm.serialize().as_bytes().to_vec()
    }

    fn deserialize(&mut self, data: Vec<u8>) {
        self.kvlm = kvlm::KVLM::create(data, 0);
    }

    fn fmt(&self) -> Vec<u8> {
        b"commit".to_vec()
    }

    fn repo(&self) -> Option<&Repository> {
        self.repo
    }
}

pub mod kvlm {
    use std::str::from_utf8;
    use linked_hash_map::LinkedHashMap;
    use crate::object::{Find, Replace};

    pub type KVLM = LinkedHashMap<String, Vec<String>>;

    pub trait KVLMExt {
        fn create(raw: Vec<u8>, start: usize) -> KVLM;
        fn parse(&mut self, raw: Vec<u8>, start: usize) -> KVLM;
        fn serialize(&self) -> String;
    }

    impl KVLMExt for KVLM {
        fn create(raw: Vec<u8>, start: usize) -> KVLM {
            KVLM::new().parse(raw, start)
        }

        fn parse(&mut self, raw: Vec<u8>, start: usize) -> KVLM {
            let spc: Option<usize> = raw.find_some(start, b' ');
            let nl: usize = raw.find_exact(start, b'\n');

            let spc = {
                // Scope for shadowing spc and nl as isizes and extracting spc as usize
                // Scope "returns" spc as usize for use in key
                let (spc, u_spc) = match spc {
                    Some(size) => (size as isize, size),
                    None => (-1, 0)
                };
                let nl = nl as isize;
                if spc < 0 || nl < spc {
                    self.insert(
                        "".to_owned(),
                        match from_utf8(&raw[start..]) {
                            Ok(string) => vec![string.to_owned()],
                            Err(_) => panic!("Error converting {:?} to utf8", &raw[start..])
                        }
                    );
                    // Parse is meant to be called from a new KVLM object
                    return std::mem::take(self)
                }
                u_spc
            };

            let mut end = start;
            loop {
                end = match raw.find_some(end+1, b'\n') {
                    Some(new_line) => new_line,
                    None => break
                };
                if raw[end+1] != b' ' {
                    break;
                }
            }

            let key = from_utf8(&raw[start..spc]).unwrap().to_owned();
            let value = from_utf8(&raw[spc+1..end].to_vec().replace("\n ", "\n")).unwrap().to_owned();

            if self.contains_key(&key) {
                self.get_mut(&key).unwrap().push(value);
            } else {
                self.insert(key, vec![value]);
            }

            self.parse(raw, end + 1)
        }

        fn serialize(&self) -> String {
            let mut ret: String = String::new();

            for key in self.keys() {
                let key = key.clone();
                if key == String::from("") {
                    continue;
                }
                let val = &self[&key];
                for v in val {
                    ret += key.as_str();
                    ret += " ";
                    ret += &v.replace("\n", "\n ");
                    ret += "\n";
                }
            }

            ret += "\n";
            for entry in &self["\n"] {
                ret += entry.as_str();
            }
            ret
        }
    }
}

