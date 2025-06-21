use std::path::PathBuf;

use orfail::OrFail;
use tuinix::{Terminal, TerminalInput};

use crate::editor::Editor;

#[derive(Debug)]
pub struct App {
    terminal: Terminal,
    editor: Editor,
}

impl App {
    pub fn new(path: PathBuf) -> orfail::Result<Self> {
        let terminal = Terminal::new().or_fail()?;
        Ok(Self {
            terminal,
            editor: Editor::new(path),
        })
    }

    pub fn run(mut self) -> orfail::Result<()> {
        while !self.editor.exit {
            if let Some(event) = self.terminal.poll_event(None).or_fail()? {
                self.handle_event(event).or_fail()?;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: tuinix::TerminalEvent) -> orfail::Result<()> {
        match event {
            tuinix::TerminalEvent::Input(input) => {
                let TerminalInput::Key(key) = input;
                if key.ctrl && matches!(key.code, tuinix::KeyCode::Char('c')) {
                    self.editor.exit = true;
                }
            }
            tuinix::TerminalEvent::Resize(_) => {
                // Handle terminal resize if needed
            }
        }
        Ok(())
    }
}
