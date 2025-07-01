use std::fmt::Write;

use orfail::OrFail;
use tuinix::{TerminalRegion, TerminalSize};

use crate::{config::Color, editor::Editor, tuinix_ext::TerminalFrame};

#[derive(Debug, Default)]
pub struct Preview {
    pub hide: bool,
}

impl Preview {
    pub fn toggle_hide(&mut self, editor: &mut Editor) {
        self.hide = !self.hide;
        if self.hide {
            editor.set_message("Hide Preview");
        } else {
            editor.set_message("Show Preview");
        }
        editor.dirty.render = true;
    }

    pub fn render(&self, editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        if frame.size().cols != self.size(editor).cols {
            return Ok(());
        }

        if self.hide {
            writeln!(frame, "┌───").or_fail()?;
            return Ok(());
        }

        // Render preview border and content
        let preview_size = self.size(editor);

        // Top border
        writeln!(
            frame,
            "┌{}─",
            "─".repeat(preview_size.cols.saturating_sub(2))
        )
        .or_fail()?;

        // Content area - render as pixels using block characters
        let content_height = preview_size.rows.saturating_sub(1); // Subtract top border only
        let content_width = preview_size.cols.saturating_sub(1); // Subtract left border only

        let cursor_pixel_row = editor.cursor.row;
        let cursor_pixel_col = editor.cursor.col;

        // Calculate the viewport offset to center the cursor
        let viewport_height = content_height * 2; // Each terminal row shows 2 pixel rows
        let viewport_width = content_width;

        let viewport_start_row = cursor_pixel_row.saturating_sub(viewport_height / 2);
        let viewport_start_col = cursor_pixel_col.saturating_sub(viewport_width / 2);

        for terminal_row in 0..content_height {
            write!(frame, "│").or_fail()?;

            // Each terminal row represents 2 pixel rows (using ▄ character)
            let pixel_row_top = viewport_start_row + terminal_row * 2;
            let pixel_row_bottom = viewport_start_row + terminal_row * 2 + 1;

            for pixel_col in 0..content_width {
                let actual_pixel_col = viewport_start_col + pixel_col;

                // Get colors for top and bottom pixels
                let top_color = self.get_pixel_color(editor, pixel_row_top, actual_pixel_col);
                let bottom_color = self.get_pixel_color(editor, pixel_row_bottom, actual_pixel_col);

                // Convert to terminal colors
                let top_terminal_color =
                    tuinix::TerminalColor::new(top_color.r, top_color.g, top_color.b);
                let bottom_terminal_color =
                    tuinix::TerminalColor::new(bottom_color.r, bottom_color.g, bottom_color.b);

                // Use ▄ character with foreground as bottom pixel and background as top pixel
                write!(
                    frame,
                    "{}▄",
                    tuinix::TerminalStyle::new()
                        .fg_color(bottom_terminal_color)
                        .bg_color(top_terminal_color)
                )
                .or_fail()?;
            }

            writeln!(frame, "{}", tuinix::TerminalStyle::RESET).or_fail()?;
        }

        Ok(())
    }

    fn get_pixel_color(&self, editor: &Editor, pixel_row: usize, pixel_col: usize) -> Color {
        let buffer = &editor.buffer;
        let default_bg = Color::rgb(0xFF, 0xFF, 0xFF); // TODO: configurable
        // let default_bg = Color::rgb(200, 200, 200); // TODO: configurable

        // Check if we're within buffer bounds
        if pixel_row >= buffer.rows() {
            return default_bg;
        }

        // Get the character at this position
        let text_pos = crate::buffer::TextPosition {
            row: pixel_row,
            col: pixel_col,
        };

        if let Some(ch) = buffer.get_char_at(text_pos) {
            // Look up color in palette
            return editor
                .config
                .palette
                .colors
                .get(&ch)
                .copied()
                .expect("TODO: validate too");
        }

        // Return background color for empty spaces or unmapped characters
        default_bg
    }

    pub fn size(&self, editor: &Editor) -> TerminalSize {
        if self.hide {
            TerminalSize::rows_cols(1, 4)
        } else {
            let config = &editor.config.preview;
            TerminalSize::rows_cols(config.height / 2 + 1, config.width + 1)
        }
    }

    pub fn region(&self, editor: &Editor, size: TerminalSize) -> TerminalRegion {
        let preview_size = self.size(editor);
        size.to_region()
            .drop_bottom(2)
            .take_bottom(preview_size.rows)
            .take_right(preview_size.cols)
    }
}
