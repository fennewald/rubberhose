
/// Calculate the crc for some slice
pub fn crc64<'a, I>(data: I) -> u64
where
    I: IntoIterator<Item=&'a u8>,
    {
    const TABLE: [u64; 256] = gen_crc_table();
    let mut crc: u64 = 0;
    for b in data {
        let t = (crc >> 56) as u8 ^ b;
        crc = TABLE[t as usize] ^ (crc << 8);
    }
    return crc;
}

/// Generates a lookup table for crc calculation
// Adapted from the linux kernel lib/crc64.c
const fn gen_crc_table() -> [u64; 256] {
    const ECMA_POLY: u64 = 0x42F0E1EBA9EA3693;
    //const ROCKSOFT_POLY: u64 = 0x9A6C9329AC4BC9B5;
    let mut table = [0; 256];
    let mut crc: u64;
    let mut c: u64;
    let mut i = 0;

    while i < 256 {
        crc = 0;
        c = i << 56;
        let mut j = 0;
        while j < 8 {
            if (crc ^ c) & 0x8000000000000000 != 0 {
                crc = (crc << 1) ^ ECMA_POLY;
            } else {
                crc <<= 1;
            }
            c <<= 1;
            j += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }

    return table;
}
