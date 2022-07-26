use crate::{Address, Extent, Key};
use aes::{Aes256, NewBlockCipher};
use std::fmt;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};

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
    next_sector_id: Address<Extent>,
    checksum: u64,
}

#[repr(C)]
pub struct Block {
    header: BlockHeader,
    data: [u8; BLOCK_DATA_SIZE],
}

impl Block {
    pub fn new() -> Block {
        let header = BlockHeader {
            next_sector_id: 0xffff.into(),
            checksum: 0,
        };
        let mut block = Block {
            header: header,
            data: [0; BLOCK_DATA_SIZE],
        };
        block.update_checksum();
        return block;
    }

    /// Calculate the checksum for the block
    /// ECMA-182 based CRC64
    fn calculate_checksum(&self) -> u64 {
        crc64(&self.data)
    }

    fn update_checksum(&mut self) {
        self.header.checksum = self.calculate_checksum()
    }

    /// Tests if the checksum is valid
    pub fn validate_checksum(&self) -> bool {
        self.calculate_checksum() == self.header.checksum
    }

    pub fn encrypt(self, key: Key) -> EncryptedBlock {
        let cipher = Aes256::new(key);
        let mut buffer: EncryptedBlock = unsafe { std::mem::transmute(self) };
        cipher.encrypt_block(&mut buffer);
        return buffer;
    }

    pub const fn size() -> usize {
        BLOCK_SIZE
    }
}

/// Calculate the crc for some slice
fn crc64(data: &[u8]) -> u64 {
    const TABLE: [u64; 256] = gen_crc_table();
    let mut crc: u64 = 0;
    for b in data {
        let t = (crc >> 56) as u8 ^ b;
        crc = TABLE[t as usize] ^ (crc << 8);
    }
    return crc;
}

/// Generates a lookup table for crc calculation
// Adapted from the linux kernel lib/crc64.c
const fn gen_crc_table() -> [u64; 256] {
    const ECMA_POLY: u64 = 0x42F0E1EBA9EA3693;
    const ROCKSOFT_POLY: u64 = 0x9A6C9329AC4BC9B5;
    let mut table = [0; 256];
    let mut crc: u64;
    let mut c: u64;
    let mut i = 0;

    while i < 256 {
        crc = 0;
        c = i << 56;
        let mut j = 0;
        while j < 8 {
            if (crc ^ c) & 0x8000000000000000 != 0 {
                crc = (crc << 1) ^ ECMA_POLY;
            } else {
                crc <<= 1;
            }
            c <<= 1;
            j += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }

    return table;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_sanity() {
        assert_eq!(crc64(&[0x00, 0x01, 0x02, 0x04]), 8513814196102790297);
    }
}
