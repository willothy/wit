use std::{collections::HashMap, hash::Hash, str::from_utf8};
use crate::object::{Find, Replace};

use linked_hash_map::LinkedHashMap;

pub fn kvlm_parse(raw: Vec<u8>, start: usize, dct: &mut LinkedHashMap<String, Vec<String>>) -> &LinkedHashMap<String, Vec<String>> {
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

    kvlm_parse(raw, end + 1, dct)
}