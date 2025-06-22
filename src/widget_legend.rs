use std::fmt::Write;

use orfail::OrFail;
use tuinix::{TerminalFrame, TerminalSize};

use crate::{
    editor::Editor,
    tuinix_ext::{TerminalRegion, TerminalSizeExt},
};

#[derive(Debug)]
pub struct Legend {
    pub hide: bool,
}

impl Legend {
    const HIDE_COLS: usize = 12;
    const SHOW_COLS: usize = 20;

    pub fn new() -> Self {
        Self { hide: false }
    }

    pub fn render(&self, _editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        if frame.size() != self.size() {
            return Ok(());
        }

        if self.hide {
            writeln!(frame, "└──s(^h)ow──").or_fail()?;
            return Ok(());
        }

        // Calculate position for legend (right side of screen)

        // Basic keybindings for the editor
        let keybindings = [
            "│ quit          [^c]",
            "│ cancel        [^g]",
            "│ search        [^s]",
            "│ (↑)           [^p]",
            "│ (↓)           [^n]",
            "│ (←)           [^b]",
            "│ (→)           [^f]",
            "└──────(^h)ide──────",
        ];

        // Draw the legend box
        for line in keybindings.iter() {
            writeln!(frame, "{}", line).or_fail()?;
        }

        Ok(())
    }

    fn size(&self) -> TerminalSize {
        if self.hide {
            return TerminalSize::rows_cols(1, Self::HIDE_COLS);
        }

        TerminalSize::rows_cols(20, Self::SHOW_COLS) // TODO: calc rows
    }

    pub fn region(&self, size: TerminalSize) -> TerminalRegion {
        let legend_size = self.size();
        size.to_region()
            .top_rows(legend_size.rows)
            .right_cols(legend_size.cols)
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
