use crate::{Result, editor::Editor, tuinix_ext::put_text};

/// The one-line message drawn along the bottom of the frame.
#[derive(Debug)]
pub struct MessageLine;

impl MessageLine {
    /// Paints the editor's message, if any, into `frame`.
    pub fn render(&self, editor: &Editor, frame: &mut tuinix::Frame) -> Result<()> {
        if let Some(message) = &editor.message {
            put_text(
                frame,
                tuinix::Position::ORIGIN,
                message,
                tuinix::Style::new(),
            );
        }
        Ok(())
    }
}
