use puzzler::ingest;
use puzzler::wordstore::{WordStore, PairString};
use puzzler::bigramindex::BigramIndex;
use puzzler::puzzlegrid::PuzzleGrid;

const SOURCE_FILE: &str = "/usr/share/dict/words";
const PUZZLE_WIDTH: usize = 3;
const PUZZLE_HEIGHT: usize = 4;

fn main() {
    // collect all of the source words, and store by length
    let word_store = generate_wordstore(SOURCE_FILE);

    // generate two indices
    let horizontal_index: BigramIndex = BigramIndex::new(PUZZLE_WIDTH, &word_store);

    // build out the puzzle_grid, building a second index if necessary
    let puzzle_grid = match PUZZLE_WIDTH == PUZZLE_HEIGHT {
        true  => populate_grid(PUZZLE_WIDTH, PUZZLE_HEIGHT, &word_store, &horizontal_index, &horizontal_index),
        false => {
            let vertical_index: BigramIndex = BigramIndex::new(PUZZLE_HEIGHT, &word_store);
            populate_grid(PUZZLE_WIDTH, PUZZLE_HEIGHT, &word_store, &horizontal_index, &vertical_index)
        }
    };

    // print out the grid if successful
    match puzzle_grid {
        Some(p) => p.print(),
        None    => { println!("No matches found for size {}x{}", PUZZLE_WIDTH, PUZZLE_HEIGHT); }
    }
}

fn generate_wordstore(source_file: &str) -> WordStore {
    let mut word_store = WordStore::new();

    for word in ingest::read_even_words(source_file) {
        word_store.add(&word);
    }

    word_store
}

fn populate_grid(width: usize, height: usize, word_store: &WordStore,
                 _horizontal_index: &BigramIndex, _vertical_index: &BigramIndex) -> Option<PuzzleGrid> {
    // generate a mutable puzzlegrid, to hold the words
    let mut puzzle_grid: PuzzleGrid = PuzzleGrid::new(width, height);

    // Identify possible start words for a given size
    let top_start_words: &Vec<PairString> = word_store.pairstring_words_by_length(width-1);
    let _left_start_words: &Vec<PairString> = word_store.pairstring_words_by_length(height-1);

    puzzle_grid.insert_horizontal(0, &top_start_words[0]);

    Some(puzzle_grid)
}

