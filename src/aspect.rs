use crate::{Address, Block, Extent, ExtentHandle, Key, Keyword};
use std::sync::{Arc, Mutex, RwLock};

pub enum Error {
    /// The seed could not be located in the seed block
    SeedNotFound,
    /// The decoded pointer is bad
    BadPointer,
    /// Could not read a superblock header
    BadSuperBlock,
}

pub struct Aspect {
    extent_handle: ExtentHandle,
    keyword: Keyword,
    blocks: Vec<Address<Extent>>,
}

impl Aspect {
    pub fn new(extent: &ExtentHandle, keyword: Keyword) -> Aspect {
        Aspect {
            extent_handle: extent.clone(),
            keyword: keyword,
            blocks: Vec::new(),
        }
    }

    pub fn read_aspect(extent: &ExtentHandle, keyword: Keyword) -> Aspect {
        // Calculate seed_index
        // Scan until seed block found
        // Parse until next block id is invalid
        todo!()
    }

    pub fn read_block(&mut self, block_id: Address<Aspect>) -> Block {
        let block_id = self.transform_block_id(block_id);
        todo!()
    }

    pub fn len(&self) -> u64 {
        self.blocks.len() as u64
    }

    fn transform_block_id(&self, block_id: Address<Aspect>) -> Address<Extent> {
        self.blocks[*block_id as usize]
    }

    /// Calculate the first possible seed index
    fn seed_index(&self) -> Address<Extent> {
        self.keyword
            .seed_index(self.extent_handle.n_blocks())
            .into()
    }

    pub fn write_seed_block(&mut self) {
        let mut extent = self.extent_handle.lock().unwrap();
        let seed_index = extent.alloc_first(self.seed_index());
        let seed_block = Block::new().encrypt(self.block_key(0.into()));
        self.blocks.push(seed_index);
        extent.write_block(seed_index, seed_block);
    }

    /// Return the block key for the given block id
    pub fn block_key(&self, block_id: Address<Aspect>) -> Key {
        todo!()
    }

    /// Migrate a block from old_block_id to new_block_id
    pub fn move_block(
        &mut self,
        old_block_id: Address<Extent>,
        new_block_id: Address<Extent>,
    ) -> Result<(), Error> {
        //log::info!("Moving block {} to {}", old_block_id, new_block_id);
        //let block_content = self.read_block(old_block_id)?;
        //self.write_block(new_block_id, block_content);
        Ok(())
    }

    fn write_block(&mut self, block_id: u64, content: Block) -> Result<(), Error> {
        todo!()
    }
}
