use std::{collections::BTreeSet, fmt::Write};

use orfail::OrFail;
use tuinix::{TerminalFrame, TerminalStyle};
use unicode_width::UnicodeWidthChar;

use crate::{buffer::TextPosition, editor::Editor, tuinix_ext::UnicodeCharWidthEstimator};

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

    pub fn render(
        &mut self,
        editor: &Editor,
        frame: &mut TerminalFrame<UnicodeCharWidthEstimator>,
    ) -> orfail::Result<()> {
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
                current_col += c.width().unwrap_or_default();

                // Check if this position is marked
                if marked_positions.contains(&position) {
                    // Render with highlight style
                    let style = TerminalStyle::new().reverse();
                    //let style = TerminalStyle::new().bg_color(TerminalColor::new(200, 200, 200));
                    let reset = TerminalStyle::RESET;
                    write!(frame, "{}{}{}", style, c, reset).or_fail()?;
                } else {
                    // Render normally
                    write!(frame, "{}", c).or_fail()?;
                }
            }

            // Move to next line
            writeln!(frame).or_fail()?;
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
