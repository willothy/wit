use std::path::PathBuf;

use crate::error::WitError;
use crate::object::Find;
use crate::util::hex;


pub struct Index {
    header: [u8; 12],
    signature: [u8; 4],
    version: String,
    nindex: u32,

    entries: Vec<IndexEntry>,
}

impl Index {
    pub fn open(path: &PathBuf) -> Result<Index, Box<WitError>> {
        let raw = std::fs::read(path)?;
        Self::from(raw)
    }

    pub fn from(raw: Vec<u8>) -> Result<Index, Box<WitError>> {
        let header: [u8; 12] = raw[..12].try_into()?;
        let nindex = u32::from_be_bytes(header[8..12].try_into()?);
        
        let content = raw[12..].to_vec();
        let mut entries = Vec::<IndexEntry>::new();
        let mut curs: usize = 0;
        for i in 0..nindex {
            let null_idx = content.find_exact(b'\x00', curs+62);
            entries.push(
                IndexEntry::from(
                    content[curs..null_idx].to_vec(),
                )?
            );
            curs = null_idx + 1;
        }

        Ok(Index {
            header,
            signature: header[..4].try_into()?,
            version: hex(&header[4..8].to_owned()),
            nindex,
            entries,
        })
    }
}

pub struct IndexEntry {
    // The last time a file's metadata changed.
    // (seconds, nanoseconds)
    ctime: (u32, u32),

    // The last time a file's metadata changed.
    // (seconds, nanoseconds)
    mtime: (u32, u32),

    //The ID of device containing the file.
    dev: u32,

    //The inode number of the file.
    ino: u32,

    // The object type of the file.
    // First 4 bits: (b1000 = regular file, b1010 = symlink, b1110 = gitlink)
    // 3 bits unused
    // 9 bits: The object permissions, an integer
    // Only 0755 and 0644 are valid for regular files.
    // Symbolic links and gitlinks have value 0 in this field.
    mode: u32,

    // The User ID of owner
    uid: u32,

    // The Group ID of owner
    gid: u32,

    // The file size, in bytes
    size: u32,

    // The object's hash as a hex string
    hash: String,

    // 4 bits for flags
    //flag_assume_valid: bool, // 1 bit
    //flag_extended: bool, // 1 bit
    //flag_stage: (bool, bool), // 2 bits

    // Length of name if < 0xFFF, -1 otherwise
    //flag_name_length: bool, // 12 bits
    flags: u16, // TODO: Bit manipulation lib for u2 and u12

    // The name of the file
    file_path: String
}

impl IndexEntry {
    pub fn from(raw: Vec<u8>) -> Result<Self, Box<WitError>> {
        Ok(Self {
            ctime: (
                u32::from_be_bytes(raw[0..4].try_into()?),
                u32::from_be_bytes(raw[4..8].try_into()?),
            ),
            mtime: (
                u32::from_be_bytes(raw[8..12].try_into()?),
                u32::from_be_bytes(raw[12..16].try_into()?),
            ),
            dev: u32::from_be_bytes(raw[16..20].try_into()?),
            ino: u32::from_be_bytes(raw[20..24].try_into()?),
            mode: u32::from_be_bytes(raw[24..28].try_into()?),
            uid: u32::from_be_bytes(raw[28..32].try_into()?),
            gid: u32::from_be_bytes(raw[32..36].try_into()?),
            size: u32::from_be_bytes(raw[36..40].try_into()?),
            hash: hex(&raw[40..60].to_vec()),
            flags: u16::from_be_bytes(raw[60..62].try_into()?),
            file_path: String::from_utf8(raw[62..].to_vec())?,
        })
    }
}