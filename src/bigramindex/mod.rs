use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
// use std::ops::{Index, IndexMut};

use super::types::{PairChar, PairString};
use super::wordstore::WordStore;

pub struct BigramIndex {
    index: BigramIndexInner,
    depth: usize,
}
type BigramIndexInner = HashMap<PairChar, Option<RefCell<BigramIndex>>>;

impl BigramIndex {
    pub fn new(depth: usize) -> BigramIndex {
        let index: BigramIndexInner = HashMap::new();

        BigramIndex { index, depth }
    }

    fn add_leaf(&mut self, key: PairChar) {
        let new_leaf = BigramIndex::new(self.depth + 1);
        self.index.insert(key, Some(RefCell::new(new_leaf)));
    }

    pub fn print(&self, prefix: &str) {
        for (key, leaf) in &self.index {
            let word = format!("{}-{}", prefix, &key.decode());
            match leaf {
                Some(l) => {
                    l.borrow().print(&word);
                }
                None => println!("{}", word),
            }
        }
    }

    // use a wordlist...
    pub fn build(size: usize, word_store: &WordStore) -> BigramIndex {
        let mut root = BigramIndex::new(0);

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
            1 => {
                node.index.insert(*key_char, None);
            }
            _ => {
                if !node.index.contains_key(key_char) {
                    node.add_leaf(*key_char);
                };

                // insert the rest of the pair_slice
                let remaining_slice = &pair_slice[1..];
                BigramIndex::index_word(
                    &mut node.index[&key_char].as_ref().unwrap().borrow_mut(),
                    remaining_slice,
                );
            }
        }
    }

    pub fn get_possibles(&self, pair_slices: Vec<&[PairChar]>) -> Vec<HashSet<PairChar>> {
        let mut possible_chars: Vec<HashSet<PairChar>> = Vec::new();

        for slice in pair_slices {
            let new_possibles = match BigramIndex::next_possibles(self, slice) {
                Some(v) => v,
                None => HashSet::new(),
                // None    => HashSet::new::<PairChar>(),
            };
            possible_chars.push(new_possibles);
        }

        possible_chars
    }

    fn next_possibles(node: &BigramIndex, pair_slice: &[PairChar]) -> Option<HashSet<PairChar>> {
        let key_char = &pair_slice[0];

        let last_char = pair_slice.len() == 1;

        if let None = node.index.get(&key_char) {
            // we've hit the end of the indexchain before we've run out of word
            // meaning there are no subsequent next_possibles
            return None;
        }

        // let next_index_ref = node.index[&key_char].as_ref().unwrap().borrow();
        let next_index_ref = match node
            .index
            .get(&key_char)
            .expect("Key Not Found")
            .as_ref() {
                None => { return None; },
                Some(v) => v.borrow(),
            };
        if last_char {
            // we've got to the end of our sequence
            //
            // extract the keys for that index
            let mut next_keys: HashSet<PairChar> = HashSet::new();
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

    pub fn get_candidate_words(
        root_index_node: &BigramIndex,
        filters: &Vec<HashSet<PairChar>>,
    ) -> Option<Vec<PairString>> {
        // extract the possible words derived from a BigramIndex and a set of filters for each
        // depth

        // first make sure that all of our filters can match something, immediately return None
        // otherwise
        for filter in filters {
            if filter.len() == 0 {
                return None;
            }
        }

        let reverse_words =
            match BigramIndex::get_reversed_candidate_words(root_index_node, filters) {
                Some(v) => v,
                None => {
                    return None;
                }
            };

        let mut forward_words: Vec<PairString> = Vec::new();
        for word in reverse_words {
            let mut new_word = word.clone();
            new_word.reverse();
            forward_words.push(new_word);
        }

        Some(forward_words)
    }

    fn get_reversed_candidate_words(
        index_node: &BigramIndex,
        filters: &Vec<HashSet<PairChar>>,
    ) -> Option<Vec<PairString>> {
        // check that all of the filter sets have some characters at least,
        // return early if any are None
        let filters_length = filters.len();
        if index_node.depth >= filters_length {
            panic!(
                "Filters length does not match horizontal index depth: {} vs {}",
                filters_length, index_node.depth
            );
        }

        let mut reversed_words: Vec<PairString> = Vec::new();
        let intersection =
            BigramIndex::pairchar_intersection(index_node, &filters[index_node.depth]);

        if intersection.len() == 0 {
            // no available matches at this depth, meaning we've not got any matches for this
            // branch
            return None;
        }

        if index_node.depth == (filters_length - 1) {
            // we've hit the last node in the index

            for pairchar in intersection {
                let mut initial_word: PairString = PairString::new();
                initial_word.push(pairchar);

                reversed_words.push(initial_word);
            }
        } else {
            // we're at an intermediate layer, so recurse down
            // for pairchar in intersection {
            for key_char in intersection {
                let next_index_ref = index_node.index[&key_char].as_ref().unwrap().borrow();
                let partial_words =
                    match BigramIndex::get_reversed_candidate_words(&next_index_ref, filters) {
                        Some(v) => v,
                        None => {
                            return None;
                        }
                    };

                for word in partial_words {
                    let mut new_word: PairString = word.clone();
                    new_word.push(key_char);
                    reversed_words.push(new_word);
                }
            }
        }

        Some(reversed_words)
    }

    fn pairchar_intersection(
        index_node: &BigramIndex,
        filter_set: &HashSet<PairChar>,
    ) -> Vec<PairChar> {
        // return a vec of PairChars which match both the index and the filter_set
        let mut return_set: Vec<PairChar> = Vec::new();
        for key in index_node.index.keys() {
            if filter_set.contains(key) {
                return_set.push(*key);
            }
        }

        return_set
    }
}
