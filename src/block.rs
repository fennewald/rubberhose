use aes::cipher::generic_array::GenericArray;
use aes::{Aes256, NewBlockCipher, BlockEncrypt};
use std::fmt;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};

use crate::crc::crc64;
use crate::{Extent, Key};

const BLOCK_SIZE: usize = 1024;
const BLOCK_DATA_SIZE: usize = BLOCK_SIZE - size_of::<BlockHeader>();

#[repr(transparent)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct RawBlock([u8; BLOCK_SIZE]);

impl RawBlock {
}

/*
impl EncryptedBlock {
    pub fn new() -> EncryptedBlock {
        [0; BLOCK_SIZE] // TODO add uninit version
    }
    pub fn decrypt(self, key: Key) -> Block {
        todo!()
    }
}
*/

pub struct EncryptedBlock([u8; BLOCK_SIZE]);

impl From<Sector> for EncryptedBlock {
    fn from(sector: Sector) -> EncryptedBlock {
        EncryptedBlock(sector.into())
    }
}

/*
impl AsRef<Sector> for EncryptedBlock {
    fn as_ref(&self) -> &Sector {
        &self.0
    }
}
*/

impl fmt::Debug for EncryptedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x?}", self.0)
    }
}

#[repr(C)]
pub struct BlockHeader {
    next_sector_id: u64,
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
            next_sector_id: 0xffff,
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
        let key = GenericArray::from(key);
        let cipher = Aes256::new(&key);
        let mut buffer: EncryptedBlock = unsafe { std::mem::transmute(self) };
        let generic = GenericArray::from_mut_slice(&mut buffer.0);
        cipher.encrypt_block(generic);
        return buffer;
    }

    pub const fn size() -> usize {
        BLOCK_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_size() {
        assert_eq!(BLOCK_SIZE, std::mem::size_of::<Block>())
    }

    #[test]
    fn crc_sanity() {
        assert_eq!(crc64(&[0x00, 0x01, 0x02, 0x04]), 8513814196102790297);
    }
}
