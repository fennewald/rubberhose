use crate::{Block, Device, Keyword};

pub const SEED_FLAG: u64 = 0xc0debeefc0debeef;

pub enum Error {
    /// The seed could not be located in the seed block
    SeedNotFound,
    /// The decoded pointer is bad
    BadPointer,
    /// Could not read a superblock header
    BadSuperBlock,
}

pub struct Aspect<'a> {
    keyword: &'a Keyword,
    #[cfg(feature = "cache_address_table")]
    addresses: Vec<u64>,
}

impl<'a> Aspect<'_> {
    pub fn read_disk(keyword: &'a Keyword, disk: &mut Device) -> Result<Aspect<'a>, Error> {
        let seed_index = disk.keyword_head_index(keyword);
        let mut seed_block: Block = disk.read_block(seed_index);
        keyword.xor_block(&mut seed_block);
        for i in (0..BLOCK_SIZE).step_by(128) {
            let magic = u64::from_be_bytes(seed_block[i..i+32]).unwrap();
            if magic == SEED_FLAG {
                let head_index = u64::from_be_bytes(seed_block[i..i+32]).unwrap();
            }
        }
    }

    fn read(&self, address: u64, length: u64) {
        // Read the block
        // decrypt it
        // cut out section
    }
}


