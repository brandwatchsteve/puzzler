use clap::{App, Arg};
use puzzler;
use puzzler::bigramindex::BigramIndex;

fn main() {
    // parse commandline
    let matches = App::new("Puzzler")
        .version("0.1")
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging")
                .short("d")
                .multiple(true)
                .long("debug"),
        )
        .arg(
            Arg::with_name("dictionary")
                .help("Dictionary file to read from")
                .short("D")
                .takes_value(true)
                .default_value("/usr/share/dict/words")
                .long("dictionary"),
        )
        .arg(
            Arg::with_name("width")
                .help("grid width")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("depth")
                .help("grid depth")
                .required(true)
                .index(2),
        )
        .get_matches();

    let dictionary_file = matches.value_of("dictionary").unwrap();
    let puzzle_width: usize = matches.value_of("width").unwrap().parse::<usize>().unwrap();
    let puzzle_depth: usize = matches.value_of("depth").unwrap().parse::<usize>().unwrap();
    let debug: bool = matches.is_present("debug");

    // collect all of the source words, and store by length
    println!("Extracting Word List from {}", dictionary_file);
    let word_store = puzzler::generate_wordstore(dictionary_file);

    // generate two indices
    println!("Building Indices");
    let horizontal_index: BigramIndex = BigramIndex::build(puzzle_width, &word_store);
    if debug {
        horizontal_index.print("");
    }

    // only generate a real vertical index if the grid is not square
    let vertical_index = if puzzle_width == puzzle_depth {
        // allocate an empty instance just to simplify code flow
        BigramIndex::new(0)
    } else {
        BigramIndex::build(puzzle_depth, &word_store)
    };
    let vertical_index_ref = if puzzle_width == puzzle_depth {
        &horizontal_index
    } else {
        &vertical_index
    };

    // build the start words
    println!("Building Top Layer Candidates");
    let top_start_words = puzzler::generate_top_words(puzzle_width, &word_store, vertical_index_ref);

    // build out the puzzle_grid, building a second index if necessary
    println!("Populating the puzzle grid");
    let puzzle_grid = puzzler::populate_grid(
        puzzle_width,
        puzzle_depth,
        &top_start_words,
        &horizontal_index,
        vertical_index_ref,
    );

    // print out the grid if successful
    match puzzle_grid {
        // Some(p) => {p.print(); println!("{:?}",p.get_rows());},
        Some(p) => p.print(),
        None => {
            println!(
                "No matches found for size {}x{}",
                puzzle_width, puzzle_depth
            );
        }
    }
}
