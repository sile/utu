use std::path::PathBuf;

use orfail::OrFail;
use tuinix::{Terminal, TerminalEvent, TerminalFrame, TerminalInput};

use crate::{
    editor::Editor, widget_notification::NotificationBarWidget, widget_status::StatusBarWidget,
};

#[derive(Debug)]
pub struct App {
    terminal: Terminal,
    editor: Editor,
    status_bar: StatusBarWidget,
    notification_bar: NotificationBarWidget,
}

impl App {
    pub fn new(path: PathBuf) -> orfail::Result<Self> {
        let terminal = Terminal::new().or_fail()?;
        Ok(Self {
            terminal,
            editor: Editor::new(path),
            status_bar: StatusBarWidget,
            notification_bar: NotificationBarWidget,
        })
    }

    pub fn run(mut self) -> orfail::Result<()> {
        while !self.editor.exit {
            self.render().or_fail()?;

            if let Some(event) = self.terminal.poll_event(None).or_fail()? {
                self.handle_event(event).or_fail()?;
            }
        }
        Ok(())
    }

    fn render(&mut self) -> orfail::Result<()> {
        if !self.editor.dirty.render {
            return Ok(());
        }

        let mut frame = TerminalFrame::new(self.terminal.size());

        // Basic content - show the file path
        use std::fmt::Write;
        writeln!(frame, "Editing: {}", self.editor.path.display()).or_fail()?;
        writeln!(frame, "Press Ctrl+C to exit").or_fail()?;

        self.terminal.draw(frame).or_fail()?;
        self.editor.dirty.render = false;

        Ok(())
    }

    fn handle_event(&mut self, event: TerminalEvent) -> orfail::Result<()> {
        match event {
            TerminalEvent::Input(input) => {
                let TerminalInput::Key(key) = input;
                if key.ctrl && matches!(key.code, tuinix::KeyCode::Char('c')) {
                    self.editor.exit = true;
                }
            }
            TerminalEvent::Resize(_) => {
                self.editor.dirty.render = true;
            }
        }
        Ok(())
    }
}
