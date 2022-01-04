
/// The hash returned from our hashing function
pub type Hash = [u8; 32];

pub struct Keyword {
    text: String,
    hash: Hash,
}

fn hash(text: &str) -> Hash {

}

impl Keyword {
    pub fn new(text: String) -> Keyword {
        Keyword {
            text: text,
        }
    }
}
