/// An internal address in the filesystem
pub struct Address(u64);

impl Address {
    pub fn byte_index(&self) -> u8 {
        (self.0 & 0b0111) as u8
    }
}

impl From<u64> for Address {
    fn from(n: u64) -> Address {
        Address(n)
    }
}
