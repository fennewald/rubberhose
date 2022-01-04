

/// Anything that implements this interface can be turned into a block device
trait BlockDevice {
    fn open() -> i32;
    fn release() -> i32;
    fn ioctl() -> i32;
    fn media_changed() -> i32;
    fn revalidate_disk() -> i32;
}
