use std::io::{Read as _, Write as _};
use std::path::PathBuf;

use tuinix::{Frame, Input, InputDecoder, TerminalDriver};

use crate::{
    Result, clipboard::Clipboard, config::Config, editor::Editor, editor_command::EditorCommand,
    widget_legend::Legend, widget_message::MessageLine, widget_preview::Preview,
    widget_status::StatusLine, widget_text::TextView,
};

/// How long to wait for the rest of an escape sequence before a lone `ESC` byte
/// is treated as the Escape key.
const ESCAPE_TIMEOUT_MS: libc::c_int = 50;

#[derive(Debug)]
pub struct App {
    driver: TerminalDriver,
    input: InputDecoder,
    prev_frame: Option<Frame>,
    pub editor: Editor,
    text_view: TextView,
    status_line: StatusLine,
    message_line: MessageLine,
    preview: Preview,
    legend: Legend,
}

impl App {
    pub fn new(path: PathBuf, config: Config) -> Result<Self> {
        let driver = TerminalDriver::new()?;
        Ok(Self {
            driver,
            input: InputDecoder::new(),
            prev_frame: None,
            editor: Editor::new(path, config)?,
            text_view: TextView::new(),
            status_line: StatusLine,
            message_line: MessageLine,
            preview: Preview::default(),
            legend: Legend::new(),
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.editor.reload()?;

        let mut fds = [
            libc::pollfd {
                fd: self.driver.resize_signal_fd(),
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: self.driver.input_fd(),
                events: libc::POLLIN,
                revents: 0,
            },
        ];

        while !self.editor.exit {
            self.render()?;

            // A lone `ESC` byte is ambiguous, so wait only briefly while one is
            // held so it is reported as Escape promptly.
            let timeout = if self.input.has_uncommitted_escape() {
                ESCAPE_TIMEOUT_MS
            } else {
                -1
            };
            #[expect(
                unsafe_code,
                reason = "libc::poll is the only way to wait on the resize and input fds at once"
            )]
            let n = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, timeout) };
            if n < 0 {
                let err = std::io::Error::last_os_error();
                // `poll` is never restarted by `SA_RESTART`, so SIGWINCH makes it
                // return `EINTR`; the resize byte is already in the pipe, so retry.
                if err.kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(err.into());
            }

            if fds
                .iter()
                .any(|fd| fd.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) != 0)
            {
                return Err(crate::Error::new("terminal closed"));
            }

            if n == 0 {
                self.input.commit_escape();
            }

            if fds[0].revents & libc::POLLIN != 0 {
                self.driver.handle_resize_signal()?;
                self.editor.dirty.render = true;
            }

            if fds[1].revents & libc::POLLIN != 0 {
                self.read_input()?;
            }

            while let Some(input) = self.input.next() {
                self.handle_input(input)?;
            }
        }
        Ok(())
    }

    fn read_input(&mut self) -> Result<()> {
        let mut raw = [0u8; 256];
        loop {
            match self.driver.read(&mut raw) {
                Ok(n @ 1..) => self.input.feed(&raw[..n]),
                Ok(_) => break,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        if !self.editor.dirty.render {
            return Ok(());
        }

        let mut frame = Frame::new(self.driver.size());

        // Create regions for different UI components
        let full_region = frame.size().to_region();
        let main_region = full_region.drop_bottom(2);
        let status_region = full_region.drop_bottom(1).take_bottom(1);
        let message_region = full_region.take_bottom(1);

        // Render widgets
        let mut subframe = Frame::new(main_region.size);
        self.text_view.render(&self.editor, &mut subframe)?;
        frame.put_frame(main_region.position, &subframe);

        let mut subframe = Frame::new(status_region.size);
        self.status_line.render(&self.editor, &mut subframe)?;
        frame.put_frame(status_region.position, &subframe);

        let mut subframe = Frame::new(message_region.size);
        self.message_line.render(&self.editor, &mut subframe)?;
        frame.put_frame(message_region.position, &subframe);

        let preview_region = self.preview.region(&self.editor, frame.size());
        let mut subframe = Frame::new(preview_region.size);
        self.preview.render(&self.editor, &mut subframe)?;
        frame.put_frame(preview_region.position, &subframe);

        let legend_region = self.legend.region(&self.editor, frame.size());
        let mut subframe = Frame::new(legend_region.size);
        self.legend.render(&self.editor, &mut subframe)?;
        frame.put_frame(legend_region.position, &subframe);

        // Set cursor position for text editing
        let cursor_pos = self.text_view.cursor_terminal_position(&self.editor);

        let out = frame.render(self.prev_frame.as_ref(), Some(cursor_pos));
        self.driver.write_all(&out)?;
        self.driver.flush()?;
        self.prev_frame = Some(frame);

        self.editor.dirty.render = false;

        Ok(())
    }

    fn handle_input(&mut self, input: Input) -> Result<()> {
        self.editor.clear_message();

        // Only key events map to commands; mouse events are ignored.
        let Input::Key(key) = input else {
            return Ok(());
        };

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
            .find(root_group, &self.editor.pending_keys)
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
                self.handle_command(&command.clone())?;
            }
        }
        Ok(())
    }

    fn handle_command(&mut self, command: &EditorCommand) -> Result<()> {
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
            EditorCommand::Dot(c) => self.editor.dot(*c)?,
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
            EditorCommand::Save => self.editor.save()?,
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
                    if self.editor.buffer.update_bulk(clipboard.pixels()) {
                        self.editor.dirty.content = true;
                    }
                    self.editor.set_message("Pasted");
                } else {
                    self.editor.set_message("No clipboard content");
                }
            }
        }
        Ok(())
    }
}
