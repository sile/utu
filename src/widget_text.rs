use std::collections::BTreeSet;

use tuinix::{Char, Frame, Position, Style};
use unicode_width::UnicodeWidthChar;

use crate::{Result, buffer::TextPosition, editor::Editor};

#[derive(Debug)]
pub struct TextView {
    scroll_offset: TextPosition,
}

impl Default for TextView {
    fn default() -> Self {
        Self::new()
    }
}

impl TextView {
    pub fn new() -> Self {
        Self {
            scroll_offset: TextPosition::default(),
        }
    }

    pub fn render(&mut self, editor: &Editor, frame: &mut Frame) -> Result<()> {
        let terminal_size = frame.size();

        // Adjust scroll offset to keep cursor visible
        self.adjust_scroll_offset_for_cursor(editor, terminal_size.rows, terminal_size.cols);

        // Collect all marked positions for efficient lookup
        let marked_positions: BTreeSet<TextPosition> = editor
            .marker
            .as_ref()
            .map(|marker| marker.marked_positions().collect())
            .unwrap_or_default();

        // Render visible lines
        for (line_index, line) in editor
            .buffer
            .lines()
            .skip(self.scroll_offset.row)
            .take(terminal_size.rows)
            .enumerate()
        {
            let current_row = self.scroll_offset.row + line_index;
            let mut current_col = self.scroll_offset.col;

            // Process each character in the visible portion of the line
            for c in line
                .chars()
                .skip(self.scroll_offset.col)
                .take(terminal_size.cols)
            {
                let c = editor.buffer.filter.apply(c);
                let position = TextPosition {
                    row: current_row,
                    col: current_col,
                };
                let (c, is_clipboard) = editor
                    .clipboard
                    .as_ref()
                    .and_then(|cb| cb.get(position).map(|c| (c, true)))
                    .unwrap_or((c, false));
                let width = c.width().unwrap_or_default();

                let style = if is_clipboard {
                    Style::new().reverse().underline()
                } else if marked_positions.contains(&position) {
                    // Render with highlight style
                    Style::new().reverse()
                } else {
                    // Render normally
                    Style::new()
                };

                if width > 0
                    && let Some(ch) = Char::new(c, width, style)
                {
                    let at = Position {
                        row: line_index,
                        col: current_col - self.scroll_offset.col,
                    };
                    frame.put_char(at, ch);
                }
                current_col += width;
            }
        }

        Ok(())
    }

    fn adjust_scroll_offset_for_cursor(
        &mut self,
        editor: &Editor,
        terminal_rows: usize,
        terminal_cols: usize,
    ) {
        let cursor = editor.cursor;

        // Adjust vertical scrolling
        if cursor.row < self.scroll_offset.row {
            // Cursor is above visible area, scroll up
            self.scroll_offset.row = cursor.row;
        } else if cursor.row >= self.scroll_offset.row + terminal_rows {
            // Cursor is below visible area, scroll down
            self.scroll_offset.row = cursor.row.saturating_sub(terminal_rows.saturating_sub(1));
        }

        // Adjust horizontal scrolling
        if cursor.col < self.scroll_offset.col {
            // Cursor is left of visible area, scroll left
            self.scroll_offset.col = cursor.col;
        } else if cursor.col >= self.scroll_offset.col + terminal_cols {
            // Cursor is right of visible area, scroll right
            self.scroll_offset.col = cursor.col.saturating_sub(terminal_cols.saturating_sub(1));
        }
    }

    pub fn cursor_terminal_position(&self, editor: &Editor) -> Position {
        Position {
            row: editor.cursor.row.saturating_sub(self.scroll_offset.row),
            col: editor.cursor.col.saturating_sub(self.scroll_offset.col),
        }
    }
}
