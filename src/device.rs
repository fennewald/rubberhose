use std::io::{self, Seek, Read, Write};
use std::fs::{self, File};
use std::path::Path;
use std::fmt;

pub const SECTOR_SIZE: usize = 1024;
pub type Sector = [u8; SECTOR_SIZE];

pub struct DeviceMetadata {
    name: &'static str,
}

pub trait BlockDevice {
    fn read_sector(&mut self, index: u64) -> Sector;
    fn write_sector(&mut self, index: u64, sector: Sector);
    fn n_sectors(&self) -> u64;
}

pub struct RAMDisk {
    data: Vec<Sector>,
}

impl BlockDevice for RAMDisk {
    fn read_sector(&mut self, index: u64) -> Sector {
        self.data[index as usize]
    }
    fn write_sector(&mut self, index: u64, sector: Sector) {
        self.data[index as usize] = sector
    }
    fn n_sectors(&self) -> u64 {
        self.data.len() as u64
    }
}

impl RAMDisk {
    pub fn new(n_sectors: usize) -> RAMDisk {
        RAMDisk {
            data: vec![[0; SECTOR_SIZE]; n_sectors],
        }
    }
}

pub struct ImageFile {
    metadata: fs::Metadata,
    file: File,
}

impl BlockDevice for ImageFile {
    fn read_sector(&mut self, index: u64) -> Sector {
        self.file.seek(io::SeekFrom::Start(index as u64)).unwrap();
        let mut buffer: Sector = [0; SECTOR_SIZE];
        self.file.read_exact(&mut buffer).unwrap();
        return buffer;
    }
    fn write_sector(&mut self, index: u64, sector: Sector) {
        self.file.seek(io::SeekFrom::Start(index as u64)).unwrap();
        self.file.write_all(&sector).unwrap();
    }
    fn n_sectors(&self) -> u64 {
        self.metadata.len() as u64
    }
}

impl ImageFile {
    pub fn open<P: AsRef<Path>>(filename: P) -> ImageFile {
        let file = File::options()
            .read(true)
            .write(true)
            .open(filename)
            .unwrap();
        let metadata = file.metadata().unwrap();
        ImageFile {
            metadata,
            file,
        }
    }

    pub fn create<P: AsRef<Path>>(filename: P, n_sectors: u64) -> ImageFile {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(filename)
            .unwrap();
        let sector = [0; SECTOR_SIZE];
        for _ in 0..n_sectors {
            file.write_all(&sector).expect("Couldn't write zeros");
        }
        let metadata = file.metadata().unwrap();
        ImageFile {
            metadata,
            file,
        }
    }
}


impl fmt::Debug for RAMDisk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n_sectors = self.n_sectors();
        writeln!(f, "Block Device: Size {:08x}", n_sectors)?;
        for i in 0..n_sectors {
            let sector = self.data[i as usize];
            let sum: u64 = sector.iter().map(|&n| n as u64).sum();
            writeln!(
                f,
                "{:04x}: {} {:02x?} {:02x?}",
                i,
                sum,
                &sector[0..8],
                &sector[SECTOR_SIZE-8..SECTOR_SIZE]
            )?;
        }
        writeln!(
            f,
            " id  sum  0                          7   {:4x}                        {:4x}",
            SECTOR_SIZE-8,
            SECTOR_SIZE-1
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write() {
        let mut disk = RAMDisk::new(10);
        let sector = [0xff; SECTOR_SIZE];
        disk.write_sector(0, sector);
        assert_eq!(disk.read_sector(0), sector);
    }

    #[test]
    fn persist_file() {
        let mut disk = ImageFile::create("test.bin", 10);
        let sector = [0xff; SECTOR_SIZE];
        disk.write_sector(0, sector);
        assert_eq!(disk.read_sector(0), sector);
        fs::remove_file("test.bin").unwrap();
    }
}
