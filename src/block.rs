use std::fmt;
use std::mem::{self, size_of};
use std::ops::{Deref, DerefMut};
use crate::Key;

const BLOCK_SIZE: usize = 1024;
const BLOCK_DATA_SIZE: usize = BLOCK_SIZE - size_of::<BlockHeader>();

#[repr(align(16))]
pub struct EncryptedBlock([u8; BLOCK_SIZE]);

impl EncryptedBlock {
    pub fn new() -> EncryptedBlock {
        EncryptedBlock([0; BLOCK_SIZE]) // TODO add uninit version
    }
    pub fn decrypt(self, key: Key) -> Block {
        todo!()
    }
}

impl Deref for EncryptedBlock {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EncryptedBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for EncryptedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x?}", self.0)
    }
}

#[repr(C)]
pub struct BlockHeader {
    next_sector_id: u64,
    /// Checksums are calculated for data using a 64bit CRC code
    checksum: u64,
}

#[repr(C)]
pub struct Block {
    header: BlockHeader,
    data: [u8; BLOCK_DATA_SIZE],
}

impl Block {
    fn new() -> Block {
        let header = BlockHeader {
            next_sector_id: 0,
            checksum: 0,
        };
        let mut block = Block {
            header: header,
            data: [0; BLOCK_DATA_SIZE],
        };
        block.calculate_checksum();
        return block;
    }

    /// Calculate the checksum for the block
    /// ECMA-182 based CRC64
    fn calculate_checksum(&self) -> u64 {
        todo!()
    }

    fn update_checksum(&mut self) {
        self.header.checksum = self.calculate_checksum()
    }

    /// Tests if the checksum is valid
    pub fn validate_checksum(&self) -> bool {
        self.calculate_checksum() == self.header.checksum
    }

    pub fn encrypt(self) -> EncryptedBlock {
        todo!()
    }

    pub const fn size() -> usize { BLOCK_SIZE }
}

