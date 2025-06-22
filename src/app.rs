use std::path::PathBuf;

use orfail::OrFail;
use tuinix::{Terminal, TerminalEvent, TerminalFrame, TerminalInput};

use crate::{
    editor::Editor,
    editor_command::EditorCommand,
    tuinix_ext::{TerminalFrameExt, TerminalSizeExt},
    widget_legend::Legend,
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
    legend: Legend,
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
            legend: Legend::new(),
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
            self.text_view.render(&self.editor, frame).or_fail()
        })?;
        frame.draw_in_region(status_region, |frame| {
            self.status_line.render(&self.editor, frame).or_fail()
        })?;
        frame.draw_in_region(message_region, |frame| {
            self.message_line.render(&self.editor, frame).or_fail()
        })?;
        frame.draw_in_region(self.legend.region(&self.editor, frame.size()), |frame| {
            self.legend.render(&self.editor, frame).or_fail()
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
                self.editor.pending_keys.push(key);
                match self.editor.key_bindings.find(&self.editor.pending_keys) {
                    Err(()) => {
                        self.editor
                            .set_message(format!("Undefined: {}", self.editor.pending_keys));
                        self.editor.pending_keys.clear();
                    }
                    Ok(None) => {
                        self.editor
                            .set_message(format!("[INPUT] {} ->", self.editor.pending_keys));
                    }
                    Ok(Some(command)) => {
                        self.editor.pending_keys.clear();
                        self.handle_command(command).or_fail()?;
                    }
                }
            }
            TerminalEvent::Resize(_) => {
                self.editor.dirty.render = true;
            }
        }
        Ok(())
    }

    fn handle_command(&mut self, command: EditorCommand) -> orfail::Result<()> {
        match command {
            EditorCommand::Quit => {
                self.editor.exit = true;
            }
            EditorCommand::ToggleLegend => {
                self.legend.toggle_hide(&mut self.editor);
            }
            EditorCommand::Cancel => {
                // Clear any pending operations or selections
                self.editor.pending_keys.clear();
                self.editor.set_message("Canceled");
            }
            EditorCommand::PrevLine => {
                if self.editor.cursor.row > 0 {
                    self.editor.cursor.row -= 1;
                    self.editor.dirty.render = true;
                }
            }
            EditorCommand::NextLine => {
                let max_row = self.editor.buffer.lines().count().saturating_sub(1);
                if self.editor.cursor.row < max_row {
                    self.editor.cursor.row += 1;
                    self.editor.dirty.render = true;
                }
            }
            EditorCommand::PrevChar => {
                if self.editor.cursor.col > 0 {
                    self.editor.cursor.col -= 1;
                    self.editor.dirty.render = true;
                }
            }
            EditorCommand::NextChar => {
                self.editor.cursor.col += 1;
                self.editor.dirty.render = true;
            }
            EditorCommand::Dot(_c) => {
                // Insert character at cursor position
                // This would need to be implemented based on the editor's text manipulation capabilities
                // For now, just mark as dirty to trigger a render
                self.editor.dirty.render = true;
                // TODO: Implement actual character insertion
            }
            EditorCommand::MarkStroke => {
                // Set marking mode to stroke
                self.editor.dirty.render = true;
                // TODO: Implement stroke marking mode
            }
            EditorCommand::MarkLine => {
                // Set marking mode to line
                self.editor.dirty.render = true;
                // TODO: Implement line marking mode
            }
            EditorCommand::MarkRect => {
                // Set marking mode to rectangle
                self.editor.dirty.render = true;
                // TODO: Implement rectangle marking mode
            }
            EditorCommand::MarkFill => {
                // Set marking mode to fill
                self.editor.dirty.render = true;
                // TODO: Implement fill marking mode
            }
        }
        Ok(())
    }
}
