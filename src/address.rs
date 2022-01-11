

/// An internal address in the filesystem
pub struct Address(u64);

impl Address {
    /// First 4
    pub fn super_block_index(&self) -> u8 {

    }

    pub fn

    pub fn byte_index(&self) -> u8 {
        self.0 & 0b0111
    }

    pub fn 
}
