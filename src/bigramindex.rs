use std::collections::{HashMap, HashSet};

use super::types::{PairChar, PairString};
use super::wordstore::WordStore;

pub struct BigramIndexTree {
    index: BigramIndexTreeInner,
    depth: usize,
}
type BigramIndexTreeInner = HashMap<PairChar, Option<Box<BigramIndexTree>>>;

impl BigramIndexTree {
    pub fn new(depth: usize) -> BigramIndexTree {
        let index: BigramIndexTreeInner = HashMap::new();

        BigramIndexTree { index, depth }
    }

    fn add_leaf(&mut self, key: PairChar) {
        let new_leaf = BigramIndexTree::new(self.depth + 1);
        self.index.insert(key, Some(Box::new(new_leaf)));
    }

    pub fn print(&self, prefix: &str) {
        for (key, leaf) in &self.index {
            let word = format!("{}-{}", prefix, &key.decode());
            match leaf {
                Some(l) => {
                    l.print(&word);
                }
                None => println!("{}", word),
            }
        }
    }

    // populate the index based on a series of pairwords
    pub fn build(size: usize, word_store: &WordStore, max_spaces: usize) -> BigramIndexTree {
        let mut root = BigramIndexTree::new(0);

        for word in word_store.permuted_words_by_length(size, max_spaces) {
            BigramIndexTree::index_word(&mut root, word.slice());
        }

        // return the populated index
        root
    }

    // recursive function to create the index tree, as used by build
    fn index_word(node: &mut BigramIndexTree, pair_slice: &[PairChar]) {
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

                // unbox the index to add to it, in order to avoid interior mutability woes
                // TODO: Find a way to use the mutable ref from get_mut directly...
                let mut child_node: BigramIndexTree =
                    *(node.index.get_mut(&key_char).unwrap().take().unwrap());

                BigramIndexTree::index_word(&mut child_node, remaining_slice);

                // compact the index
                // TODO: Check that this is actually worth doing
                child_node.index.shrink_to_fit();

                // reinsert the modified child
                node.index.insert(*key_char, Some(Box::new(child_node)));
            }
        }
    }

    // create a hashset containing all of the possible next characters for a given set of stems
    pub fn get_possible_pairchars(&self, stems: Vec<&[PairChar]>) -> Vec<HashSet<PairChar>> {
        let mut possible_chars: Vec<HashSet<PairChar>> = Vec::new();

        for stem in stems {
            let new_possibles = match BigramIndexTree::next_possible_pairchars(self, stem) {
                Some(v) => v,
                None => HashSet::new(),
            };
            possible_chars.push(new_possibles);
        }

        possible_chars
    }

    // recursively descend the tree structure
    // returning Some<HashSet> of keys if we can travel down the tree, None if we run out of tree
    fn next_possible_pairchars(node: &BigramIndexTree, stem: &[PairChar]) -> Option<HashSet<PairChar>> {
        // check to see that we haven't descended too far
        if node.depth >= stem.len() {
            panic!(
                "we've got a slice of length {} at an index of depth {}",
                stem.len(),
                node.depth
            );
        }
        let key_char = &stem[node.depth];

        if node.index.get(&key_char).is_none() {
            // we've hit the end of the indexchain before we've run out of word
            // meaning there are no subsequent next_possible_pairchars
            return None;
        }

        let next_index_ref = match node.index.get(&key_char).expect("Key Not Found").as_ref() {
            None => {
                panic!("We've hit a none in the index at depth {}", node.depth);
            }
            Some(v) => v,
        };

        let is_last_char = stem.len() - 1 == node.depth;
        if is_last_char {
            // return the keys in the last node as a HashSet
            Some(next_index_ref.get_keys_as_hashset())
        } else {
            // we've got more stem to descend down...
            BigramIndexTree::next_possible_pairchars(&next_index_ref, stem)
        }
    }

    pub fn get_keys_as_hashset(&self) -> HashSet<PairChar> {
        let mut key_set: HashSet<PairChar> = HashSet::new();
        for key in self.index.keys() {
            key_set.insert(key.clone());
        }
        key_set
    }

    pub fn get_candidate_words(
        root_index_node: &BigramIndexTree,
        filters: &[HashSet<PairChar>],
    ) -> Option<Vec<PairString>> {
        // extract the possible words derived from a BigramIndexTree and a set of filters for each
        // depth

        // first make sure that all of our filters can match something, immediately return None
        // otherwise
        for filter in filters {
            if filter.is_empty() {
                return None;
            }
        }

        // descend the index tree with the set filters to find possible matches
        let reverse_words =
            match BigramIndexTree::get_reversed_candidate_words(root_index_node, filters, 0) {
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
        index_node: &BigramIndexTree,
        filters: &[HashSet<PairChar>],
        depth: usize,
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
            BigramIndexTree::pairchar_intersection(index_node, &filters[index_node.depth]);

        if intersection.is_empty() {
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
                let next_index_ref = index_node.index[&key_char].as_ref().unwrap();
                let partial_words = match BigramIndexTree::get_reversed_candidate_words(
                    &next_index_ref,
                    filters,
                    depth + 1,
                ) {
                    Some(v) => v,
                    None => {
                        continue;
                    }
                };

                for word in partial_words {
                    let mut new_word: PairString = word.clone();
                    new_word.push(key_char);
                    reversed_words.push(new_word);
                }
            }
        }

        if !reversed_words.is_empty() {
            Some(reversed_words)
        } else {
            None
        }
    }

    fn pairchar_intersection(
        index_node: &BigramIndexTree,
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
