use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
// use std::ops::{Index, IndexMut};

use super::types::{PairChar, PairString};
use super::wordstore::WordStore;

pub struct BigramIndex {
    index: BigramIndexInner,
}
type BigramIndexInner = HashMap<PairChar, Option<RefCell<BigramIndex>>>;

impl BigramIndex {
    pub fn new() -> BigramIndex {
        let index: BigramIndexInner = HashMap::new();

        BigramIndex{ index }
    }

    fn add_leaf(&mut self, key: PairChar) {
        let new_leaf = BigramIndex::new();
        self.index.insert(key, Some(RefCell::new(new_leaf)));
    }

    pub fn print(&self, prefix: &str) {
        for (key, leaf) in &self.index {
            let word = format!("{}-{}", prefix, &key.decode());
            match leaf {
                Some(l) => {
                    l.borrow().print(&word);
                },
                None => println!("{}", word),
            }
        }
    }

    // use a wordlist...
    pub fn build(size: usize, word_store: &WordStore) -> BigramIndex {
        let mut root = BigramIndex::new();

        for word in word_store.words_by_length(size) {
            BigramIndex::index_word(&mut root, word.slice());
        }

        // return the populated index
        root
    }

    fn index_word(node: &mut BigramIndex, pair_slice: &[PairChar]) {
        let key_char = &pair_slice[0];

        match pair_slice.len() {
            0 => panic!("Inserting empty string into the index"),
            1 => {node.index.insert(*key_char, None); },
            _ => {
                if !node.index.contains_key(key_char) {
                    node.add_leaf(*key_char);
                };

                // insert the rest of the pair_slice
                let remaining_slice = &pair_slice[1..];
                BigramIndex::index_word(&mut node.index[&key_char].as_ref().unwrap().borrow_mut(), remaining_slice);
            }
        }
    }

    fn next_possibles(node: &BigramIndex, pair_slice: &[PairChar]) -> Option<HashSet<PairChar>> {
        let key_char = &pair_slice[0];

        let last_char = pair_slice.len() == 1;

        if let None = node.index[&key_char] {
            // we've hit the end of the indexchain before we've run out of word
            // meaning there are no subsequent next_possibles
            return None;
        }

        let next_index_ref = node.index[&key_char].as_ref().unwrap().borrow();
        if last_char {
            // we've got to the end of our sequence
            //
            // extract the keys for that index
            let mut next_keys : HashSet<PairChar> = HashSet::new();
            for key in next_index_ref.index.keys() {
                next_keys.insert(key.clone());
            }
            //
            Some(next_keys)
        } else {
            // we've got more pair_slice to descend down...
            let remaining_slice = &pair_slice[1..];
            BigramIndex::next_possibles(&next_index_ref, remaining_slice)
        }

    }

    fn next_candidate_word(node: &BigramIndex, word: &mut PairString, filters: &Vec<HashSet<PairChar>>) -> Option<()> {
        Some(())
    }
}

