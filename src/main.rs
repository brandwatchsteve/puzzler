use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::collections::HashMap;

// struct BigramIndex {
//     bigram: HashMap<String, Option<Box<BigramIndex>>>,
// }

struct WordStore {
    word_store : Vec<Vec<String>>,
}

impl WordStore {
    pub fn new() -> WordStore {
        let size = 10;
        // vec! macro would be useful here!
        let mut word_store : Vec<Vec<String>> = Vec::with_capacity(size);
        for _ in 0..size {
            let new_store : Vec<String> = Vec::new();
            word_store.push(new_store);
        }
        WordStore {
            word_store,
        }
    }

    pub fn add(&mut self, word : String) {
        let word_len : usize = word.len();

        // exit if the word has an uneven size
        if word_len % 2 == 1 { return };

        // extend the vector if needs be
        let index_pt = (word_len / 2) - 1;

        if index_pt >= self.word_store.len() {
            // panic!("Word store len is less than {}", index_pt);
            println!("Growing the wordstore");
            self.word_store.resize_with(index_pt+1, || {Vec::new()});
        }

        // insert the word into the word_store
        self.word_store[index_pt].push(word);
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


fn main() {
    let mut word_store = WordStore::new();

    let source_file = "/usr/share/dict/words";
    let words = read_contents(source_file);

    // for (index, line) in words.iter().enumerate() {
    // words.iter().map(|x| word_store.add(x.to_string())).collect();
    for word in words {
        word_store.add(word);
    }

    word_store.print();
}

pub fn read_contents(src_file: &str) -> Vec<String> {
    let file = File::open(src_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|x| x.len() % 2 == 0)
        .filter(|x| x.chars().all(|y| y.is_ascii_lowercase()))
        .map(|x| x.to_string())
        .collect()
}

