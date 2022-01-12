extern crate openssl;
extern crate aes;

mod address;
mod block;
mod device;
mod keyword;
mod aspect;
mod mem;

pub use address::Address;
pub use block::{Block,BLOCK_SIZE};
pub use device::{Device, DeviceBuilder};
pub use keyword::{Keyword, Key};

fn demo_keyword() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.truncate(line.len()-1);
    let k = keyword::Keyword::new(line);
    println!("{}", k);
}

fn main() {
    let k = Keyword::new("test".to_string());
    println!("Lower 64: {:016x}", k.lower_64());
}
