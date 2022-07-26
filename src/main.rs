extern crate aes;
extern crate bitvec;
extern crate log;
extern crate openssl;

mod address;
mod aspect;
mod block;
mod extent;
mod keyword;

pub use address::Address;
pub use aspect::Aspect;
pub use block::{Block, EncryptedBlock};
pub use extent::{Extent, ExtentHandle};
pub use keyword::{Key, Keyword};

fn main() {
    env_logger::init();
    let e = Extent::load("extent-0.bin");
    let mut h = e.to_handle();
    let b = h.read_block(0.into()).unwrap();
    dbg!(b);
}
