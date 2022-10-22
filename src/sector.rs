use openssl::rand::rand_bytes;

use std::fmt;

use crate::crc;

pub const SECTOR_SIZE: usize = 1024;

/// A disk sector, the smallest unit that can be read or written from the block device
#[derive(Debug, Clone)]
pub struct Sector([u8; SECTOR_SIZE]);

impl Sector {
    pub fn new() -> Sector {
        Sector([0; SECTOR_SIZE])
    }

    pub fn new_rand() -> Sector {
        let mut s = Sector::new();
        s.randomize();
        s
    }

    /// Randomize all data within self
    pub fn randomize(&mut self) {
        rand_bytes(&mut self.0).unwrap()
    }

    pub fn crc(&self) -> u64 {
        crc::crc64(
            &self.0
        )
    }

    /// Outputs a format line header
    pub fn header(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "crc         0                          7   {:4x}                        {:4x}",
            SECTOR_SIZE-8,
            SECTOR_SIZE-1
        )
    }
}

impl Into<[u8; SECTOR_SIZE]> for Sector {
    fn into(self) -> [u8; SECTOR_SIZE] {
        self.0
    }
}

impl fmt::Display for Sector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:016x} {:02x?} {:02x?}",
            self.crc(),
            &self.0[0..8],
            &self.0[SECTOR_SIZE-8..SECTOR_SIZE]
        )
    }
}

impl Eq for Sector {}
impl PartialEq for Sector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl AsRef<[u8; SECTOR_SIZE]> for Sector {
    fn as_ref(&self) -> &[u8; SECTOR_SIZE] {
        &self.0
    }
}

impl AsMut<[u8; SECTOR_SIZE]> for Sector {
    fn as_mut(&mut self) -> &mut [u8; SECTOR_SIZE] {
        &mut self.0
    }
}
