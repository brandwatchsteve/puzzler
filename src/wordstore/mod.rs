use super::types::{PairString, WordList};

pub struct WordStore {
    word_store: Vec<WordList>,
}

impl WordStore {
    pub fn new() -> WordStore {
        let size = 12;
        // vec! macro would be useful here!
        let mut word_store: Vec<WordList> = Vec::with_capacity(size);
        for _ in 0..size {
            let new_store: WordList = Vec::new();
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
            self.word_store.resize_with(index_pt + 1, Vec::new);
        }

        // insert the word into the word_store
        self.word_store[index_pt].push(PairString::encode(word));
    }

    pub fn words_by_length(&self, size: usize) -> &WordList {
        &self.word_store[size - 1]
    }

    pub fn permuted_words_by_length(&self, pattern_size: usize) -> WordList {
        let mut return_list = WordList::new();
        let min_bound = if pattern_size % 2 == 0 {
            pattern_size / 2
        } else {
            1 + pattern_size / 2
        };
        for word_size in min_bound..pattern_size {
            let spaces = pattern_size - word_size;
            for word in self.word_store[word_size - 1].iter().cloned() {
                for permutation in word.permute(spaces) {
                    return_list.push(permutation);
                }
            }
        }
        return_list
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
