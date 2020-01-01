use puzzler;
// use puzzler::wordstore::{WordStore, PairString};
use puzzler::bigramindex::BigramIndex;
// use puzzler::puzzlegrid::PuzzleGrid;

const SOURCE_FILE: &str = "/usr/share/dict/words";
const PUZZLE_WIDTH: usize = 3;
const PUZZLE_HEIGHT: usize = 4;

fn main() {
    // collect all of the source words, and store by length
    let word_store = puzzler::generate_wordstore(SOURCE_FILE);

    // generate two indices
    let horizontal_index: BigramIndex = BigramIndex::build(PUZZLE_WIDTH, &word_store);
    horizontal_index.print("");

    // build out the puzzle_grid, building a second index if necessary
    let puzzle_grid = match PUZZLE_WIDTH == PUZZLE_HEIGHT {
        true  => puzzler::populate_grid(PUZZLE_WIDTH, PUZZLE_HEIGHT, &word_store, &horizontal_index, &horizontal_index),
        false => {
            let vertical_index: BigramIndex = BigramIndex::build(PUZZLE_HEIGHT, &word_store);
            puzzler::populate_grid(PUZZLE_WIDTH, PUZZLE_HEIGHT, &word_store, &horizontal_index, &vertical_index)
        }
    };

    // print out the grid if successful
    match puzzle_grid {
        Some(p) => p.print(),
        None    => { println!("No matches found for size {}x{}", PUZZLE_WIDTH, PUZZLE_HEIGHT); }
    }
}
