use std::{collections::HashSet, num::NonZeroUsize};

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
                return self.filter.fg_chars.contains(&c).then_some(c);
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
        self.update_bulk(std::iter::once((pos, new)))
    }

    pub fn update_bulk(&mut self, updates: impl Iterator<Item = (TextPosition, char)>) -> bool {
        let mut any_updated = false;
        let mut bulk_undo_updates = Vec::new();

        // Collect all valid updates first
        for (pos, new) in updates {
            if !(pos.row < self.rows() && pos.col < self.cols(pos.row)) {
                continue;
            }

            let mut current_cols = 0;
            for (i, c) in self.lines[pos.row].char_indices() {
                if current_cols >= pos.col {
                    assert_eq!(current_cols, pos.col);
                    if new == c || !self.filter.fg_chars.contains(&c) {
                        break;
                    }

                    // Record the operation for bulk undo
                    bulk_undo_updates.push((pos, c)); // old char

                    // Perform the actual update
                    self.lines[pos.row].remove(i);
                    self.lines[pos.row].insert(i, new);
                    any_updated = true;
                    break;
                }
                current_cols += c.width().unwrap_or(0);
            }
        }

        // Add bulk undo operation to stack if any updates occurred
        if any_updated && !bulk_undo_updates.is_empty() {
            self.undo_stack.push(UndoOperation::BulkUpdate {
                updates: bulk_undo_updates,
            });
            if !self.undoing {
                self.undo_index = self.undo_stack.len();
            }
        }

        any_updated
    }

    pub fn undo(&mut self) -> Option<usize> {
        self.undo_index = self.undo_index.checked_sub(1)?;
        self.undoing = true;
        let undo_op = self.undo_stack[self.undo_index].clone();
        match undo_op {
            UndoOperation::BulkUpdate { updates } => {
                // Restore all characters in bulk
                self.update_bulk(updates.into_iter());
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

impl std::str::FromStr for TextPosition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, col) = s
            .split_once(':')
            .ok_or_else(|| format!("Invalid format: expected 'ROW:COLUMN', got '{}'", s))?;
        let row: NonZeroUsize = row
            .parse()
            .map_err(|e| format!("Invalid row number '{}': {}", row, e))?;
        let col: NonZeroUsize = col
            .parse()
            .map_err(|e| format!("Invalid column number '{}': {}", col, e))?;
        Ok(Self {
            row: row.get() - 1,
            col: col.get() - 1,
        })
    }
}

#[derive(Debug, Clone)]
enum UndoOperation {
    BulkUpdate { updates: Vec<(TextPosition, char)> },
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
