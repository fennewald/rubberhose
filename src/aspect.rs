use crate::{Key, Keyword, ExtentInfo, Block, Extent};
use std::sync::{Arc,RwLock, Mutex};

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
    extent_info: &'a ExtentInfo,
    extent: Arc<Mutex<Extent>>,
    keyword: Keyword,
}


impl<'a> Aspect<'_> {
    /// Return the block key for the given block id
    pub fn block_key(&self, block_id: u64) -> Key {
        todo!()
    }

    /// Migrate a block from old_block_id to new_block_id
    pub fn move_block(&mut self, old_block_id: u64, new_block_id: u64) -> Result<(), Error> {
        log::info!("Moving block {} to {}", old_block_id, new_block_id);
        let block_content = self.read_block(old_block_id)?;
        self.write_block(new_block_id, block_content);
        Ok(())
    }

    fn write_block(&mut self, block_id: u64, content: u8) -> Result<(), Error> {
        todo!()
    }

    fn read_block(&self, block_id: u64) -> Result<Block, Error> {
        let mut extent = self.extent.lock().unwrap();
        let raw_block = extent.read_block(block_id);
        let block = raw_block.decrypt(self.block_key(block_id));
        todo!()
    }
}
