

pub struct BitVec(u8);

/// A rubberhose extent
pub struct Extent {
    /// Length, in bytes, of the entire extent
    pub length: u64,
    use_map: BitVec,
}

impl Extent {

}
