use crate::device::{RAMDisk, BlockDevice};
use crate::{Aspect, Block, Keyword, EncryptedBlock};
use bitvec::prelude::*;
use std::fmt;
use std::io::{self, prelude::*, BufWriter, SeekFrom};
use std::sync::{self, Arc, Mutex, MutexGuard, PoisonError};

type Device = RAMDisk;

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
    handle: Arc<Mutex<Extent<Device>>>,
    n_blocks: u64,
}

impl fmt::Debug for ExtentHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.handle.lock().unwrap().fmt(f)
    }
}

impl ExtentHandle {
    /// Number of blocks in the entire aspect
    pub fn n_blocks(&self) -> u64 {
        self.n_blocks
    }

    /// Create a new aspect on disk
    pub fn create_aspect(&self, keyword: Keyword) -> Aspect {
        let mut aspect = Aspect::new(&self, keyword);
        aspect.write_seed_block();
        return aspect;
    }

    // Return a handle to the underlying extent object
    pub fn lock(&self) -> Result<MutexGuard<'_, Extent<Device>>, PoisonError<MutexGuard<'_, Extent<Device>>>> {
        self.handle.lock()
    }
}

pub struct Extent<T: BlockDevice + fmt::Debug> {
    block_usage_map: BitVec,
    device: T,
}

impl<T: BlockDevice + fmt::Debug> fmt::Debug for Extent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Extent:")?;
        writeln!(f, "    Block Usage map: {}", self.block_usage_map)?;
        writeln!(f, "    {:?}", self.device)?;
        Ok(())
    }
}

impl Extent<Device> {
    /// Move self into a handle
    pub fn to_handle(self) -> ExtentHandle {
        ExtentHandle {
            n_blocks: self.n_blocks(),
            handle: Arc::new(Mutex::new(self)),
        }
    }
}

impl<T: BlockDevice + fmt::Debug> Extent<T> {
    pub fn new(device: T) -> Extent<T> {
        Extent {
            block_usage_map: bitvec![0; device.n_sectors() as usize],
            device,
        }
    }

    pub fn n_blocks(&self) -> u64 {
        self.device.n_sectors()
    }

    pub fn read_block(&mut self, block_index: u64) -> EncryptedBlock {
        self.device.read_sector(block_index).into()
    }

    pub fn write_block(&mut self, block_index: u64, block: &EncryptedBlock) {
        self.device.write_sector(block_index, block.as_ref())
    }

    // Allocator functions
    /// Mark the given block as deallocated
    pub fn deallocate_block(&mut self, block_index: u64) {
        self.block_usage_map.set(block_index as usize, false)
    }

    /// Mark a block as owned
    pub fn alloc_block(&mut self, block_index: u64) {
        self.block_usage_map.set(block_index as usize, true)
    }

    /// Mark the first block after seed
    pub fn alloc_next_block(&mut self, block_index: u64) -> u64 {
        let mut index = block_index;
        loop {
            if !self.block_usage_map[index as usize] {
                self.alloc_block(index);
                return index;
            }
            index = (index + 1) % self.device.n_sectors();
        }
    }
}
