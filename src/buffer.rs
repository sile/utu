use std::{collections::HashSet, num::NonZeroUsize};

use unicode_width::UnicodeWidthChar;

/// The edited text, cut into lines, with an undo history.
///
/// Positions are [`TextPosition`] values in display columns: a position names
/// a column of the rendered line, not a byte or character index into the
/// stored [`String`].
#[derive(Debug)]
pub struct TextBuffer {
    lines: Vec<String>,
    undo_stack: Vec<UndoOperation>,
    undo_index: usize,
    undoing: bool,

    /// The display filter applied to every character read or written.
    pub filter: TextBufferFilter,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextBuffer {
    /// Makes an empty buffer.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            undo_stack: Vec::new(),
            undo_index: 0,
            undoing: false,
            filter: TextBufferFilter::default(),
        }
    }

    /// Returns the character drawn at `pos`, or `None` when `pos` is out of
    /// range, falls in the middle of a wide character, or holds a character the
    /// filter hides.
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

    /// Iterates over the lines, top to bottom.
    pub fn lines(&self) -> impl '_ + Iterator<Item = &str> {
        self.lines.iter().map(|s| s.as_ref())
    }

    /// Returns the display width of line `row`, or `0` when the row is out of
    /// range.
    pub fn cols(&self, row: usize) -> usize {
        self.lines.get(row).map_or(0, |line| {
            line.chars()
                .map(|c| self.filter.apply(c).width().unwrap_or(0))
                .sum::<usize>()
        })
    }

    /// Returns the column that precedes `col` on `row`, stopping at column
    /// `0`.
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

    /// Returns the column that follows `col` on `row`, stopping at the end of
    /// the line.
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

    /// Returns the number of lines.
    pub fn rows(&self) -> usize {
        self.lines.len()
    }

    /// Replaces the whole buffer with `text`, splitting it into lines and
    /// clearing the undo history.
    pub fn set_text(&mut self, text: String) {
        self.lines = text.lines().map(|s| s.to_owned()).collect();
        // Clear undo history when setting new text
        // TODO: keep history
        self.undo_stack.clear();
        self.undo_index = 0;
    }

    /// Writes `new` at `pos`, returning whether anything changed.
    pub fn update(&mut self, pos: TextPosition, new: char) -> bool {
        self.update_bulk(std::iter::once((pos, new)))
    }

    /// Writes each `(position, character)` pair, returning whether anything
    /// changed.
    ///
    /// The whole batch is one undo step: [`undo()`](TextBuffer::undo) reverses
    /// every write it made.
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

    /// Reverses the most recent update, returning how many undo steps remain,
    /// or `None` when there is nothing left to undo.
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

/// A zero-based position in the buffer: a line and a display column.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    /// The line, counted from the top.
    pub row: usize,

    /// The display column, counted from the left.
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

/// Which characters the buffer draws, and what it draws in place of the rest.
#[derive(Debug, Default)]
pub struct TextBufferFilter {
    /// Drawn in place of any character not in [`fg_chars`](Self::fg_chars),
    /// or the character itself when unset.
    pub bg_char: Option<char>,

    /// The characters drawn as themselves.
    pub fg_chars: HashSet<char>,
}

impl TextBufferFilter {
    /// Returns the character to draw for `c`: `c` itself when it is a
    /// foreground character, and the background character otherwise.
    pub fn apply(&self, c: char) -> char {
        if self.fg_chars.contains(&c) {
            c
        } else {
            // TODO: consider c.width()
            self.bg_char.unwrap_or(c)
        }
    }
}
