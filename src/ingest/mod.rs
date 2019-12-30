use std::fs::File;
use std::io::{BufRead, BufReader};

// read words from a file, returning only those which are all-lowercase-ascii
// and having an even number of characters
pub fn read_even_words(src_file: &str) -> Vec<String> {
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
