use std::fmt;
use std::collections::HashSet;
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

    fn single_char_convert(test_char: u8) -> u16 {
        if PairChar::is_lowercase_ascii(test_char) {
            (test_char - b'a') as u16
        } else {
            panic!("Tried to encode a non-ascii character: {}", test_char);
        }

    }

    // handle non-ascii characters
    pub fn encode(char1: u8, char2: u8) -> PairChar {
        if char1 == b'_' && char2 == b'_' {
            PairChar { pair_char: 27*27 }
        } else {
            let val1: u16 = PairChar::single_char_convert(char1);
            let val2: u16 = PairChar::single_char_convert(char2);

            let pair_char: u16 = (26 * val1) + val2;

            PairChar { pair_char }
        }
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    pub fn decode(&self) -> String {
        self.pair_string.iter().map(|x| x.decode()).collect()
    }

    // return a set of words permuted by some number of spaces
    pub fn permute(&self, space_count: usize) -> Vec<PairString> {
        let mut word_set: HashSet<PairString> = HashSet::new();
        for word in PairString::permutation_recursor(self.clone(), space_count, 0) {
            word_set.insert(word);
        }
        word_set.iter().cloned().collect()
    }

    fn permutation_recursor(word: PairString, space_count: usize, depth: usize) -> Vec<PairString> {
        let mut return_vec = Vec::new();
        if depth == space_count {
            return_vec.push(word)
        } else {
            // get the vector of words from the next lower depth
            for permuted_word in PairString::permutation_recursor(word, space_count, depth+1) {
                for i in 0..(permuted_word.len() + 1) {
                    let mut new_word = permuted_word.clone();
                    new_word.pair_string.insert(i, PairChar::encode(b'_', b'_'));
                    return_vec.push(new_word);
                }
            }
        }
        return_vec
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

    pub fn assemble(slice: &[PairChar]) -> PairString {
        let mut return_val = PairString::new();
        for pair_char in slice {
            return_val.push(*pair_char);
        }
        return_val
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
