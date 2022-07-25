extern crate openssl;
extern crate aes;
extern crate bitvec;
#[macro_use]
extern crate log;

mod aspect;
pub mod block;
mod keyword;
mod extent;

pub use aspect::Aspect;
pub use block::{EncryptedBlock, Block};
pub use keyword::{Keyword, Key};
pub use extent::{Extent, ExtentInfo};

fn main() {
    env_logger::init();
    let e = Extent::new("extent-0.bin", 10);
}
