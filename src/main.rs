use std::fs::File;
use std::io::{BufRead, BufReader};

use puzzler::WordStore;


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

