pub mod bigramindex;
pub mod ingest;
pub mod puzzlegrid;
pub mod types;
pub mod wordstore;

// const FIRST_VAL: usize = 2;

// use types::{PairChar, PairString, WordList};
use bigramindex::BigramIndex;
use puzzlegrid::PuzzleGrid;

pub fn generate_wordstore(source_file: &str) -> wordstore::WordStore {
    let mut word_store = wordstore::WordStore::new();

    for word in ingest::read_even_words(source_file) {
        word_store.add(&word);
    }

    word_store
}

// Store possible words in a WordIterator
struct WordIterator {
    candidates: types::WordList,
    next: usize,
}

impl WordIterator {
    fn new(list: types::WordList) -> WordIterator {
        WordIterator {
            candidates: list,
            next: 0,
        }
    }
}

impl Iterator for WordIterator {
    type Item = types::PairString;

    fn next(&mut self) -> Option<types::PairString> {
        if self.next > self.candidates.len() {
            None
        } else {
            let return_val = self.candidates[self.next].clone();
            self.next += 1;
            Some(return_val)
        }
    }
}

// TODO use a result instead of an option for the return wrapper
pub fn populate_grid(
    width: usize,
    height: usize,
    word_store: &wordstore::WordStore,
    horizontal_index: &bigramindex::BigramIndex,
    vertical_index: &bigramindex::BigramIndex,
) -> Option<puzzlegrid::PuzzleGrid> {
    // Identify possible start words for a given size
    // let top_start_words: &types::WordList = word_store.words_by_length(width);
    let top_start_words: types::WordList = word_store.words_by_length(width).clone();

    // generate a mutable puzzlegrid, to hold the words
    let mut puzzle_grid: puzzlegrid::PuzzleGrid = puzzlegrid::PuzzleGrid::new(width, height);
    // if populate_layer(
    //     &mut puzzle_grid,
    //     WordIterator::new(top_start_words),
    //     horizontal_index,
    //     vertical_index,
    // ) {
    //     return Some(puzzle_grid);
    // }
    for word in WordIterator::new(top_start_words) {
        puzzle_grid.add_layer(&word);

        let stems = puzzle_grid.get_stems();
        let possibles = vertical_index.get_possibles(stems);
        let candidate_words = BigramIndex::get_candidate_words(horizontal_index, &possibles);

        if let Some(v) = candidate_words {
            puzzle_grid.add_layer(&v[0]);
            return Some(puzzle_grid);
        }

        puzzle_grid.remove_layer();
    }

    None
}

fn populate_layer(
    puzzle_grid: &mut PuzzleGrid,
    word_list: WordIterator,
    horizontal_index: &BigramIndex,
    vertical_index: &BigramIndex,
) -> bool {
    for word in word_list {
        puzzle_grid.add_layer(&word);

        if puzzle_grid.is_complete() {
            return true;
        };

        let stems = puzzle_grid.get_stems();
        let possibles = vertical_index.get_possibles(stems);
        let candidate_words = BigramIndex::get_candidate_words(horizontal_index, &possibles);

        if let Some(v) = candidate_words {
            let word_iterator = WordIterator::new(v);
            if populate_layer(puzzle_grid, word_iterator, horizontal_index, vertical_index) {
                return true;
            }
        };

        puzzle_grid.remove_layer();
    }

    false
}
