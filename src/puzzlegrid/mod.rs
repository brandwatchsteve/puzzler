use super::types::{PairChar, PairString};

#[derive(Clone,Default,Debug)]
pub struct PuzzleGrid {
    grid: Vec<Vec<PairChar>>,
}

impl PuzzleGrid {
    pub fn new(x: usize, y: usize) -> PuzzleGrid {
        let mut grid: Vec<Vec<PairChar>> = Vec::new();

        for _ in 0..y {
            grid.push(vec![PairChar::new(); x]);
        }

        PuzzleGrid { grid }
    }

    // insert a word into the puzzlegrid
    // TODO: replace the Option with a Result
    pub fn insert_horizontal(&mut self, pos: usize, word: &PairString) -> Option<()> {
        if pos >= self.grid.len() {
            println!("Position too large");
            return None;
        }

        if word.len() != self.grid[pos].len() {
            println!("Word too large: grid={} but word={}", self.grid[pos].len(), word.len());
            return None;
        }

        for i in 0..(self.grid[pos].len()) {
            self.grid[pos][i] = word.pair_string[i];
            println!("Setting [{}, {}] to {:?}", pos, i, word.pair_string[i]);
        }

        Some(())
    }

    // insert a word into the puzzlegrid
    // TODO: replace the Option with a Result
    pub fn insert_vertical(&mut self, pos: usize, word: &PairString) -> Option<()> {
        if pos >= self.grid[0].len() {
            println!("Position too large");
            return None;
        }

        if word.len() != self.grid.len() {
            println!("Word too long: grid={} word={}", self.grid.len(), word.len());
            return None;
        }

        for i in 0..(self.grid.len()) {
            self.grid[i][pos] = word.pair_string[i];
            println!("Setting [{}, {}] to {:?}", i, pos, word.pair_string[i]);
        }

        Some(())
    }

    pub fn print(&self) {
        for i in 0..(self.grid.len()) {
            for j in 0..(self.grid[i].len()) {
                print!("{} ", self.grid[i][j].decode());
            }
            println!("");
        }
    }
}


