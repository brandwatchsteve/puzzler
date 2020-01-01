use std::cell::RefCell;
use std::collections::HashMap;
// use std::ops::{Index, IndexMut};

use super::types::{PairChar};
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

        for word in word_store.pairstring_words_by_length(size) {
            BigramIndex::index_word(&mut root, word.slice_from(0));
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

}

// impl Index<PairChar> for BigramIndex {
//     type Output = BigramIndexInner;
// 
//     fn index(&self, key: PairChar) -> &Self::Output {
//         // println!("Accessing {:?}-side of balance immutably", index);
//         &self.index[&key].unwrap().index
//     }
// }

// impl IndexMut<PairChar> for BigramIndex {
//     fn index_mut(&mut self, index: PairChar) -> &mut Self::Output {
//         &mut self.index
//     }
// }

