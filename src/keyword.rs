use openssl::sha::sha256;
use std::{fmt, mem};

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

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn hash(&self) -> &Key {
        &self.hash
    }

    /// A pretty form of the hash
    pub fn pretty_hash(&self) -> String {
        format!(
            "{:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x}",
            self.slice_int::<u32>(0x00).unwrap(),
            self.slice_int::<u32>(0x04).unwrap(),
            self.slice_int::<u32>(0x08).unwrap(),
            self.slice_int::<u32>(0x0c).unwrap(),
            self.slice_int::<u32>(0x10).unwrap(),
            self.slice_int::<u32>(0x14).unwrap(),
            self.slice_int::<u32>(0x18).unwrap(),
            self.slice_int::<u32>(0x1c).unwrap(),
        )
    }

    // Slicing and int helper functions
    /// Returns a slice at index of length length from the hash
    fn get_slice(&self, index: usize, length: usize) -> Option<&[u8]> {
        if index + length > self.hash.len() {
            return None;
        }
        return Some(&self.hash[index..index+length]);
    }
    /// Return a int, read as an unsigned, big-endian integer starting at offset
    pub fn slice_int<T: FromByteSlice>(&self, offset: usize) -> Option<T> {
        if let Some(slice) = self.get_slice(offset, mem::size_of::<T>()) {
            return Some(T::from_bytes(slice));
        } else {
            return None;
        }
    }

    // Slice shorthands
    /// Lower 16 bytes of the hash interpreted as a big-endian, unsigned int
    pub fn lower_128(&self) -> u128 {
        self.slice_int(0).unwrap()
    }
    /// Lower 8 bytes of the hash interpreted as a big-endian, unsigned int
    pub fn lower_64(&self) -> u64 {
        self.slice_int(0).unwrap()
    }
    /// Lower 4 bytes of the hash interpreted as a big-endian, unsigned int
    pub fn lower_32(&self) -> u32 {
        self.slice_int(0).unwrap()
    }
    /// Lower 2 bytes of the hash interpreted as a big-endian, unsigned int
    pub fn lower_16(&self) -> u16 {
        self.slice_int(0).unwrap()
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Keyword('{}', {})", self.text, self.pretty_hash())
    }
}

