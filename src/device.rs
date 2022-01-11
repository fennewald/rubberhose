use crate::mem::RAM;
use crate::{Block, BLOCK_SIZE, Keyword};

use std::collections::{HashMap, BTreeSet};
use std::io::{Read, Seek};
use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;

/// A filesystem device
pub struct Device {
    seed_blocks: BTreeSet<u64>,
    aspects: HashMap<Keyword, usize>,
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
    /// Create an aspect with the given name
    pub fn create_aspect(&mut self, keyword: &str) {
        let keyword = Keyword::new(keyword.to_string());
        let index = self.keyword_head_index(&keyword);
    }

    pub fn n_sectors(&self) -> u64 {
        10
    }

    pub fn keyword_head_index(&mut self, keyword: &Keyword) -> u64 {
        keyword.id() % self.n_sectors()
    }

    pub fn read_block(&mut self, index: u64) -> Block {
        let block = [0; BLOCK_SIZE];
        // TODO, seek and read
        return block;
    }
}
