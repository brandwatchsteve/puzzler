pub mod bigramindex;
pub mod ingest;
pub mod puzzlegrid;
pub mod types;
pub mod wordstore;


pub fn generate_wordstore(source_file: &str) -> wordstore::WordStore {
    let mut word_store = wordstore::WordStore::new();

    for word in ingest::read_even_words(source_file) {
        word_store.add(&word);
    }

    word_store
}

pub fn populate_grid(width: usize, height: usize, word_store: &wordstore::WordStore,
                 _horizontal_index: &bigramindex::BigramIndex, _vertical_index: &bigramindex::BigramIndex)
    -> Option<puzzlegrid::PuzzleGrid> {
    // generate a mutable puzzlegrid, to hold the words
    let mut puzzle_grid: puzzlegrid::PuzzleGrid = puzzlegrid::PuzzleGrid::new(width, height);

    // Identify possible start words for a given size
    let top_start_words: &types::WordList = word_store.pairstring_words_by_length(width);
    let _left_start_words: &types::WordList = word_store.pairstring_words_by_length(height);

    puzzle_grid.insert_horizontal(0, &top_start_words[0]);

    Some(puzzle_grid)
}
