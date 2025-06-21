use std::path::PathBuf;

use orfail::OrFail;
use tuinix::{KeyCode, Terminal, TerminalEvent, TerminalFrame, TerminalInput};

use crate::{
    editor::Editor,
    tuinix_ext::{TerminalFrameExt, TerminalSizeExt},
    widget_message::MessageLine,
    widget_status::StatusLine,
    widget_text::TextView,
};

#[derive(Debug)]
pub struct App {
    terminal: Terminal,
    editor: Editor,
    text_view: TextView,
    status_line: StatusLine,
    message_line: MessageLine,
}

impl App {
    pub fn new(path: PathBuf) -> orfail::Result<Self> {
        let terminal = Terminal::new().or_fail()?;
        Ok(Self {
            terminal,
            editor: Editor::new(path),
            text_view: TextView::new(),
            status_line: StatusLine,
            message_line: MessageLine,
        })
    }

    pub fn run(mut self) -> orfail::Result<()> {
        while !self.editor.exit {
            self.editor.try_reload().or_fail()?;
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

        // Create regions for different UI components
        let full_region = frame.size().to_region();
        let main_region = full_region.without_bottom_rows(2);
        let status_region = full_region.without_bottom_rows(1).bottom_rows(1);
        let message_region = full_region.bottom_rows(1);

        // Render widgets
        frame.draw_in_region(main_region, |frame| {
            self.text_view.render(&self.editor, frame)
        })?;
        frame.draw_in_region(status_region, |frame| {
            self.status_line.render(&self.editor, frame)
        })?;
        frame.draw_in_region(message_region, |frame| {
            self.message_line.render(&self.editor, frame)
        })?;

        // Set cursor position for text editing
        let cursor_pos = self.text_view.cursor_terminal_position(&self.editor);
        self.terminal.set_cursor(Some(cursor_pos));

        self.terminal.draw(frame).or_fail()?;

        self.editor.dirty.render = false;

        Ok(())
    }

    fn handle_event(&mut self, event: TerminalEvent) -> orfail::Result<()> {
        self.editor.clear_message();

        match event {
            TerminalEvent::Input(input) => {
                let TerminalInput::Key(key) = input;
                if key.ctrl && matches!(key.code, tuinix::KeyCode::Char('c')) {
                    self.editor.exit = true;
                } else {
                    // Handle basic cursor movement for now
                    match key.code {
                        KeyCode::Up => {
                            if self.editor.cursor.row > 0 {
                                self.editor.cursor.row -= 1;
                                self.editor.dirty.render = true;
                            }
                        }
                        KeyCode::Down => {
                            // Limit to buffer length - 1
                            let max_row = self.editor.buffer.lines().count().saturating_sub(1);
                            if self.editor.cursor.row < max_row {
                                self.editor.cursor.row += 1;
                                self.editor.dirty.render = true;
                            }
                        }
                        KeyCode::Left => {
                            if self.editor.cursor.col > 0 {
                                self.editor.cursor.col -= 1;
                                self.editor.dirty.render = true;
                            }
                        }
                        KeyCode::Right => {
                            // Allow cursor to move one position past end of line for editing
                            self.editor.cursor.col += 1;
                            self.editor.dirty.render = true;
                        }
                        _ => {}
                    }
                }
            }
            TerminalEvent::Resize(_) => {
                self.editor.dirty.render = true;
            }
        }
        Ok(())
    }
}
