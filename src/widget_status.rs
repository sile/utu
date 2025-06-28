use std::fmt::Write;

use orfail::OrFail;
use tuinix::{TerminalFrame, TerminalStyle};

use crate::{editor::Editor, tuinix_ext::UnicodeCharWidthEstimator};

#[derive(Debug)]
pub struct StatusLine;

impl StatusLine {
    pub fn render(
        &self,
        editor: &Editor,
        frame: &mut TerminalFrame<UnicodeCharWidthEstimator>,
    ) -> orfail::Result<()> {
        // Create a styled status bar with reverse colors (dark background, light text)
        let style = TerminalStyle::new().reverse().bold();
        let reset = TerminalStyle::RESET;
        let filler = " ".repeat(frame.size().cols);

        // Show file status, cursor position, and mode information
        writeln!(
            frame,
            "{style}{}[{}:{}:{}] {} {filler}{reset}",
            if editor.dirty.content { '*' } else { ' ' }, // Dirty indicator
            editor.path.file_name().and_then(|n| n.to_str()).or_fail()?,
            editor.cursor.row + 1,
            editor.cursor.col + 1,
            if let Some(m) = &editor.marker {
                m.name()
            } else if editor.clipboard.is_some() {
                "CLIPBOARD"
            } else {
                "DRAW"
            }
        )
        .or_fail()?;

        Ok(())
    }
}
