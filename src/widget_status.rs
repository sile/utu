use unicode_width::UnicodeWidthStr;

use crate::{Error, Result, editor::Editor, tuinix_ext::put_text};

/// The reversed status bar drawn above the message line.
#[derive(Debug)]
pub struct StatusLine;

impl StatusLine {
    /// Paints the file name, cursor position, and mode into `frame`.
    pub fn render(&self, editor: &Editor, frame: &mut tuinix::Frame) -> Result<()> {
        // Create a styled status bar with reverse colors (dark background, light text)
        let style = tuinix::Style::new().reverse().bold();

        // Show file status, cursor position, and mode information
        let text = format!(
            "{}[{}:{}:{}] [CANVAS:{:?}] {}",
            if editor.dirty.content { '*' } else { ' ' }, // Dirty indicator
            editor
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| Error::new("file name is not valid UTF-8"))?,
            editor.cursor.row + 1,
            editor.cursor.col + 1,
            editor.config.keybindings.canvas_char(),
            if let Some(m) = &editor.marker {
                m.name()
            } else if editor.clipboard.is_some() {
                "CLIPBOARD"
            } else {
                "DRAW"
            },
        );

        // Pad the row so the reverse style reaches the right edge. The padding
        // is measured in columns, not characters, so a wide character in the
        // file name does not leave the bar short.
        let pad = frame.size().cols.saturating_sub(text.width());
        let padded = format!("{text}{}", " ".repeat(pad));
        put_text(frame, tuinix::Position::ORIGIN, &padded, style);

        Ok(())
    }
}
