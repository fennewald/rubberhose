use super::*;

#[repr(transparent)]
pub struct RawBlock([u8; BLOCK_SIZE]);
