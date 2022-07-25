use crate::mem::RAM;
use crate::{Address, Block, BLOCK_SIZE, Keyword};

use std::collections::{HashMap, BTreeSet};
use std::io::{Read, Seek};
use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;

/// A filesystem device
pub struct Device {
    /// A set of all blocks in the filesystem that are seeds
    seed_blocks: BTreeSet<Address>,
    aspects: HashMap<Keyword, Address>,
    reader: Box<dyn Read>,
    pub config: Config,
}

/// Device configurations
pub struct Config {
    pub device: &'static str,
    pub sector_size: u64,
}

/// A device builder struct
pub struct DeviceBuilder {
    device: Option<&'static str>,
    sector_size: Option<u64>,
    aspects: Option<Vec<String>>,
}

impl DeviceBuilder {
    pub fn new() -> DeviceBuilder {
        DeviceBuilder {
            device: None,
            sector_size: None,
            aspects: None,
        }
    }

    pub fn with_aspect(mut self, aspect: String) -> DeviceBuilder {
        if self.aspects.is_none() {
            self.aspects = Some(vec![aspect]);
        } else {
            if let Some(ref mut v) = self.aspects {
                v.push(aspect);
            } else {
                panic!("Invalid builder vector");
            }
        }
        self
    }

    pub fn device_name(mut self, name: &'static str) -> DeviceBuilder {
        self.device = Some(name);
        self
    }

    pub fn sector_size(mut self, size: u64) -> DeviceBuilder {
        self.sector_size = Some(size);
        self
    }

    /// Set the aspect
    pub fn from_file(mut self, file: File) -> Device {
        Device {
            seed_blocks: BTreeSet::new(),
            aspects: HashMap::new(),
            reader: Box::new(file),
            config: Config {
                device: self.device.unwrap_or("File"),
                sector_size: self.sector_size.unwrap_or(1024),
            },
        }
    }

    pub fn from_new_file(mut self) -> Device {
        let mut file_index = 0;
        let mut path = format!("tmp-file-{:02x}.bin", file_index);
        loop {
            if Path::new(&path).exists() {
                file_index += 1;
                path = format!("device-{:02x}.bin", file_index);
            } else {
                break;
            }
        }
        return self.from_file(File::create(path).unwrap());
    }

    pub fn load_file(mut self, filename: &str) -> Device {
        self.from_file(File::open(filename).unwrap())
    }

    pub fn overwrite_file(mut self, filename: &str) -> Device {
        fs::remove_file(filename).unwrap();
        self.from_file(File::create(filename).unwrap())
    }
}

impl Device {
    /// Load the seed block specified by keyword
    /// This fully decrypts the block
    fn load_seed_block(&mut self, keyword: &Keyword) -> Block {
        let index = self.keyword_seed_index(&keyword);
        let mut block = self.read_block(index);
        block.xor_key(keyword.hash());
        return block;
    }

    /// Load the aspect given
    pub fn load_aspect(&mut self, keyword: Keyword) {
        let seed_block = self.load_seed_block(&keyword);
    }

    /// Create an aspect with the given name
    pub fn create_aspect(&mut self, keyword: &str) {
        let keyword = Keyword::new(keyword.to_string());
        let index = self.keyword_seed_index(&keyword);
        // Encode seed block
        let seed_block = self.read_block(index);
    }

    pub fn n_sectors(&self) -> u64 {
        10
    }

    pub fn keyword_seed_index(&mut self, keyword: &Keyword) -> Address {
        (keyword.lower_64() % self.n_sectors()).into()
    }

    pub fn read_block(&mut self, index: Address) -> Block {
        return Block::default();
    }
}
