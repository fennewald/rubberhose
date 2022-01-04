mod device;

use device::Device;

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
fn main() {
    let d = Device::new_from_file("test.bin");
    println!("Device type: {}", d.config.device);
}
