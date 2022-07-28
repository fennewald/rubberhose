extern crate aes;
extern crate bitvec;
extern crate log;
extern crate openssl;

//mod address;
//mod aspect;
//mod block;
mod device;
//mod extent;
//mod keyword;

//pub use address::Address;
//pub use aspect::Aspect;
//pub use block::{Block, EncryptedBlock};
use device::BlockDevice;
//pub use extent::{Extent, ExtentHandle};
//pub use keyword::{Key, Keyword};

fn main() {
    env_logger::init();
    let mut image = device::RAMDisk::new(1000000);
    println!("{:?}", image);
}
