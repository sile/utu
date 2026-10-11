use tuinix::{Frame, Position, Style};

use crate::{Result, editor::Editor, tuinix_ext::put_text};

#[derive(Debug)]
pub struct MessageLine;

impl MessageLine {
    pub fn render(&self, editor: &Editor, frame: &mut Frame) -> Result<()> {
        if let Some(message) = &editor.message {
            put_text(frame, Position::ORIGIN, message, Style::new());
        }
        Ok(())
    }
}
