use std::fmt;
// use std::collections::HashMap;

// struct BigramIndex {
//     bigram: HashMap<String, Option<Box<BigramIndex>>>,
// }

struct PairChar {
    pair_char: u16,
}

impl PairChar {
    fn encode(char1: u8, char2: u8) -> PairChar {
        let val1: u16 = (char1 - b'a') as u16;
        let val2: u16 = (char2 - b'a') as u16;

        let pair_char: u16 = (26 * val1) + val2;

        PairChar { pair_char }
    }

    fn decode(&self) -> String {
        let char1 = (self.pair_char / 26) as u8 + b'a';
        let char2 = (self.pair_char % 26) as u8 + b'a';

        let mut result = String::new();
        result.push(char1 as char);
        result.push(char2 as char);

        result
    }
}

struct PairString {
    pair_string: Vec<PairChar>,
}

impl PairString {
    fn encode(input_string: &str) -> PairString {
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
}

impl fmt::Display for PairString {
    // convert the u16 into a pair of characters
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.decode())
    }
}

pub struct WordStore {
    // word_store : Vec<Vec<String>>,
    word_store: Vec<Vec<PairString>>,
}

impl WordStore {
    pub fn new() -> WordStore {
        let size = 10;
        // vec! macro would be useful here!
        let mut word_store: Vec<Vec<PairString>> = Vec::with_capacity(size);
        for _ in 0..size {
            let new_store: Vec<PairString> = Vec::new();
            word_store.push(new_store);
        }
        WordStore { word_store }
    }

    pub fn add(&mut self, word: &str) {
        let word_len: usize = word.len();

        // exit if the word has an uneven size
        if word_len % 2 == 1 {
            return;
        };

        let index_pt = (word_len / 2) - 1;

        // grow the word_store if required
        if index_pt >= self.word_store.len() {
            println!("Growing the wordstore");
            self.word_store.resize_with(index_pt + 1, || Vec::new());
        }

        // insert the word into the word_store
        self.word_store[index_pt].push(PairString::encode(word));
    }

    pub fn print(&self) {
        for (index, array) in self.word_store.iter().enumerate() {
            println!("{}", index);
            for word in array {
                println!("  - {}", word);
            }
        }
    }
}
