#[derive(Debug)]
pub struct TextBuffer {
    lines: Vec<String>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn lines(&self) -> impl '_ + Iterator<Item = &str> {
        self.lines.iter().map(|s| s.as_ref())
    }

    pub fn set_text(&mut self, text: String) {
        self.lines = text.lines().map(|s| s.to_owned()).collect();
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    pub row: usize,
    pub col: usize,
}
