use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

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
        self.lines.get(row).map_or(0, |line| line.width())
    }

    pub fn prev_col(&self, TextPosition { row, col }: TextPosition) -> usize {
        if row >= self.rows() || col == 0 {
            return 0;
        }

        let line = &self.lines[row];
        let mut current_col = 0;
        let mut prev_col = 0;

        for c in line.chars() {
            let char_width = c.width().unwrap_or(0);
            if current_col >= col {
                return prev_col;
            }
            prev_col = current_col;
            current_col += char_width;
        }

        prev_col
    }

    pub fn next_col(&self, TextPosition { row, col }: TextPosition) -> usize {
        if row >= self.rows() {
            return col;
        }

        let line = &self.lines[row];
        let mut current_col = 0;

        for c in line.chars() {
            let char_width = c.width().unwrap_or(0);
            if current_col > col {
                return current_col;
            }
            current_col += char_width;
            if current_col > col {
                return current_col;
            }
        }

        current_col
    }

    pub fn rows(&self) -> usize {
        self.lines.len()
    }

    pub fn set_text(&mut self, text: String) {
        self.lines = text.lines().map(|s| s.to_owned()).collect();
    }

    pub fn update(&mut self, pos: TextPosition, new: char) -> bool {
        if !(pos.row < self.rows() && pos.col < self.cols(pos.row)) {
            return false;
        }

        let mut current_cols = 0;
        for (i, c) in self.lines[pos.row].char_indices() {
            if current_cols >= pos.col {
                assert_eq!(current_cols, pos.col);
                if new == c {
                    return false;
                }
                self.lines[pos.row].remove(i);
                self.lines[pos.row].insert(i, c);
                return true;
            }
            current_cols += c.width().unwrap_or(0);
        }
        panic!("bug")
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    pub row: usize,
    pub col: usize,
}
