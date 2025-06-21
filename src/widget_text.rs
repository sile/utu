use std::fmt::Write;

use orfail::OrFail;
use tuinix::TerminalFrame;

use crate::{buffer::TextPosition, editor::Editor};

#[derive(Debug)]
pub struct TextView {
    scroll_offset: TextPosition,
}

impl TextView {
    pub fn new() -> Self {
        Self {
            scroll_offset: TextPosition::default(),
        }
    }

    pub fn render(&mut self, editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        let terminal_size = frame.size();

        // Adjust scroll offset to keep cursor visible
        self.adjust_scroll_offset_for_cursor(editor, terminal_size.rows, terminal_size.cols);

        // Render visible lines
        for (display_row, line) in editor
            .buffer
            .lines()
            .skip(self.scroll_offset.row)
            .take(terminal_size.rows)
            .enumerate()
        {
            // Get the visible portion of the line
            let visible_chars: String = line
                .chars()
                .skip(self.scroll_offset.col)
                .take(terminal_size.cols)
                .collect();

            // Write the line to the frame
            if display_row < terminal_size.rows - 1 {
                writeln!(frame, "{}", visible_chars).or_fail()?;
            } else {
                // Don't add newline on last row to avoid scrolling
                write!(frame, "{}", visible_chars).or_fail()?;
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

    pub fn cursor_terminal_position(&self, editor: &Editor) -> tuinix::TerminalPosition {
        tuinix::TerminalPosition {
            row: editor.cursor.row.saturating_sub(self.scroll_offset.row),
            col: editor.cursor.col.saturating_sub(self.scroll_offset.col),
        }
    }
}
