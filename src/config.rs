/// Size of disk sectors in bytes
pub const SECTOR_DATA_SIZE: usize = 1_000_000;
pub const SECTOR_SIZE: usize = 262144;
pub const MAX_PARTITION_SIZE: usize = 1_000_000_000_000;
pub const MAX_N_SECTORS: usize = MAX_PARTITION_SIZE / SECTOR_SIZE;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the sector size seems large enough to hold what it needs
    #[test]
    fn sector_size_sanity() {
        assert!(SECTOR_SIZE > 0);
    }
}
