use super::types::{PairChar, PairString, WordIterator};
use std::collections::HashSet;
use super::bigramindex::{BigramIndexTree};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Default, Debug)]
pub struct PuzzleGrid {
    columns: Vec<Vec<PairChar>>,
    next_layer: usize,
    width: usize,
    depth: usize,
}

impl PuzzleGrid {
    pub fn new(width: usize, depth: usize) -> PuzzleGrid {
        PuzzleGrid {
            columns: vec![vec![PairChar::encode(b'u', b'u'); depth]; width],
            next_layer: 0,
            width,
            depth,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.next_layer >= self.depth
    }

    // insert a word into the puzzlegrid
    // TODO: replace the Option with a Result
    pub fn add_layer(&mut self, word: &PairString) -> Option<()> {
        if self.next_layer >= self.depth {
            println!("Depth too large");
            return None;
        }

        if word.len() != self.width {
            println!("Word too long: columns={} word={}", self.width, word.len());
            return None;
        }

        for i in 0..(self.columns.len()) {
            self.columns[i][self.next_layer] = word.pair_string[i];
        }

        self.next_layer += 1;

        Some(())
    }

    pub fn remove_layer(&mut self) {
        if self.next_layer > 0 {
            self.next_layer -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.next_layer = 0;
    }

    pub fn get_rows(&self) -> Vec<Vec<PairChar>> {
        let mut rows: Vec<Vec<PairChar>> = vec![Vec::new(); self.next_layer];
        for column in &self.columns {
            // &column.iter().enumerate().for_each(|x| rows[x.0].push(*x.1));
            for i in 0..self.next_layer {
                let pairchar = column[i];
                if !pairchar.is_spacer() {
                    rows[i].push(pairchar);
                }
            }
        }

        rows
    }

    pub fn get_columns(&self) -> Vec<&[PairChar]> {
        let mut return_val: Vec<&[PairChar]> = Vec::new();

        for column in &self.columns {
            return_val.push(&column[0..self.next_layer]);
        }

        return_val
    }

    pub fn has_duplicates(&self) -> bool {
        let mut match_set: HashSet<Vec<PairChar>> = HashSet::new();

        for column in self.get_columns() {
            let compact_word = PuzzleGrid::clone_without_spacers(column);
            if match_set.contains(&compact_word) {
                return true;
            } else {
                match_set.insert(compact_word);
            }
        }

        for row in self.get_rows() {
            let compact_word = PuzzleGrid::clone_without_spacers(&row);
            if match_set.contains(&compact_word) {
                return true;
            } else {
                match_set.insert(compact_word);
            }
        }

        false
    }

    fn clone_without_spacers(pairchar_slice: &[PairChar]) -> Vec<PairChar> {
        let mut pair_vec: Vec<PairChar> = Vec::new();
        for pairchar in pairchar_slice {
            if !pairchar.is_spacer() {
                pair_vec.push(pairchar.clone());
            }
        }
        pair_vec
    }

    pub fn print(&self) {
        for y in 0..(self.depth) {
            for x in 0..(self.width) {
                print!("{} ", self.columns[x][y].decode());
            }
            println!();
        }
    }

    // recursion function for populate_grid
    pub fn populate_layer(
        &mut self,
        word: &PairString,
        depth: usize,
        horizontal_index: &BigramIndexTree,
        vertical_index: &BigramIndexTree,
        continue_running: Option<&AtomicBool>,
    ) -> bool {
        // check whether to continue loop (only bother for the two highest levels)
        if depth <= 1
            && continue_running.is_some()
            && !continue_running.unwrap().load(Ordering::Relaxed)
        {
            return false;
        }

        self.add_layer(word);

        if self.is_complete() {
            // if we've found a duplicate discard this solution, continue checking possibles
            if self.has_duplicates() {
                self.remove_layer();
                return false;
            }

            // else this true should propagate up through the call stack, and complete the run
            return true;
        };

        let column_stems = self.get_columns();
        let current_rows = self.get_rows();

        let possible_pairchars = vertical_index.get_possible_pairchars(column_stems, current_rows);
        let candidate_words = BigramIndexTree::get_candidate_words(horizontal_index, &possible_pairchars);

        // recurse down if we have candidate words to check
        if let Some(v) = candidate_words {
            let word_iterator = WordIterator::new(v);
            for word in word_iterator {
                if self.populate_layer(
                    &word,
                    depth + 1,
                    horizontal_index,
                    vertical_index,
                    continue_running,
                ) {
                    return true;
                }
            }
        };

        self.remove_layer();

        false
    }
}

