use std::fmt::Write;

use orfail::OrFail;
use tuinix::{TerminalRegion, TerminalSize};

use crate::{editor::Editor, tuinix_ext::TerminalFrame};

#[derive(Debug)]
pub struct Legend {
    pub hide: bool,
}

impl Legend {
    const HIDE_COLS: usize = 4;
    const SHOW_COLS: usize = 20;

    pub fn new() -> Self {
        Self { hide: false }
    }

    pub fn render(&self, editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        if frame.size().cols != self.size(editor).cols {
            return Ok(());
        }

        if self.hide {
            writeln!(frame, "└───").or_fail()?;
            return Ok(());
        }

        // Get actual possible commands based on current pending keys
        let group = if editor.clipboard.is_some() {
            &editor.config.keybindings.clipboard
        } else {
            &editor.config.keybindings.main
        };
        let possible_commands: Vec<_> = editor
            .config
            .keybindings
            .possible_commands(&group, &editor.pending_keys)
            .collect();

        // Draw the legend box
        for (key, command) in possible_commands.iter() {
            writeln!(frame, "│ [{}] {}", key, command).or_fail()?;
        }

        // Add the hide option at the bottom
        writeln!(frame, "└─────────────────────").or_fail()?;

        Ok(())
    }

    // TODO: remove (TerminalFrame::shrink_rows())
    fn size(&self, editor: &Editor) -> TerminalSize {
        if self.hide {
            TerminalSize::rows_cols(1, Self::HIDE_COLS)
        } else {
            let group = if editor.clipboard.is_some() {
                &editor.config.keybindings.clipboard
            } else {
                &editor.config.keybindings.main
            };
            let rows = 1 + editor
                .config
                .keybindings
                .possible_commands(&group, &editor.pending_keys)
                .count();
            TerminalSize::rows_cols(rows, Self::SHOW_COLS)
        }
    }

    pub fn region(&self, editor: &Editor, size: TerminalSize) -> TerminalRegion {
        let legend_size = self.size(editor);
        size.to_region()
            .take_top(legend_size.rows)
            .take_right(legend_size.cols)
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
