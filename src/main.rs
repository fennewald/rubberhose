extern crate openssl;
extern crate aes;
extern crate bitvec;
extern crate log;

mod address;
mod aspect;
mod block;
mod keyword;
mod extent;

pub use address::Address;
pub use aspect::Aspect;
pub use block::{EncryptedBlock, Block};
pub use keyword::{Keyword, Key};
pub use extent::{Extent, ExtentHandle};

fn main() {
    env_logger::init();
    let e = Extent::load("extent-0.bin");
    let mut h = e.to_handle();
    let b = h.read_block(0).unwrap();
    dbg!(b);
}
