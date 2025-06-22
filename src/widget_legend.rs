use std::fmt::Write;

use orfail::OrFail;
use tuinix::{TerminalFrame, TerminalPosition, TerminalSize};

use crate::{editor::Editor, tuinix_ext::TerminalRegion};

#[derive(Debug)]
pub struct Legend {
    pub hide: bool,
}

impl Legend {
    pub fn new() -> Self {
        Self { hide: false }
    }

    pub fn render(&self, _editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        if self.hide {
            return Ok(());
        }

        // Get available space for legend
        // let frame_size = frame.size();
        // if frame_size.cols < 20 || frame_size.rows < 3 {
        //     return Ok(()); // Not enough space to show legend
        // }

        // Calculate position for legend (right side of screen)

        // Basic keybindings for the editor
        let keybindings = [
            "│ quit     [^c] │",
            "│ (↑)      [^p] │",
            "│ (↓)      [^n] │",
            "│ (←)      [^b] │",
            "│ (→)      [^f] │",
            "└────(^h)ide────┘",
        ];

        // Draw the legend box
        for line in keybindings.iter() {
            writeln!(frame, "{}", line).or_fail()?;
        }

        Ok(())
    }

    pub fn region(&self, size: TerminalSize) -> TerminalRegion {
        if self.hide {
            // Return an empty region if hidden
            return TerminalRegion {
                position: TerminalPosition::ZERO,
                size: TerminalSize { rows: 0, cols: 0 },
            };
        }

        // Legend dimensions
        let legend_width = 17; // Width of the legend box
        let legend_height = 6; // Number of lines in the keybindings array

        // Check if we have enough space
        if size.cols < 20 || size.rows < 3 {
            // Not enough space, return empty region
            return TerminalRegion {
                position: TerminalPosition::ZERO,
                size: TerminalSize { rows: 0, cols: 0 },
            };
        }

        // Position legend on the right side of the screen
        let legend_col = size.cols.saturating_sub(legend_width);
        let legend_row = 0; // Start at the top

        // Ensure legend fits within available space
        let actual_width = legend_width.min(size.cols);
        let actual_height = legend_height.min(size.rows);

        TerminalRegion {
            position: TerminalPosition {
                row: legend_row,
                col: legend_col,
            },
            size: TerminalSize {
                rows: actual_height,
                cols: actual_width,
            },
        }
    }

    pub fn toggle_hide(&mut self, editor: &mut Editor) {
        self.hide = !self.hide;
        if self.hide {
            editor.set_message("Hide Legend");
        } else {
            editor.set_message("Show Legend");
        }
        editor.dirty.render = true;
    }
}
