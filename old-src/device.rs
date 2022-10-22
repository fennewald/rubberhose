use std::io::{self, Seek, Read, Write};
use std::fs::{self, File};
use std::path::Path;
use std::fmt;

use crate::crc::crc64;
use crate::RawBlock;


pub struct DeviceMetadata {
    name: &'static str,
}

/// A backing device, capable of reading and writing blocks
pub trait BlockDevice: fmt::Debug {
    fn read(&mut self, index: u64) -> RawBlock;
    fn write(&mut self, index: u64, block: &RawBlock);
    fn len(&self) -> u64;

    fn meta(&self) -> DeviceMetadata {
        DeviceMetadata {
            name: "Not set"
        }
    }

    /// Fill the entire disk with random information
    /// WARNING: This erases all data
    fn randomize(&mut self) {
        log::info!("Random-overwriting disk {self:?}");
        for i in 0..self.len() {
            self.write(i, &Sector::new_rand())
        }
    }
}

pub struct RAMDisk {
    data: Vec<Sector>,
}

impl BlockDevice for RAMDisk {
    fn read_sector(&mut self, index: u64) -> Sector {
        log::debug!("Read sector 0x{:x}", index);
        self.data[index as usize]
    }

    fn write_sector(&mut self, index: u64, sector: &Sector) {
        log::debug!("Write sector 0x{:x}", index);
        self.data[index as usize] = *sector;
    }

    fn n_sectors(&self) -> u64 {
        self.data.len() as u64
    }
}

impl fmt::Debug for RAMDisk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n_sectors = self.n_sectors();
        writeln!(f, "Block Device: Size {:8x}", n_sectors)?;
        for i in 0..n_sectors {
            let sector = self.data[i as usize];
            writeln!(f, "{i:04x}: {sector}")?;
        }
        write!(f, " id    ")?;
        Sector::header(f)?;
        writeln!(f);
        Ok(())
    }
}

impl RAMDisk {
    pub fn new(n_sectors: usize) -> RAMDisk {
        log::info!("Created a RAMDisk with {} sectors", n_sectors);
        RAMDisk {
            data: vec![Sector::new(); n_sectors],
        }
    }
}

#[derive(Debug)]
pub struct ImageFile {
    metadata: fs::Metadata,
    file: File,
}

impl BlockDevice for ImageFile {
    fn meta(&self) -> DeviceMetadata {
        todo!()
    }

    fn read_sector(&mut self, index: u64) -> Sector {
        log::debug!("Read sector 0x{:x}", index);
        self.file.seek(io::SeekFrom::Start(index as u64)).unwrap();
        let mut buffer: Sector = Sector::new();
        self.file.read_exact(buffer.as_mut()).unwrap();
        return buffer;
    }
    fn write_sector(&mut self, index: u64, sector: &Sector) {
        log::debug!("Write sector 0x{:x}", index);
        self.file.seek(io::SeekFrom::Start(index as u64)).unwrap();
        self.file.write_all(sector.as_ref()).unwrap();
    }
    fn n_sectors(&self) -> u64 {
        self.metadata.len() as u64
    }
}

impl ImageFile {
    /// Open an existing ImageFile from disk
    pub fn open<P: AsRef<Path>>(filename: P) -> io::Result<ImageFile> {
        let file = File::options()
            .read(true)
            .write(true)
            .open(filename)?;
        let metadata = file.metadata()?;
        Ok(ImageFile {
            metadata,
            file,
        })
    }

    /// Create a new image file
    pub fn create<P: AsRef<Path>>(filename: P, n_sectors: u64) -> io::Result<ImageFile> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(filename)?;
        let sector = Sector::new();
        for _ in 0..n_sectors {
            file.write_all(sector.as_ref())?;
        }
        let metadata = file.metadata()?;

        Ok(ImageFile {
            metadata,
            file,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write() {
        let mut disk = RAMDisk::new(10);
        let sector = Sector::new_rand();
        disk.write_sector(0, &sector);
        assert_eq!(disk.read_sector(0), sector);
    }

    #[test]
    fn persist_file() {
        let sector = Sector::new_rand();
        {
            let mut disk = ImageFile::create("test.bin", 10).unwrap();
            disk.write_sector(0, &sector);
        }
        {
            let mut disk = ImageFile::create("test.bin", 10).unwrap();
            assert_eq!(disk.read_sector(0), sector);
        }
        fs::remove_file("test.bin").unwrap();
    }
}
