use crate::{Address, Block, BLOCK_SIZE, Device, Keyword};

pub const SEED_FLAG: u64 = 0xc0debeefc0debeef;

pub enum Error {
    /// The seed could not be located in the seed block
    SeedNotFound,
    /// The decoded pointer is bad
    BadPointer,
    /// Could not read a superblock header
    BadSuperBlock,
}

/// On-disk representation of a saved aspect
struct AspectSeed {
    master_key: u8,
    checksum: u32,
}

pub struct Aspect<'a> {
    keyword: &'a Keyword,
    addresses: Vec<Address>,
}

impl<'a> Aspect<'_> {
    pub fn load_aspect(keyword: &Keyword, extent: &

    fn read(&self, address: Address, length: u64) {
        // Read the block
        // decrypt it
        // cut out section
    }
}


