use crate::object::Object;


pub struct Commit {
    kvlm: kvlm::KVLM,
}

impl Commit {
    
}

impl Object for Commit {
    fn serialize(&self) -> Vec<u8> {
        kvlm::serialize(self.kvlm.clone()).as_bytes().to_vec()
    }

    fn deserialize(&mut self, data: Vec<u8>) {
        self.kvlm = std::mem::take(kvlm::parse(data, 0, &mut kvlm::KVLM::new()));
    }

    fn fmt(&self) -> Vec<u8> {
        b"commit".to_vec()
    }

    fn repo(&self) -> Option<crate::repository::Repository> {
        todo!()
    }
}

pub mod kvlm {
    use std::str::from_utf8;
    use linked_hash_map::LinkedHashMap;
    use crate::object::{Find, Replace};

    pub type KVLM = LinkedHashMap<String, Vec<String>>;

    pub fn parse(raw: Vec<u8>, start: usize, dct: &mut KVLM) -> &mut KVLM {
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
                dct.insert(
                    "".to_owned(),
                    match from_utf8(&raw[start..]) {
                        Ok(string) => vec![string.to_owned()],
                        Err(_) => panic!("Error converting {:?} to utf8", &raw[start..])
                    }
                );
                return dct
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

        if dct.contains_key(&key) {
            dct.get_mut(&key).unwrap().push(value);
        } else {
            dct.insert(key, vec![value]);
        }

        self::parse(raw, end + 1, dct)
    }

    pub fn serialize(kvlm: KVLM) -> String {
        let mut ret: String = String::new();

        for key in kvlm.keys() {
            let key = key.clone();
            if key == String::from("") {
                continue;
            }
            let val = &kvlm[&key];
            for v in val {
                ret += key.as_str();
                ret += " ";
                ret += &v.replace("\n", "\n ");
                ret += "\n";
            }
        }

        ret += "\n";
        for entry in &kvlm["\n"] {
            ret += entry.as_str();
        }

        ret
    }
}

