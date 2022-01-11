use openssl::sha::sha256;

use crate::{Block, BLOCK_SIZE};

/// The hashed, secure key
pub type Key = [u8; 32];

/// A combination of the text and hash of a given key. Needed for decryption
pub struct Keyword {
    text: String,
    hash: Key,
}

/// Key the given string
fn hash(text: &str) -> Key {
    sha256(text.as_bytes())
}

impl Keyword {
    /// Create a new keyword from a given string
    pub fn new(text: String) -> Keyword {
        Keyword {
            hash: hash(&text),
            text: text,
        }
    }
    // TODO make this give a lifetimed reference
    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn hash(&self) -> &Key {
        &self.hash
    }

    /// A pretty form of the hash
    pub fn pretty_hash(&self) -> String {
        format!(
            "{:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x}",
            u32::from_be_bytes(self.hash[0..4].try_into().unwrap()),
            u32::from_be_bytes(self.hash[4..8].try_into().unwrap()),
            u32::from_be_bytes(self.hash[8..12].try_into().unwrap()),
            u32::from_be_bytes(self.hash[12..16].try_into().unwrap()),
            u32::from_be_bytes(self.hash[16..20].try_into().unwrap()),
            u32::from_be_bytes(self.hash[20..24].try_into().unwrap()),
            u32::from_be_bytes(self.hash[24..28].try_into().unwrap()),
            u32::from_be_bytes(self.hash[28..32].try_into().unwrap()),
        )
    }

    pub fn id(&self) -> u64 {
        u64::from_be_bytes(self.hash[24..32].try_into().unwrap())
    }

    pub fn xor_block(&self, block: &mut Block) {
        let mut i = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sector;

    use std::mem;

    /// Verify that a hash is longer than a Head sector pointer. If it isn't,
    /// then certain types of probablity attacks are viable
    #[test]
    fn hash_size_xor() {
        assert!(mem::size_of::<Key>() >= mem::size_of::<sector::Header>());
    }

    fn disp_hash(text: String) {
        let k = Keyword::new(text);
        println!("{} -> {:?}", k.text, k.hash);
    }

    #[test]
    fn hash_simple() {
        disp_hash("test".to_string());
        disp_hash("test1".to_string());
    }

    #[test]
    fn sha_test() {
        let k = Keyword::new("test".to_string());
        assert_eq!(k.id(), 0xd15d6c15b0f00a08);
    }
}
