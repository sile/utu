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

    pub fn rows(&self) -> usize {
        self.lines.len()
    }

    pub fn set_text(&mut self, text: String) {
        self.lines = text.lines().map(|s| s.to_owned()).collect();
    }

    pub fn update(&mut self, pos: TextPosition, c: char) -> bool {
        if !(pos.row < self.rows() && pos.col < self.cols(pos.row)) {
            return false;
        }

        self.lines[pos.row].insert(pos.col, c);
        c != self.lines[pos.row].remove(pos.col + 1) // todo: width
    }

    // save() (seek & write uodated pixels)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    pub row: usize,
    pub col: usize,
}
