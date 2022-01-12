use openssl::rand::rand_bytes;

const RAND_BUF_SIZE: usize = 1024;
static RAND_BUF: Vec<u8> = vec![0; RAND_BUF_SIZE];

fn refill_buffer() {
    RAND_BUF = vec![0; RAND_BUF_SIZE];
    rand_bytes(&mut RAND_BUF).unwrap();
}

fn rand_u8() -> u8 {
    if RAND_BUF.len() == 0 {
        refill_buffer();
    }
    RAND_BUF.pop().unwrap()
}
