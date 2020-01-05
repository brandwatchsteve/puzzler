pub mod bigramindex;
pub mod ingest;
pub mod puzzlegrid;
pub mod types;
pub mod wordstore;

use types::{PairChar, PairString, WordList};

pub fn generate_wordstore(source_file: &str) -> wordstore::WordStore {
    let mut word_store = wordstore::WordStore::new();

    for word in ingest::read_even_words(source_file) {
        word_store.add(&word);
    }

    word_store
}

// store the stems for each column
struct CandidateStems {
    stems: Vec<PairString>,
    next_layer: usize,
    width: usize,
}

impl CandidateStems {
    fn new(width: usize, length: usize) -> CandidateStems {
        CandidateStems {
            stems: vec!(PairString::build(length); width),
            next_layer: 0,
            width,
        }
    }

    fn add_layer(&mut self, new_row: &PairString) {
        // check that the layer is correctly sized
        if new_row.len() != self.width {
            panic!("Trying to insert too long a character: {} into {}", new_row.len(), self.width);
        }

        // overwrite the characters at this level
        for (column, pairchar) in new_row.pair_string.iter().enumerate() {
            let mut new_string : PairString = PairString::new();
            // TODO: make sure this works with zero layer
            let new_slice : &[PairChar] = self.stems[column].slice_to(self.next_layer);
            for new_char in new_slice {
                new_string.push(new_char.clone());
            }
            new_string.push(pairchar.clone());
            self.stems[column] = new_string;
        }

        // bump up the layer pointer
        self.next_layer += 1;
    }

    fn remove_layer(&mut self) {
        if self.next_layer > 0 {
            self.next_layer -= 1;
        }
    }

    fn get_next_row_words(&self, vertical_index: &bigramindex::BigramIndex) -> WordIterator {
        let mut wordlist = WordList::new();

        // here's the heart of the 

        WordIterator::new(wordlist)
    }
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
pub fn populate_grid(width: usize, height: usize, word_store: &wordstore::WordStore,
                 _horizontal_index: &bigramindex::BigramIndex, _vertical_index: &bigramindex::BigramIndex)
    -> Option<puzzlegrid::PuzzleGrid> {
    // generate a mutable puzzlegrid, to hold the words
    let mut puzzle_grid: puzzlegrid::PuzzleGrid = puzzlegrid::PuzzleGrid::new(width, height);

    // Identify possible start words for a given size
    let top_start_words: &types::WordList = word_store.words_by_length(width);
    let _left_start_words: &types::WordList = word_store.words_by_length(height);

    puzzle_grid.insert_horizontal(0, &top_start_words[0]);
    // build a list of candidate words for the horizontal
    //   - first row from the start words
    //   - subsequent rows from the list of candidate characters in the vertical indices
    //   - if no suitable word is found, pick the next word from the preceeding level, and continue
    //   - proceed to next lowest level if candidate is found
    //   - return grid if last layer is reached, otherwise return a failure

    Some(puzzle_grid)
}
