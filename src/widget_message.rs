use std::fmt::Write;

use crate::{Result, editor::Editor, tuinix_ext::TerminalFrame};

#[derive(Debug)]
pub struct MessageLine;

impl MessageLine {
    pub fn render(&self, editor: &Editor, frame: &mut TerminalFrame) -> Result<()> {
        if let Some(message) = &editor.message {
            writeln!(frame, "{}", message,)?;
        }
        Ok(())
    }
}
