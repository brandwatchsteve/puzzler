pub mod bigramindex;
pub mod ingest;
pub mod puzzlegrid;
pub mod types;
pub mod wordstore;

use bigramindex::BigramIndex;
use puzzlegrid::PuzzleGrid;
use types::{PairString, WordList, WordIterator};
use wordstore::WordStore;

use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub fn generate_wordstore(source_file: &str) -> wordstore::WordStore {
    let mut word_store = wordstore::WordStore::new();

    for word in ingest::read_even_words(source_file) {
        word_store.add(&word);
    }

    word_store
}

fn generate_top_words(width: usize, word_store: &WordStore, index: &BigramIndex) -> WordList {
    // not all words are valid on the top line, only those whose pairchars are all valid
    // starting pairchars of other words,
    // eg. there's no English word starting "ZZ" so we can immediately rule out bu-zz from the top row

    let mut result_set = WordList::new();
    let first_character_set = index.get_keys_as_hashset();

    // could possibly speed this up by permuting the words afterwards
    'outer: for candidate_word in word_store.permuted_words_by_length(width).clone() {
        'inner: for pairchar in candidate_word.slice() {
            if !first_character_set.contains(pairchar) {
                continue 'outer;
            }
        }
        result_set.push(candidate_word);
    }

    result_set
}

// TODO use a result instead of an option for the return wrapper
pub fn populate_grid(
    width: usize,
    height: usize,
    word_store: &WordStore,
    horizontal_index: &BigramIndex,
    vertical_index: &BigramIndex,
) -> Option<PuzzleGrid> {
    // Identify possible start words for a given size
    let top_start_words: WordList = generate_top_words(width, &word_store, vertical_index);

    let continue_running = AtomicBool::new(true);
    let puzzle_arc = Arc::new(Mutex::<Option<PuzzleGrid>>::new(None));

    top_start_words.par_iter().for_each(|x| {
        let mut puzzle_grid: PuzzleGrid = PuzzleGrid::new(width, height);

        if continue_running.load(Ordering::Relaxed) {
            let found_result = populate_layer(
                &mut puzzle_grid,
                &x,
                0,
                horizontal_index,
                vertical_index,
                Some(&continue_running),
            );
            if found_result {
                continue_running.store(false, Ordering::Relaxed);

                // assign the found puzzlegrid into the puzzle_mutex
                let puzzle_mutex = puzzle_arc.clone();
                let mut puzzle_guard = puzzle_mutex.lock().unwrap();
                *puzzle_guard = Some(puzzle_grid);
            }
        }
    });

    let puzzle_mutex = puzzle_arc.clone();
    let mut puzzle_guard = puzzle_mutex.lock().unwrap();
    (*puzzle_guard).take()
}

fn populate_layer(
    puzzle_grid: &mut PuzzleGrid,
    word: &PairString,
    depth: usize,
    horizontal_index: &BigramIndex,
    vertical_index: &BigramIndex,
    continue_running: Option<&AtomicBool>,
) -> bool {
    // check whether to continue loop on the two highest levels
    if depth <= 1
        && continue_running.is_some()
        && !continue_running.unwrap().load(Ordering::Relaxed)
    {
        return false;
    }

    puzzle_grid.add_layer(word);

    if puzzle_grid.is_complete() {
        return true;
    };

    let stems = puzzle_grid.get_stems();
    let possibles = vertical_index.get_possibles(stems);
    let candidate_words = BigramIndex::get_candidate_words(horizontal_index, &possibles);

    if let Some(v) = candidate_words {
        // check out the lower levels
        let word_iterator = WordIterator::new(v);
        for word in word_iterator {
            if populate_layer(
                puzzle_grid,
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

    puzzle_grid.remove_layer();

    false
}
