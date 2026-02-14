pub struct Dictionary {
    words: Vec<String>
}

impl Dictionary {
    pub fn new(path: &str) -> Self {
        let contents = std::fs::read_to_string(path).expect("Failed to read dictionary file");
        let words = contents.lines().map(|line| line.trim().to_string()).collect();
        Dictionary {
            words
        }
    }
    
    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }
}
