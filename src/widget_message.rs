use std::fmt::Write;

use orfail::OrFail;
use tuinix::TerminalFrame;

use crate::{editor::Editor, tuinix_ext::UnicodeCharWidthEstimator};

#[derive(Debug)]
pub struct MessageLine;

impl MessageLine {
    pub fn render(
        &self,
        editor: &Editor,
        frame: &mut TerminalFrame<UnicodeCharWidthEstimator>,
    ) -> orfail::Result<()> {
        if let Some(message) = &editor.message {
            writeln!(frame, "{}", message,).or_fail()?;
        }
        Ok(())
    }
}
