// struct BigramIndex {
//     bigram: HashMap<String, Option<Box<BigramIndex>>>,
// }

pub mod bigramindex;
pub mod ingest;
pub mod wordstore;
pub mod puzzlegrid;

#[derive(Clone,Copy,Default,Debug)]
pub struct PairChar {
    pair_char: u16,
}

impl PairChar {
    pub fn new() -> PairChar {
        let pair_char = std::u16::MAX;

        PairChar{ pair_char }
    }

    pub fn encode(char1: u8, char2: u8) -> PairChar {
        let val1: u16 = (char1 - b'a') as u16;
        let val2: u16 = (char2 - b'a') as u16;

        let pair_char: u16 = (26 * val1) + val2;

        PairChar { pair_char }
    }

    pub fn decode(&self) -> String {
        if self.pair_char >= (27*26) {
            return "__".to_string();
        }

        let char1 = (self.pair_char / 26) as u8 + b'a';
        let char2 = (self.pair_char % 26) as u8 + b'a';

        let mut result = String::new();
        result.push(char1 as char);
        result.push(char2 as char);

        result
    }
}
