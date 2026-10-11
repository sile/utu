use tuinix::{Char, Frame, Position, Region, Size, Style};

use crate::{Result, config::Color, editor::Editor, tuinix_ext::put_text};

/// The terminal color matching a palette [`Color`], ignoring its alpha.
fn to_terminal_color(color: Color) -> tuinix::Color {
    tuinix::Color::Rgb(color.r, color.g, color.b)
}

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

    pub fn render(&self, editor: &Editor, frame: &mut Frame) -> Result<()> {
        if frame.size().cols != self.size(editor).cols {
            return Ok(());
        }

        if self.hide {
            put_text(frame, Position::ORIGIN, "┌───", Style::new());
            return Ok(());
        }

        // Render preview border and content
        let preview_size = self.size(editor);

        // Top border
        let top_border = format!("┌{}─", "─".repeat(preview_size.cols.saturating_sub(2)));
        put_text(frame, Position::ORIGIN, &top_border, Style::new());

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
            let at = Position {
                row: terminal_row + 1,
                col: 0,
            };
            put_text(frame, at, "│", Style::new());

            // Each terminal row represents 2 pixel rows (using ▄ character)
            let pixel_row_top = viewport_start_row + terminal_row * 2;
            let pixel_row_bottom = viewport_start_row + terminal_row * 2 + 1;

            for pixel_col in 0..content_width {
                let actual_pixel_col = viewport_start_col + pixel_col;

                // Get colors for top and bottom pixels
                let top_color = self.get_pixel_color(editor, pixel_row_top, actual_pixel_col);
                let bottom_color = self.get_pixel_color(editor, pixel_row_bottom, actual_pixel_col);

                // Use ▄ character with foreground as bottom pixel and background as top pixel
                let style = Style::new()
                    .fg_color(to_terminal_color(bottom_color))
                    .bg_color(to_terminal_color(top_color));
                let at = Position {
                    row: terminal_row + 1,
                    col: pixel_col + 1,
                };
                if let Some(ch) = Char::new('▄', 1, style) {
                    frame.put_char(at, ch);
                }
            }
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

    pub fn size(&self, editor: &Editor) -> Size {
        if self.hide {
            Size { rows: 1, cols: 4 }
        } else {
            let config = &editor.config.preview;
            Size {
                rows: config.height / 2 + 1,
                cols: config.width + 1,
            }
        }
    }

    pub fn region(&self, editor: &Editor, size: Size) -> Region {
        let preview_size = self.size(editor);
        size.to_region()
            .drop_bottom(2)
            .take_bottom(preview_size.rows)
            .take_right(preview_size.cols)
    }
}
