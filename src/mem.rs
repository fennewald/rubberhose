

pub trait RAM {
    /// Read len bytes starting at address address
    fn read(&self, address: usize, len: usize) -> &[u8];
    /// Read len bytes starting at address address
    fn write(&self, address: usize, len: usize) -> &[u8];
}
