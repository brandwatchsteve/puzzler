use super::types::{PairChar, PairString};

#[derive(Clone, Default, Debug)]
pub struct PuzzleGrid {
    columns: Vec<Vec<PairChar>>,
    next_layer: usize,
    width: usize,
    depth: usize,
}

impl PuzzleGrid {
    pub fn new(width: usize, depth: usize) -> PuzzleGrid {
        PuzzleGrid {
            // columns: vec![PairString::build(depth); width],
            columns: vec![vec![PairChar::encode(b'u', b'u'); depth]; width],
            next_layer: 0,
            width,
            depth,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.next_layer > self.depth
    }

    // insert a word into the puzzlegrid
    // TODO: replace the Option with a Result
    pub fn add_layer(&mut self, word: &PairString) -> Option<()> {
        if self.next_layer >= self.depth {
            println!("Depth too large");
            return None;
        }

        if word.len() != self.width {
            println!("Word too long: columns={} word={}", self.width, word.len());
            return None;
        }

        for i in 0..(self.columns.len()) {
            self.columns[i][self.next_layer] = word.pair_string[i];
            println!("Setting [{}, {}] to {:?}", i, self.next_layer, word.pair_string[i]);
        }

        self.next_layer += 1;

        Some(())
    }

    pub fn remove_layer(&mut self) {
        if self.next_layer > 0 {
            self.next_layer -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.next_layer = 0;
    }

    pub fn get_stems(&self) -> Vec<&[PairChar]> {
        let mut return_val: Vec<&[PairChar]> = Vec::new();

        for column in &self.columns {
            return_val.push(&column[0..self.next_layer]);
        }

        return_val
    }

    pub fn print(&self) {
        for y in 0..(self.depth) {
            for x in 0..(self.width) {
                print!("{} ", self.columns[x][y].decode());
            }
            println!("");
        }
    }
}
