use std::fmt::Write;

use orfail::OrFail;
use tuinix::TerminalFrame;

use crate::editor::Editor;

#[derive(Debug)]
pub struct NotificationBar;

impl NotificationBar {
    pub fn render(&self, editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        if let Some(message) = &editor.notification {
            writeln!(frame, "{}", message,).or_fail()?;
        }
        Ok(())
    }
}
