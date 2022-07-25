use crate::Key;
use std::mem::{self, size_of};

pub const BLOCK_SIZE: usize = 1024;
const BLOCK_DATA_SIZE: usize = BLOCK_SIZE - size_of::<BlockHeader>();

#[repr(align(16))]
pub struct EncryptedBlock([u8; BLOCK_SIZE]);

impl EncryptedBlock {
    pub fn decrypt(self, key: Key) -> Block {
        todo!()
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
    /// Calculate the checksum for the block
    /// ECMA-182 based CRC64
    fn calculate_checksum(&self) -> u64 {
        todo!()
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

