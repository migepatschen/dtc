use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Column {
    letter: char,
    index: usize,
    value: String,
}

impl Column {
    pub fn from_str(key: &str) -> Vec<Self> {
        let sorted_key: String = key.chars().sorted().collect();
        let mut columns: Vec<Self> = Vec::with_capacity(key.len());
        for c in key.chars() {
            let index = sorted_key.find(c).expect("the letter should be in the key");
            columns.push(Column {
                letter: c,
                index,
                value: String::new(),
            });
        }
        columns
    }

    pub fn add(&mut self, c: char) {
        self.value.push(c);
    }

    pub fn letter(&self) -> &char {
        &self.letter
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Ord for Column {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Column {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Column {}
