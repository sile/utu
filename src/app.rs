use std::path::PathBuf;

use orfail::OrFail;
use tuinix::{Terminal, TerminalEvent, TerminalInput};

use crate::{
    clipboard::Clipboard, config::Config, editor::Editor, editor_command::EditorCommand,
    tuinix_ext::TerminalFrame, widget_legend::Legend, widget_message::MessageLine,
    widget_preview::Preview, widget_status::StatusLine, widget_text::TextView,
};

#[derive(Debug)]
pub struct App {
    terminal: Terminal,
    pub editor: Editor,
    text_view: TextView,
    status_line: StatusLine,
    message_line: MessageLine,
    preview: Preview,
    legend: Legend,
}

impl App {
    pub fn new(path: PathBuf, config: Config) -> orfail::Result<Self> {
        let terminal = Terminal::new().or_fail()?;
        Ok(Self {
            terminal,
            editor: Editor::new(path, config).or_fail()?,
            text_view: TextView::new(),
            status_line: StatusLine,
            message_line: MessageLine,
            preview: Preview::default(),
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

        let mut frame = TerminalFrame::new(self.terminal.size());

        // Create regions for different UI components
        let full_region = frame.size().to_region();
        let main_region = full_region.drop_bottom(2);
        let status_region = full_region.drop_bottom(1).take_bottom(1);
        let message_region = full_region.take_bottom(1);

        // Render widgets
        let mut subframe = TerminalFrame::new(main_region.size);
        self.text_view
            .render(&self.editor, &mut subframe)
            .or_fail()?;
        frame.draw(main_region.position, &subframe);

        let mut subframe = TerminalFrame::new(status_region.size);
        self.status_line
            .render(&self.editor, &mut subframe)
            .or_fail()?;
        frame.draw(status_region.position, &subframe);

        let mut subframe = TerminalFrame::new(message_region.size);
        self.message_line
            .render(&self.editor, &mut subframe)
            .or_fail()?;
        frame.draw(message_region.position, &subframe);

        let preview_region = self.preview.region(&self.editor, frame.size());
        let mut subframe = TerminalFrame::new(preview_region.size);
        self.preview.render(&self.editor, &mut subframe).or_fail()?;
        frame.draw(preview_region.position, &subframe);

        let legend_region = self.legend.region(&self.editor, frame.size());
        let mut subframe = TerminalFrame::new(legend_region.size);
        self.legend.render(&self.editor, &mut subframe).or_fail()?;
        frame.draw(legend_region.position, &subframe);

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
                let root_group = if self.editor.clipboard.is_some() {
                    &self.editor.config.keybindings.clipboard
                } else {
                    &self.editor.config.keybindings.main
                };
                match self
                    .editor
                    .config
                    .keybindings
                    .find(&root_group, &self.editor.pending_keys)
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
                        self.handle_command(&command.clone()).or_fail()?;
                    }
                }
            }
            TerminalEvent::Resize(_) => {
                self.editor.dirty.render = true;
            }
        }
        Ok(())
    }

    fn handle_command(&mut self, command: &EditorCommand) -> orfail::Result<()> {
        match command {
            EditorCommand::Quit => {
                self.editor.exit = true;
            }
            EditorCommand::Legend => {
                self.legend.toggle_hide(&mut self.editor);
            }
            EditorCommand::Preview => {
                self.preview.toggle_hide(&mut self.editor);
            }
            EditorCommand::Background(c) => {
                if self.editor.buffer.filter.bg_char.take().is_none() {
                    self.editor.buffer.filter.bg_char = Some(*c);
                }
                self.editor.dirty.render = true;
            }
            EditorCommand::Cancel => {
                // Clear any pending operations or selections
                self.editor.pending_keys.clear();
                self.editor.marker = None;
                self.editor.clipboard = None;
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
                if let Some(mut marker) = self.editor.marker.take() {
                    marker.handle_cursor_move(&self.editor);
                    self.editor.marker = Some(marker);
                }
                if let Some(cb) = &mut self.editor.clipboard {
                    cb.cursor = self.editor.cursor;
                }
            }
            EditorCommand::NextLine => {
                let max_row = self.editor.buffer.lines().count().saturating_sub(1);
                self.editor.cursor.row = max_row.min(self.editor.cursor.row + 1);
                self.editor.dirty.render = true;
                if let Some(mut marker) = self.editor.marker.take() {
                    marker.handle_cursor_move(&self.editor);
                    self.editor.marker = Some(marker);
                }
                if let Some(cb) = &mut self.editor.clipboard {
                    cb.cursor = self.editor.cursor;
                }
            }
            EditorCommand::PrevChar => {
                self.editor.cursor.col = self.editor.buffer.prev_col(self.editor.cursor);
                self.editor.dirty.render = true;
                if let Some(mut marker) = self.editor.marker.take() {
                    marker.handle_cursor_move(&self.editor);
                    self.editor.marker = Some(marker);
                }
                if let Some(cb) = &mut self.editor.clipboard {
                    cb.cursor = self.editor.cursor;
                }
            }
            EditorCommand::NextChar => {
                self.editor.cursor.col = self.editor.buffer.next_col(self.editor.cursor);
                self.editor.dirty.render = true;
                if let Some(mut marker) = self.editor.marker.take() {
                    marker.handle_cursor_move(&self.editor);
                    self.editor.marker = Some(marker);
                }
                if let Some(cb) = &mut self.editor.clipboard {
                    cb.cursor = self.editor.cursor;
                }
            }
            EditorCommand::Dot(c) => self.editor.dot(*c).or_fail()?,
            EditorCommand::MarkStroke => {
                self.editor.marker = Some(crate::marker::Marker::new_stroke(&self.editor));
                self.editor.set_message("Stroke marking mode started");
                self.editor.dirty.render = true;
            }
            EditorCommand::MarkLine => {
                self.editor.marker = Some(crate::marker::Marker::new_line(&self.editor));
                self.editor.set_message("Line marking mode started");
                self.editor.dirty.render = true;
            }
            EditorCommand::MarkRect => {
                self.editor.marker = Some(crate::marker::Marker::new_rect(&self.editor));
                self.editor.set_message("Rectangle marking mode started");
                self.editor.dirty.render = true;
            }
            EditorCommand::MarkFilledRect => {
                self.editor.marker = Some(crate::marker::Marker::new_filled_rect(&self.editor));
                self.editor
                    .set_message("Filled rectangle marking mode started");
                self.editor.dirty.render = true;
            }
            EditorCommand::MarkFill => {
                self.editor.marker = Some(crate::marker::Marker::new_fill(&self.editor));
                self.editor.set_message("Fill marking mode started");
                self.editor.dirty.render = true;
            }
            EditorCommand::Save => self.editor.save().or_fail()?,
            EditorCommand::Scope(_) => unreachable!(),
            EditorCommand::Cut => {
                if let Some(clipboard) = Clipboard::copy_marked_pixels(&mut self.editor) {
                    // Overwrite clipboard pixel positions with canvas char
                    let canvas_char = self.editor.config.keybindings.canvas_char();
                    let positions_to_clear = clipboard.pixels.keys().copied();
                    self.editor
                        .buffer
                        .update_bulk(positions_to_clear.map(|pos| (pos, canvas_char)));

                    self.editor.clipboard = Some(clipboard);
                    self.editor.set_message("Enter clipboard mode");
                }
            }
            EditorCommand::Copy => {
                if let Some(clipboard) = Clipboard::copy_marked_pixels(&mut self.editor) {
                    self.editor.clipboard = Some(clipboard);
                    self.editor.set_message("Enter clipboard mode");
                }
            }
            EditorCommand::Paste => {
                if let Some(clipboard) = &self.editor.clipboard {
                    self.editor.buffer.update_bulk(clipboard.pixels());
                    self.editor.set_message("Pasted");
                } else {
                    self.editor.set_message("No clipboard content");
                }
            }
        }
        Ok(())
    }
}
