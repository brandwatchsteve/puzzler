mod runpuzzler;

#[test]
fn four_by_four_solution() {
    assert!(runpuzzler::solve_puzzler("tests/words-good-4x4", 4, 4, 0));
}

#[test]
fn four_by_four_with_spaces_solution() {
    assert!(runpuzzler::solve_puzzler("tests/words-good-spaces-4x4", 4, 4, 2));
}

#[test]
fn no_repeats_rows() {
    assert!(!runpuzzler::solve_puzzler("tests/words-repeat-rows-5x5", 5, 5, 0));
}

#[test]
fn no_repeats_symmetric() {
    assert!(!runpuzzler::solve_puzzler("tests/words-symmetric-5x5", 5, 5, 0));
}
