extern crate aes;
extern crate bitvec;
extern crate log;
extern crate openssl;

mod aspect;
mod crc;
mod block;
mod device;
mod extent;
mod keyword;

#[cfg(test)]
mod tests;

use aspect::Aspect;
use block::{Block, RawBlock};
use device::{BlockDevice, RAMDisk};
use extent::{Extent, ExtentHandle};
use keyword::{Key, Keyword};

fn main() {
    env_logger::init();
    let mut disk = RAMDisk::new(8);
    disk.read_sector(0);
    //disk.write_sector(0, &Sector::new_rand());
    let mut e = Extent::new(disk).to_handle();
    log::info!("Disk view: {:?}", e);
    let a = e.create_aspect(Keyword::new("Hello".to_string()));
    log::info!("Disk view: {:?}", e);
}

