// use std::fs::File;
// use std::io::{BufRead, BufReader};

use puzzler::wordstore::WordStore;
use puzzler::ingest;


fn main() {
    let mut word_store = WordStore::new();

    // let words = read_contents(source_file);
    // for (index, line) in words.iter().enumerate() {
    // words.iter().map(|x| word_store.add(x.to_string())).collect();

    let source_file = "/usr/share/dict/words";
    for word in ingest::read_contents(source_file) {
        word_store.add(&word);
    }

    word_store.print();
}
