use crate::{Result, editor::Editor, tuinix_ext::put_text};

/// The key-binding legend drawn down the right edge.
#[derive(Debug)]
pub struct Legend {
    /// Whether the legend is collapsed to its handle.
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

    /// Makes a legend that starts shown.
    pub fn new() -> Self {
        Self { hide: false }
    }

    /// Paints the legend into `frame`, listing the commands still reachable
    /// from the pending keys.
    pub fn render(&self, editor: &Editor, frame: &mut tuinix::Frame) -> Result<()> {
        if frame.size().cols != self.size(editor).cols {
            return Ok(());
        }

        let mut row = 0;
        if self.hide {
            put_text(
                frame,
                tuinix::Position { row, col: 0 },
                "└───",
                tuinix::Style::new(),
            );
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
            put_text(
                frame,
                tuinix::Position { row, col: 0 },
                &line,
                tuinix::Style::new(),
            );
            row += 1;
        }

        // Add the hide option at the bottom
        put_text(
            frame,
            tuinix::Position { row, col: 0 },
            "└─────────────────────",
            tuinix::Style::new(),
        );

        Ok(())
    }

    // TODO: remove (Frame::shrink_rows())
    fn size(&self, editor: &Editor) -> tuinix::Size {
        if self.hide {
            tuinix::Size {
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
            tuinix::Size {
                rows,
                cols: Self::SHOW_COLS,
            }
        }
    }

    /// Returns the region of a `size`-sized frame the legend occupies.
    pub fn region(&self, editor: &Editor, size: tuinix::Size) -> tuinix::Region {
        let legend_size = self.size(editor);
        size.to_region()
            .take_top(legend_size.rows)
            .take_right(legend_size.cols)
    }

    /// Collapses or expands the legend, with a message saying which.
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
