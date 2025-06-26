use std::collections::HashSet;

use unicode_width::UnicodeWidthChar;

#[derive(Debug)]
pub struct TextBuffer {
    lines: Vec<String>,
    undo_stack: Vec<UndoOperation>,
    undo_index: usize,
    undoing: bool,
    pub filter: TextBufferFilter,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            undo_stack: Vec::new(),
            undo_index: 0,
            undoing: false,
            filter: TextBufferFilter::default(),
        }
    }

    pub fn get_char_at(&self, pos: TextPosition) -> Option<char> {
        if pos.row >= self.rows() {
            return None;
        }

        let line = &self.lines[pos.row];
        let mut current_col = 0;

        for c in line.chars() {
            let char_width = self.filter.apply(c).width().unwrap_or(0);
            if current_col == pos.col {
                return Some(c);
            }
            current_col += char_width;
            if current_col > pos.col {
                // Position is in the middle of a wide character
                return None;
            }
        }

        None
    }

    pub fn lines(&self) -> impl '_ + Iterator<Item = &str> {
        self.lines.iter().map(|s| s.as_ref())
    }

    pub fn cols(&self, row: usize) -> usize {
        self.lines.get(row).map_or(0, |line| {
            line.chars()
                .map(|c| self.filter.apply(c).width().unwrap_or(0))
                .sum::<usize>()
        })
    }

    pub fn prev_col(&self, TextPosition { row, col }: TextPosition) -> usize {
        if row >= self.rows() || col == 0 {
            return 0;
        }

        let line = &self.lines[row];
        let mut current_col = 0;
        let mut prev_col = 0;

        for c in line.chars() {
            let c = self.filter.apply(c);
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
            let c = self.filter.apply(c);
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
        // Clear undo history when setting new text
        // TODO: keep history
        self.undo_stack.clear();
        self.undo_index = 0;
    }

    pub fn update(&mut self, pos: TextPosition, new: char) -> bool {
        if !(pos.row < self.rows() && pos.col < self.cols(pos.row)) {
            return false;
        }

        let mut current_cols = 0;
        for (i, c) in self.lines[pos.row].char_indices() {
            if current_cols >= pos.col {
                assert_eq!(current_cols, pos.col);
                if new == c || !self.filter.fg_chars.contains(&c) {
                    return false;
                }

                // Record the operation for undo
                let undo_op = UndoOperation::Update { pos, old_char: c };
                self.undo_stack.push(undo_op);
                if !self.undoing {
                    self.undo_index = self.undo_stack.len();
                }

                // Perform the actual update
                self.lines[pos.row].remove(i);
                self.lines[pos.row].insert(i, new);
                return true;
            }
            current_cols += c.width().unwrap_or(0);
        }
        panic!("bug")
    }

    pub fn undo(&mut self) -> Option<usize> {
        self.undo_index = self.undo_index.checked_sub(1)?;
        self.undoing = true;
        let undo_op = self.undo_stack[self.undo_index].clone();
        match undo_op {
            UndoOperation::Update { pos, old_char } => {
                self.update(pos, old_char);
            }
        }
        self.undoing = false;

        Some(self.undo_index)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
enum UndoOperation {
    Update { pos: TextPosition, old_char: char },
}

#[derive(Debug, Default)]
pub struct TextBufferFilter {
    pub bg_char: Option<char>,
    pub fg_chars: HashSet<char>,
}

impl TextBufferFilter {
    pub fn apply(&self, c: char) -> char {
        if self.fg_chars.contains(&c) {
            c
        } else {
            // TODO: consider c.width()
            self.bg_char.unwrap_or(c)
        }
    }
}
