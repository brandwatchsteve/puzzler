# Puzzler 2019

## Overview

This is my entry for the Xmas 2019 puzzler - it's working parallel rust-based implementation but I've only been able to
run it up to 7x8 (taking between 1 and 2.5 hours on my laptop).

## Running

Prebuilt binaries for Mac and Linux (amd64) can be found in the "bin" directory at the top level. You will need to specify
the width and depth of the grid on each run, and there is help text (if you run it with -h or with missing arguments).

    puzzler <width> <depth>

Output will be something like

    rudor:release steve$ time ./puzzler 7 7
    Extracting Word List from /usr/share/dict/words
    Building Indices
    Building Top Layer Candidates
    Populating the puzzle grid
    __ __ in __ th ru st
    ot __ he ri __ st __
    __ un __ de re __ at
    oc __ re __ at __ ed
    __ re nt ab __ le __
    ra ck __ __ le ss __
    ne ed __ le ss __ ly

    real	2m22.879s
    user	8m34.226s
    sys	0m1.113s

Run time is variable, as the HashMaps and HashSets used in the program are not ordered. On a modern MacBook Pro 6x6 grids
take under 10s and 7x7 grids take under 150s.

## Approach

The program runs in four stages:

  - extract the words from the specified dictionary (/usr/share/dict/words by default)
  - generate the BigramIndexTrees for the horizontal and if different vertical axes
  - generate a list of valid words for the first row of the grid
  - solve the grid row-by-row (top-down) until either a solution is found or all possible words have been exhausted

The last step (the row-by-row solving) proceeds as:

  - using the existing grid columns and the vertical index,  establish the potential next characters for each column of the grid
  - using the array of possible characters and the horizontal index, establish possible words for the next layer of the grid
  - repeat the process for each candidate word in turn at the next layer of the grid, backtracking if the candidate words are
    exhausted

At the heart of this program we have a BigramIndexTree, which is a tree formed by rust HashMaps, each pointing to either
Some(child-tree) or None if it's the last node in the tree. The words in the tree are encoded by the sequence of keys, and all
branches of the tree should have equal length.

Blank characters are explicitly encoded into the indexes, the start-words and the grid. This means the indices can become very large
(to 2.5GB when trying to solve an 8x9 grid). There are command line options to test with fewer or no blank characters.

Short cuts used in the program are:

  - only using words of even length
  - limiting start words to only be those whose character pairs are valid starting pairs (eg. exclude bu-zz as there are no English words
    starting with "zz").

I've not removed the words with unique character pairs, and I've not set it up to verify characters on the left-side word.

## Missing Stuff and To-Dos

AFAICT this is feature complete now.

The main things yet to do here are:

  - add some unit tests...
  - filter for valid start character on first non-blank on rows and columns
  - investigate doing the blanks in the descent of the tree, rather than storing each at index time
  - make the whole code more "rustic" (iterators instead of reslicing vecs, traits instead of containers, using Result type, using iterators rather than for loops)

## Build Instructions

If you have rust installed, you can run this using:

    cargo run -- <width> <depth>

Building is just a case of:

    cargo build --release

(which will copy the built binary to ./target/release/puzzler).

The program does require two external crates (Rayon for parallel running, and Clap for the command-line processing), so an
internet connection will be required the first time this is built.





