use tuinix::{Frame, Position, Region, Size, Style};

use crate::{Result, editor::Editor, tuinix_ext::put_text};

#[derive(Debug)]
pub struct Legend {
    pub hide: bool,
}

impl Default for Legend {
    fn default() -> Self {
        Self::new()
    }
}

impl Legend {
    const HIDE_COLS: usize = 4;
    const SHOW_COLS: usize = 20;

    pub fn new() -> Self {
        Self { hide: false }
    }

    pub fn render(&self, editor: &Editor, frame: &mut Frame) -> Result<()> {
        if frame.size().cols != self.size(editor).cols {
            return Ok(());
        }

        let mut row = 0;
        if self.hide {
            put_text(frame, Position { row, col: 0 }, "└───", Style::new());
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
            .possible_commands(group, &editor.pending_keys)
            .collect();

        // Draw the legend box
        for (key, command) in possible_commands.iter() {
            let line = format!("│ [{}] {}", key, command);
            put_text(frame, Position { row, col: 0 }, &line, Style::new());
            row += 1;
        }

        // Add the hide option at the bottom
        put_text(
            frame,
            Position { row, col: 0 },
            "└─────────────────────",
            Style::new(),
        );

        Ok(())
    }

    // TODO: remove (Frame::shrink_rows())
    fn size(&self, editor: &Editor) -> Size {
        if self.hide {
            Size {
                rows: 1,
                cols: Self::HIDE_COLS,
            }
        } else {
            let group = if editor.clipboard.is_some() {
                &editor.config.keybindings.clipboard
            } else {
                &editor.config.keybindings.main
            };
            let rows = 1 + editor
                .config
                .keybindings
                .possible_commands(group, &editor.pending_keys)
                .count();
            Size {
                rows,
                cols: Self::SHOW_COLS,
            }
        }
    }

    pub fn region(&self, editor: &Editor, size: Size) -> Region {
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
