use std::collections::HashMap;

pub struct BigramIndex {
    index: HashMap<String, Option<Box<BigramIndex>>>,
}

impl BigramIndex {
    pub fn new(size: usize, word_store: &super::wordstore::WordStore) -> BigramIndex {
        let index: HashMap<String, Option<Box<BigramIndex>>> = HashMap::new();

        println!("Building index for size {}", size);

        BigramIndex { index }
    }
}
