use std::fmt::Write;

use orfail::OrFail;
use tuinix::TerminalFrame;

use crate::editor::Editor;

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
        let frame_size = frame.size();
        if frame_size.cols < 20 || frame_size.rows < 3 {
            return Ok(()); // Not enough space to show legend
        }

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

    pub fn toggle_hide(&mut self) {
        self.hide = !self.hide;
    }
}
