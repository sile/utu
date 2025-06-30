use std::fmt::Write;

use orfail::OrFail;

use crate::{editor::Editor, tuinix_ext::TerminalFrame};

#[derive(Debug)]
pub struct MessageLine;

impl MessageLine {
    pub fn render(&self, editor: &Editor, frame: &mut TerminalFrame) -> orfail::Result<()> {
        if let Some(message) = &editor.message {
            writeln!(frame, "{}", message,).or_fail()?;
        }
        Ok(())
    }
}
