use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use bitvec::prelude::BitVec;
use crate::{Aspect, Block, EncryptedBlock, Keyword};

/// Extent information
pub struct ExtentInfo {
    n_blocks: usize,
}

pub struct Extent {
    block_usage_map: BitVec,
    f: File,
}

impl Extent {
    /// Allocates a new block, marking it as in use
    /// At this point, a record needs to be written to the linked list to make
    /// this valid
    pub fn alloc_block(&mut self) -> u64 {
        todo!()
    }

    /// Creates a new extent, using a file as the backing source
    pub fn new(filename: &str, n_blocks: u64) -> Extent {
        log::info!("Creating a new extent, {}, with {} blocks", filename, n_blocks);
        let f = File::create(filename).unwrap();
        let mut writer = BufWriter::new(f.try_clone().unwrap());
        for _ in 0..n_blocks {
            let mut buffer = [0; Block::size()];
            openssl::rand::rand_bytes(&mut buffer).unwrap();
            writer.write(&buffer).unwrap();
        }
        writer.flush().unwrap();
        Extent {
            block_usage_map: BitVec::new(),
            f: f,
        }
    }

    /// Load an extent from a file
    pub fn load(filename: &str) -> Extent {
        todo!()
    }

    /// Create a new extent on the disk, marking its head index as reserved
    pub fn create_aspect(&mut self, keyword: Keyword) -> Aspect {
        let mut seed_index = keyword::seed_index(10); // TODO put actual block length here
        todo!()
    }

    /// Load a new aspect stored on disk
    pub fn load_aspect(&mut self, keyword: Keyword) -> Aspect {
        todo!()
    }

    pub fn read_block(&mut self, block_id: u64) -> EncryptedBlock {
        todo!()
    }
}