/// Able to convert from a slice of bytes
pub trait FromByteSlice {
    /// Converts the slice into the type, assuming both unsigned-ness and big-endian-ness
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromByteSlice for u8 {
    fn from_bytes(bytes: &[u8]) -> u8 {
        return bytes[0];
    }
}
impl FromByteSlice for u16 {
    fn from_bytes(bytes: &[u8]) -> u16 {
        u16::from_be_bytes(bytes.try_into().unwrap())
    }
}
impl FromByteSlice for u32 {
    fn from_bytes(bytes: &[u8]) -> u32 {
        u32::from_be_bytes(bytes.try_into().unwrap())
    }
}
impl FromByteSlice for u64 {
    fn from_bytes(bytes: &[u8]) -> u64 {
        u64::from_be_bytes(bytes.try_into().unwrap())
    }
}
impl FromByteSlice for u128 {
    fn from_bytes(bytes: &[u8]) -> u128 {
        u128::from_be_bytes(bytes.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn disp_hash(text: String) {
        let k = Keyword::new(text);
        println!("{} -> {:?}", k.text, k.hash);
    }

    /// Returns a keyword with a specific pattern in the fingerprint, useful
    /// for testing slicing
    fn fingerprint_hash() -> Keyword {
        let mut k = Keyword::new("a".to_string());
        k.hash = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
        ];
        return k;
    }

    #[test]
    fn hash_simple() {
        disp_hash("test".to_string());
        disp_hash("test1".to_string());
    }

   #[test]
    fn sha_test() {
        let k = Keyword::new("test".to_string());
        let expected = [
            0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65,
            0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a, 0xd0, 0x15,
            0xa3, 0xbf, 0x4f, 0x1b, 0x2b, 0x0b, 0x82, 0x2c,
            0xd1, 0x5d, 0x6c, 0x15, 0xb0, 0xf0, 0x0a, 0x08
        ];
        assert_eq!(k.hash, expected);
    }

    #[test]
    fn slice_u8() {
        let k = fingerprint_hash();
        for i in 0..32 {
            assert_eq!(i as u8, k.hash[i]);
        }
    }

    #[test]
    fn slice_u16() {
        let k = fingerprint_hash();
        assert_eq!(0x0001, k.slice_int::<u16>(0x00).unwrap());
        assert_eq!(0x0102, k.slice_int::<u16>(0x01).unwrap());
        assert_eq!(0x0203, k.slice_int::<u16>(0x02).unwrap());
        assert_eq!(0x0304, k.slice_int::<u16>(0x03).unwrap());
        assert_eq!(0x0405, k.slice_int::<u16>(0x04).unwrap());
        assert_eq!(0x0506, k.slice_int::<u16>(0x05).unwrap());
        assert_eq!(0x0607, k.slice_int::<u16>(0x06).unwrap());
        assert_eq!(0x0708, k.slice_int::<u16>(0x07).unwrap());
        assert_eq!(0x0809, k.slice_int::<u16>(0x08).unwrap());
        assert_eq!(0x090a, k.slice_int::<u16>(0x09).unwrap());
        assert_eq!(0x0a0b, k.slice_int::<u16>(0x0a).unwrap());
        assert_eq!(0x0b0c, k.slice_int::<u16>(0x0b).unwrap());
        assert_eq!(0x0c0d, k.slice_int::<u16>(0x0c).unwrap());
        assert_eq!(0x0d0e, k.slice_int::<u16>(0x0d).unwrap());
        assert_eq!(0x0e0f, k.slice_int::<u16>(0x0e).unwrap());
        assert_eq!(0x0f10, k.slice_int::<u16>(0x0f).unwrap());
        assert_eq!(0x1011, k.slice_int::<u16>(0x10).unwrap());
        assert_eq!(0x1112, k.slice_int::<u16>(0x11).unwrap());
        assert_eq!(0x1213, k.slice_int::<u16>(0x12).unwrap());
        assert_eq!(0x1314, k.slice_int::<u16>(0x13).unwrap());
        assert_eq!(0x1415, k.slice_int::<u16>(0x14).unwrap());
        assert_eq!(0x1516, k.slice_int::<u16>(0x15).unwrap());
        assert_eq!(0x1617, k.slice_int::<u16>(0x16).unwrap());
        assert_eq!(0x1718, k.slice_int::<u16>(0x17).unwrap());
        assert_eq!(0x1819, k.slice_int::<u16>(0x18).unwrap());
        assert_eq!(0x191a, k.slice_int::<u16>(0x19).unwrap());
        assert_eq!(0x1a1b, k.slice_int::<u16>(0x1a).unwrap());
        assert_eq!(0x1b1c, k.slice_int::<u16>(0x1b).unwrap());
        assert_eq!(0x1c1d, k.slice_int::<u16>(0x1c).unwrap());
        assert_eq!(0x1d1e, k.slice_int::<u16>(0x1d).unwrap());
        assert_eq!(0x1e1f, k.slice_int::<u16>(0x1e).unwrap());
    }

    #[test]
    fn slice_u32() {
        let k = fingerprint_hash();
        assert_eq!(0x00010203, k.slice_int::<u32>(0x00).unwrap());
        assert_eq!(0x01020304, k.slice_int::<u32>(0x01).unwrap());
        assert_eq!(0x02030405, k.slice_int::<u32>(0x02).unwrap());
        assert_eq!(0x03040506, k.slice_int::<u32>(0x03).unwrap());
        assert_eq!(0x04050607, k.slice_int::<u32>(0x04).unwrap());
        assert_eq!(0x05060708, k.slice_int::<u32>(0x05).unwrap());
        assert_eq!(0x06070809, k.slice_int::<u32>(0x06).unwrap());
        assert_eq!(0x0708090a, k.slice_int::<u32>(0x07).unwrap());
        assert_eq!(0x08090a0b, k.slice_int::<u32>(0x08).unwrap());
        assert_eq!(0x090a0b0c, k.slice_int::<u32>(0x09).unwrap());
        assert_eq!(0x0a0b0c0d, k.slice_int::<u32>(0x0a).unwrap());
        assert_eq!(0x0b0c0d0e, k.slice_int::<u32>(0x0b).unwrap());
        assert_eq!(0x0c0d0e0f, k.slice_int::<u32>(0x0c).unwrap());
        assert_eq!(0x0d0e0f10, k.slice_int::<u32>(0x0d).unwrap());
        assert_eq!(0x0e0f1011, k.slice_int::<u32>(0x0e).unwrap());
        assert_eq!(0x0f101112, k.slice_int::<u32>(0x0f).unwrap());
        assert_eq!(0x10111213, k.slice_int::<u32>(0x10).unwrap());
        assert_eq!(0x11121314, k.slice_int::<u32>(0x11).unwrap());
        assert_eq!(0x12131415, k.slice_int::<u32>(0x12).unwrap());
        assert_eq!(0x13141516, k.slice_int::<u32>(0x13).unwrap());
        assert_eq!(0x14151617, k.slice_int::<u32>(0x14).unwrap());
        assert_eq!(0x15161718, k.slice_int::<u32>(0x15).unwrap());
        assert_eq!(0x16171819, k.slice_int::<u32>(0x16).unwrap());
        assert_eq!(0x1718191a, k.slice_int::<u32>(0x17).unwrap());
        assert_eq!(0x18191a1b, k.slice_int::<u32>(0x18).unwrap());
        assert_eq!(0x191a1b1c, k.slice_int::<u32>(0x19).unwrap());
        assert_eq!(0x1a1b1c1d, k.slice_int::<u32>(0x1a).unwrap());
        assert_eq!(0x1b1c1d1e, k.slice_int::<u32>(0x1b).unwrap());
        assert_eq!(0x1c1d1e1f, k.slice_int::<u32>(0x1c).unwrap());
    }

    #[test]
    fn slice_u64() {
        let k = fingerprint_hash();
        assert_eq!(0x0001020304050607, k.slice_int::<u64>(0x00).unwrap());
        assert_eq!(0x0102030405060708, k.slice_int::<u64>(0x01).unwrap());
        assert_eq!(0x0203040506070809, k.slice_int::<u64>(0x02).unwrap());
        assert_eq!(0x030405060708090a, k.slice_int::<u64>(0x03).unwrap());
        assert_eq!(0x0405060708090a0b, k.slice_int::<u64>(0x04).unwrap());
        assert_eq!(0x05060708090a0b0c, k.slice_int::<u64>(0x05).unwrap());
        assert_eq!(0x060708090a0b0c0d, k.slice_int::<u64>(0x06).unwrap());
        assert_eq!(0x0708090a0b0c0d0e, k.slice_int::<u64>(0x07).unwrap());
        assert_eq!(0x08090a0b0c0d0e0f, k.slice_int::<u64>(0x08).unwrap());
        assert_eq!(0x090a0b0c0d0e0f10, k.slice_int::<u64>(0x09).unwrap());
        assert_eq!(0x0a0b0c0d0e0f1011, k.slice_int::<u64>(0x0a).unwrap());
        assert_eq!(0x0b0c0d0e0f101112, k.slice_int::<u64>(0x0b).unwrap());
        assert_eq!(0x0c0d0e0f10111213, k.slice_int::<u64>(0x0c).unwrap());
        assert_eq!(0x0d0e0f1011121314, k.slice_int::<u64>(0x0d).unwrap());
        assert_eq!(0x0e0f101112131415, k.slice_int::<u64>(0x0e).unwrap());
        assert_eq!(0x0f10111213141516, k.slice_int::<u64>(0x0f).unwrap());
        assert_eq!(0x1011121314151617, k.slice_int::<u64>(0x10).unwrap());
        assert_eq!(0x1112131415161718, k.slice_int::<u64>(0x11).unwrap());
        assert_eq!(0x1213141516171819, k.slice_int::<u64>(0x12).unwrap());
        assert_eq!(0x131415161718191a, k.slice_int::<u64>(0x13).unwrap());
        assert_eq!(0x1415161718191a1b, k.slice_int::<u64>(0x14).unwrap());
        assert_eq!(0x15161718191a1b1c, k.slice_int::<u64>(0x15).unwrap());
        assert_eq!(0x161718191a1b1c1d, k.slice_int::<u64>(0x16).unwrap());
        assert_eq!(0x1718191a1b1c1d1e, k.slice_int::<u64>(0x17).unwrap());
        assert_eq!(0x18191a1b1c1d1e1f, k.slice_int::<u64>(0x18).unwrap());
    }

    #[test]
    fn slice_u128() {
        let k = fingerprint_hash();
        assert_eq!(0x000102030405060708090a0b0c0d0e0f, k.slice_int::<u128>(0x00).unwrap());
        assert_eq!(0x0102030405060708090a0b0c0d0e0f10, k.slice_int::<u128>(0x01).unwrap());
        assert_eq!(0x02030405060708090a0b0c0d0e0f1011, k.slice_int::<u128>(0x02).unwrap());
        assert_eq!(0x030405060708090a0b0c0d0e0f101112, k.slice_int::<u128>(0x03).unwrap());
        assert_eq!(0x0405060708090a0b0c0d0e0f10111213, k.slice_int::<u128>(0x04).unwrap());
        assert_eq!(0x05060708090a0b0c0d0e0f1011121314, k.slice_int::<u128>(0x05).unwrap());
        assert_eq!(0x060708090a0b0c0d0e0f101112131415, k.slice_int::<u128>(0x06).unwrap());
        assert_eq!(0x0708090a0b0c0d0e0f10111213141516, k.slice_int::<u128>(0x07).unwrap());
        assert_eq!(0x08090a0b0c0d0e0f1011121314151617, k.slice_int::<u128>(0x08).unwrap());
        assert_eq!(0x090a0b0c0d0e0f101112131415161718, k.slice_int::<u128>(0x09).unwrap());
        assert_eq!(0x0a0b0c0d0e0f10111213141516171819, k.slice_int::<u128>(0x0a).unwrap());
        assert_eq!(0x0b0c0d0e0f101112131415161718191a, k.slice_int::<u128>(0x0b).unwrap());
        assert_eq!(0x0c0d0e0f101112131415161718191a1b, k.slice_int::<u128>(0x0c).unwrap());
        assert_eq!(0x0d0e0f101112131415161718191a1b1c, k.slice_int::<u128>(0x0d).unwrap());
        assert_eq!(0x0e0f101112131415161718191a1b1c1d, k.slice_int::<u128>(0x0e).unwrap());
        assert_eq!(0x0f101112131415161718191a1b1c1d1e, k.slice_int::<u128>(0x0f).unwrap());
        assert_eq!(0x101112131415161718191a1b1c1d1e1f, k.slice_int::<u128>(0x10).unwrap());
    }

    #[test]
    fn out_of_bounds_slice() {
        let k = Keyword::new("test".to_string());
        assert_eq!(k.slice_int::<u8>(0x20), None);
        assert_eq!(k.slice_int::<u16>(0x1f), None);
        assert_eq!(k.slice_int::<u32>(0x1d), None);
        assert_eq!(k.slice_int::<u64>(0x19), None);
        assert_eq!(k.slice_int::<u128>(0x11), None);
    }
}

