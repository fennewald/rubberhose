extern crate openssl;
extern crate aes;

mod device;
mod keyword;
mod aspect;
mod sector;
mod block;
mod mem;

pub use keyword::{Keyword, Key};
pub use device::{Device, DeviceBuilder};
pub use block::{Block,BLOCK_SIZE};


/*
type DevicePointer = u32;
type Hash = [u8; 32];

/// An entire filesystem
struct Device {
    aspects: HashMap<Password, Aspect>,
    sector_size: usize,
}

struct Password {
    word: String,
    hash: Hash,
}

struct Aspect {
    password: Box<Password>,
    sectors: Vec<DevicePointer>,
}

/// Reference to a on disk sector
struct Sector {
    offset: DevicePointer,
}

impl Device {

}
*/

fn demo_keyword() {
    let mut line = String::new();
    print!("Keyword: ");
    std::io::stdin().read_line(&mut line).unwrap();
    line.truncate(line.len()-1);
    let k = keyword::Keyword::new(line);
    println!("({:08x}){} -> {}", k.id(), k.text(), k.pretty_hash());
}

fn main() {
    loop {
        demo_keyword();
    }
}
