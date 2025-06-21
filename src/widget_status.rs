use std::fmt::Write;

use orfail::OrFail;
use tuinix::{TerminalFrame, TerminalStyle};

use crate::editor::Editor;

#[derive(Debug)]
pub struct StatusBarWidget;

impl StatusBarWidget {
    pub fn render(&self, editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        // Create a styled status bar with reverse colors (dark background, light text)
        let style = TerminalStyle::new().reverse().bold();
        let reset = TerminalStyle::RESET;

        // Show file status, cursor position, and mode information
        writeln!(
            frame,
            "{}{}[{}] Line: 1 Col: 1 {}{}",
            style,
            if editor.dirty.content { '*' } else { ' ' }, // Dirty indicator
            editor
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled"),
            // Fill the rest of the line with spaces for consistent background
            " ".repeat(frame.size().cols.saturating_sub(20)), // Approximate width adjustment
            reset
        )
        .or_fail()?;

        Ok(())
    }
}
