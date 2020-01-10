use puzzler;
use puzzler::bigramindex::BigramIndex;
use clap::{App, Arg};


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

    // build out the puzzle_grid, building a second index if necessary
    let puzzle_grid = match puzzle_width == puzzle_depth {
        true => puzzler::populate_grid(
            puzzle_width,
            puzzle_depth,
            &word_store,
            &horizontal_index,
            &horizontal_index,
        ),
        false => {
            let vertical_index: BigramIndex = BigramIndex::build(puzzle_depth, &word_store);
            if debug {
                vertical_index.print("");
            }
            println!("Populating grid");
            puzzler::populate_grid(
                puzzle_width,
                puzzle_depth,
                &word_store,
                &horizontal_index,
                &vertical_index,
            )
        }
    };

    // print out the grid if successful
    match puzzle_grid {
        Some(p) => p.print(),
        None => {
            println!(
                "No matches found for size {}x{}",
                puzzle_width, puzzle_depth
            );
        }
    }
}
