use std::path::PathBuf;

use orfail::OrFail;
use tuinix::{Terminal, TerminalEvent, TerminalFrame, TerminalInput};

use crate::{
    editor::Editor,
    editor_command::EditorCommand,
    tuinix_ext::{TerminalFrameExt, TerminalSizeExt, UnicodeCharWidthEstimator},
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
            editor: Editor::new(path).or_fail()?,
            text_view: TextView::new(),
            status_line: StatusLine,
            message_line: MessageLine,
            legend: Legend::new(),
        })
    }

    pub fn run(mut self) -> orfail::Result<()> {
        self.editor.reload().or_fail()?;

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

        let mut frame = TerminalFrame::with_char_width_estimator(
            self.terminal.size(),
            UnicodeCharWidthEstimator,
        );

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
                match self
                    .editor
                    .config
                    .keybindings
                    .find(&self.editor.pending_keys)
                {
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
            EditorCommand::Legend => {
                self.legend.toggle_hide(&mut self.editor);
            }
            EditorCommand::Background(c) => {
                if self.editor.buffer.filter.bg_char.take().is_none() {
                    self.editor.buffer.filter.bg_char = Some(c);
                }
                self.editor.dirty.render = true;
            }
            EditorCommand::Cancel => {
                // Clear any pending operations or selections
                self.editor.pending_keys.clear();
                self.editor.set_message("Canceled");
            }
            EditorCommand::Undo => {
                if let Some(i) = self.editor.buffer.undo() {
                    self.editor.set_message(format!("Undo: {i} remainings"));
                }
            }
            EditorCommand::PrevLine => {
                self.editor.cursor.row = self.editor.cursor.row.saturating_sub(1);
                self.editor.dirty.render = true;
            }
            EditorCommand::NextLine => {
                let max_row = self.editor.buffer.lines().count().saturating_sub(1);
                self.editor.cursor.row = max_row.min(self.editor.cursor.row + 1);
                self.editor.dirty.render = true;
            }
            EditorCommand::PrevChar => {
                self.editor.cursor.col = self.editor.buffer.prev_col(self.editor.cursor);
                self.editor.dirty.render = true;
            }
            EditorCommand::NextChar => {
                self.editor.cursor.col = self.editor.buffer.next_col(self.editor.cursor);
                self.editor.dirty.render = true;
            }
            EditorCommand::Dot(c) => self.editor.dot(c).or_fail()?,
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
            EditorCommand::Save => self.editor.save().or_fail()?,
        }
        Ok(())
    }
}
