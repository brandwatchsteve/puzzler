use std::fmt;
use std::ops::Index;

#[derive(Clone, Copy, Default, Debug, Eq, Hash, PartialEq)]
pub struct PairChar {
    pair_char: u16,
}

impl PairChar {
    pub fn new() -> PairChar {
        let pair_char = std::u16::MAX;

        PairChar { pair_char }
    }

    fn is_lowercase_ascii(test_char: u8) -> bool {
        test_char >= b'a' && test_char <= b'z'
    }

    // handle non-ascii characters
    pub fn encode(char1: u8, char2: u8) -> PairChar {
        let val1: u16 = (char1 - b'a') as u16;
        let val2: u16 = (char2 - b'a') as u16;

        let pair_char: u16 = (26 * val1) + val2;

        PairChar { pair_char }
    }

    pub fn decode(&self) -> String {
        if self.pair_char >= (27 * 26) {
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

#[derive(Clone,Debug)]
pub struct PairString {
    pub pair_string: Vec<PairChar>,
}

impl PairString {
    pub fn new() -> PairString {
        PairString {
            pair_string: Vec::new(),
        }
    }

    pub fn build(length: usize) -> PairString {
        let pair_string: Vec<PairChar> = vec![PairChar::new(); length];
        PairString { pair_string }
    }

    pub fn push(&mut self, pair_char: PairChar) {
        self.pair_string.push(pair_char);
    }

    pub fn encode(input_string: &str) -> PairString {
        let mut pair_string: Vec<PairChar> = Vec::new();
        for char_pair in input_string.as_bytes().chunks(2) {
            let char1 = char_pair[0];
            let char2 = char_pair[1];
            pair_string.push(PairChar::encode(char1, char2));
        }

        PairString { pair_string }
    }

    fn decode(&self) -> String {
        self.pair_string.iter().map(|x| x.decode()).collect()
    }

    pub fn len(&self) -> usize {
        self.pair_string.len()
    }

    pub fn slice(&self) -> &[PairChar] {
        &self.pair_string[..]
    }

    pub fn slice_to(&self, pos: usize) -> &[PairChar] {
        &self.pair_string[..pos]
    }

    pub fn reverse(&mut self) -> () {
        self.pair_string.reverse();
    }
}

impl fmt::Display for PairString {
    // convert the u16 into a pair of characters
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.decode())
    }
}

impl Index<usize> for PairString {
    type Output = PairChar;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.pair_string[pos]
    }
}

pub type WordList = Vec<PairString>;
