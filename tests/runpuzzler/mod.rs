use puzzler;
use puzzler::bigramindex::BigramIndexTree;

pub fn solve_puzzler(dictionary_file: &str, puzzle_width: usize, puzzle_depth: usize, spaces: usize) -> bool {
    let word_store = puzzler::generate_wordstore(dictionary_file);

    let horizontal_index: BigramIndexTree = BigramIndexTree::build(puzzle_width, &word_store, spaces);

    let top_start_words = puzzler::generate_top_words(puzzle_width, &word_store, spaces, &horizontal_index);

    let grid = puzzler::populate_grid(puzzle_width, puzzle_depth, &top_start_words, &horizontal_index, &horizontal_index);

    match grid {
        Some(_p) => true,
        None => false,
    }
}
