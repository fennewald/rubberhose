

pub struct CollisionMap {
    bitmap: Vec<u8>,
    n_set: u64,
}

impl CollisionMap {
    fn get(&self, index: u32) -> bool {
        let (byte_index, bit_index) = (index / 8, index % 8);
    }
}

/// Given an index, split it into the byte and bit index of a bitvec
fn split_index(index: u32) -> (u32, u8) {
    (index / 8, (index % 8) as u8)
}
