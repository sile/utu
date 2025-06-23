// todo: rename
#[derive(Debug)]
pub struct TextBuffer {
    lines: Vec<String>,
    // todo: undo handling
}

impl TextBuffer {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn lines(&self) -> impl '_ + Iterator<Item = &str> {
        self.lines.iter().map(|s| s.as_ref())
    }

    pub fn cols(&self, row: usize) -> usize {
        // TODO: unicode width
        self.lines.get(row).map_or(0, |line| line.chars().count())
    }

    pub fn set_text(&mut self, text: String) {
        self.lines = text.lines().map(|s| s.to_owned()).collect();
    }

    pub fn update(&mut self, position: TextPosition, c: char) -> orfail::Result<bool> {
        todo!()
    }

    // save() (seek & write uodated pixels)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    pub row: usize,
    pub col: usize,
}
