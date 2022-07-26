use crate::{Address, Aspect, Block, EncryptedBlock, Keyword};
use bitvec::prelude::BitVec;
use std::fs::File;
use std::io::{self, prelude::*, BufWriter, SeekFrom};
use std::sync::{self, Arc, Mutex, MutexGuard, PoisonError};

#[derive(Debug)]
pub enum Error {
    PoisonedData,
    IoError(io::Error),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_e: PoisonError<T>) -> Error {
        Error::PoisonedData
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

#[derive(Clone)]
pub struct ExtentHandle {
    handle: Arc<Mutex<Extent>>,
    n_blocks: u64,
}

impl ExtentHandle {
    /// Number of blocks in the entire aspect
    pub fn n_blocks(&self) -> u64 {
        self.n_blocks
    }

    /// Create a new aspect on disk
    pub fn create_aspect(&self, keyword: Keyword) -> Aspect {
        let mut aspect = Aspect::new(&self, keyword);
        aspect.write_seed_block()
    }

    /// Return a handle to the underlying extent object
    pub fn lock(&self) -> Result<MutexGuard<'_, Extent>, PoisonError<MutexGuard<'_, Extent>>> {
        self.handle.lock()
    }

    pub fn read_block(&self, block_id: Address<Extent>) -> Result<EncryptedBlock, Error> {
        let mut extent = self.handle.lock()?;
        extent.seek(block_id);
        let mut buffer = EncryptedBlock::new();
        extent.f.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_block(
        &self,
        block_id: Address<Extent>,
        block: EncryptedBlock,
    ) -> Result<(), Error> {
        let mut extent = self.handle.lock()?;
        extent.seek(block_id);
        extent.f.write_all(&*block)?;
        Ok(())
    }
}

pub struct Extent {
    block_usage_map: BitVec,
    f: File,
}

impl Extent {
    /// Get the number of blocks
    pub fn n_blocks(&self) -> u64 {
        self.block_usage_map.len() as u64
    }

    /// Return the first unallocated block after ID
    /// Mark the block as owned
    pub fn alloc_first(&self, seed_id: Address<Extent>) -> Address<Extent> {
        let n_blocks = self.n_blocks();
        let mut index = *seed_id;
        while self.is_allocated(index) {
            index = (index + 1) % n_blocks;
        }
        self.reserve_block(index);
        return index.into();
    }

    /// Move self into a
    pub fn to_handle(self) -> ExtentHandle {
        ExtentHandle {
            n_blocks: self.n_blocks(),
            handle: Arc::new(Mutex::new(self)),
        }
    }

    pub fn write_block(
        &mut self,
        address: Address<Extent>,
        block: EncryptedBlock,
    ) -> Result<(), Error> {
        self.seek(address);
        self.f.write_all(&block);
        Ok(())
    }

    /// Seek to the given block id
    fn seek(&mut self, block_id: Address<Extent>) {
        // TODO handle failure
        self.f
            .seek(SeekFrom::Start(*block_id * Block::size() as u64));
    }

    /// Allocates a new block, marking it as in use
    /// At this point, a record needs to be written to the linked list to make
    /// this valid
    pub fn alloc_block(&mut self) -> u64 {
        todo!()
    }

    /// Reserve the given block, returning Ok(()) on succses
    pub fn reserve_block(&mut self, block_id: u64) -> Result<(), ()> {
        if self.block_usage_map[block_id as usize] {
            Err(())
        } else {
            self.block_usage_map.set(block_id as usize, true);
            Ok(())
        }
    }

    /// Test if the given block is currently allocated
    pub fn is_allocated(&self, block_id: u64) -> bool {
        self.block_usage_map[block_id as usize]
    }

    /// Creates a new extent, using a file as the backing source
    pub fn new(filename: &str, n_blocks: u64) -> Extent {
        log::info!(
            "Creating a new extent, {}, with {} blocks",
            filename,
            n_blocks
        );
        let f = File::create(filename).unwrap();
        let mut writer = BufWriter::new(f.try_clone().unwrap());
        for _ in 0..n_blocks {
            let mut buffer = [0; Block::size()];
            openssl::rand::rand_bytes(&mut buffer).unwrap();
            writer.write(&buffer).unwrap();
        }
        writer.flush().unwrap();
        Extent {
            block_usage_map: BitVec::with_capacity(n_blocks as usize),
            f: f,
        }
    }

    /// Load an extent from a file
    pub fn load(filename: &str) -> Extent {
        let f = File::open(filename).unwrap();
        let length = f.metadata().unwrap().len();
        Extent {
            block_usage_map: BitVec::new(),
            f: f,
        }
    }
}
