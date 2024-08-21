use std::{cmp, ops::Range};

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }

    pub fn get(&self, rang: Range<usize>) -> String {
        let start = rang.start;
        let end = cmp::min(rang.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}
