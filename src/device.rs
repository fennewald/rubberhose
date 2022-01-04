use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;

/// A filesystem device
pub struct Device {
    aspects: HashMap<usize, usize>,
    reader: Box<dyn std::io::Read>,
    pub config: Config,
}

/// Device configurations
pub struct Config {
    pub device: &'static str,
    pub sector_size: u64,
}

// A device builder struct
pub struct DeviceBuilder {
    device: Option<&'static str>,
    sector_size: Option<u64>,
}

impl DeviceBuilder {
    pub fn device_name(mut self, name: &'static str) -> DeviceBuilder {
        self.device = Some(name);
        self
    }

    pub fn sector_size(mut self, size: u64) -> DeviceBuilder {
        self.sector_size = Some(size);
        self
    }

    /// Set the aspect
    pub fn from_file(mut self, file: File) -> DeviceBuilder {
        self
    }
}

impl Device {
    /// Create a new device, using a filename
    pub fn new(filename: &str) -> Device {
        let f = File::create(filename).unwrap();
        Device {
            aspects: HashMap::new(),
            reader: Box::new(f),
            config: Config {
                device: "File",
                sector_size: 1024,
            },
        }
    }
}
