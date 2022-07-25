use crate::{Key, Keyword, Block, ExtentHandle};
use std::sync::{Arc, RwLock, Mutex};

pub enum Error {
    /// The seed could not be located in the seed block
    SeedNotFound,
    /// The decoded pointer is bad
    BadPointer,
    /// Could not read a superblock header
    BadSuperBlock,
}

pub struct Aspect {
    extent: ExtentHandle,
    keyword: Keyword,
    blocks: Vec<u64>,
}


impl Aspect {
    pub fn new(extent: &ExtentHandle, keyword: Keyword) -> Aspect {
        Aspect {
            extent: extent.clone(),
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

    pub fn len(&self) -> u64 {
        self.blocks.len() as u64
    }

    fn transform_block_id(&self, block_id: u64) -> u64 {
        self.blocks[block_id as usize]
    }

    /// Calculate the first possible seed index
    fn seed_index(&self) -> u64 {
        self.keyword.seed_index(self.extent.n_blocks())
    }

    pub fn write_seed_block(&mut self) {
        let mut seed_index = self.seed_index();
        let n_blocks = self.extent.n_blocks();
        let mut extent = self.extent.lock().unwrap();

        while extent.is_allocated(seed_index) {
            log::info!("Skipping allocated block {} while searching for seed index", seed_index);
            seed_index = (seed_index + 1) % n_blocks;
        }
    }

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

    fn write_block(&mut self, block_id: u64, content: Block) -> Result<(), Error> {
        todo!()
    }

    fn read_block(&self, block_id: u64) -> Result<Block, Error> {
        todo!()
    }
}
