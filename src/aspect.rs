use crate::{Key, Keyword, ExtentInfo, Block, Extent};
use std::sync::{Arc,RwLock, Mutex};

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
    extent_info: &'a ExtentInfo,
    extent: Arc<Mutex<Extent>>,
    keyword: Keyword,
    collision_map: Arc<RwLock<usize>>,
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

    /*
    fn read_from_disk<'b>(keyword: Keyword, extent_info: &'a ExtentInfo) -> Aspect<'b> {
        let mut self = Aspect {
            extent_info: extent_info,
            keyword: Keyword,
        };
    }
    */
    /*
    pub fn read_disk(keyword: &'a Keyword, disk: &mut Device) -> Result<Aspect<'a>, Error> {
        let seed_index = disk.keyword_seed_index(keyword);
        let mut seed_block: Block = disk.read_block(seed_index);
        for i in (0..BLOCK_SIZE).step_by(128) {
            let magic =
                u64::from_be_bytes(seed_block[i..i+32].try_into().unwrap());
            if magic == SEED_FLAG {
                let head_index =
                    u64::from_be_bytes(seed_block[i..i+32].try_into().unwrap());
            }
        }
        Err(Error::SeedNotFound)
    }
    */
}
