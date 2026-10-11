use std::path::PathBuf;

use crate::{
    Result,
    buffer::{TextBuffer, TextPosition},
    clipboard::Clipboard,
    config::Config,
    keybinding::KeySequence,
    marker::Marker,
};

/// Everything the editor knows: the file, the buffer, the cursor, and the
/// surrounding mode state.
///
/// The fields are public so the widgets and the I/O edge can read them
/// directly. The commands that change them go through the `dot`, `save`, and
/// `reload` methods, so the buffer and the dirty flags stay consistent.
#[derive(Debug)]
pub struct Editor {
    /// The file the buffer is read from and saved to.
    pub path: PathBuf,

    /// Whether the user has asked to quit.
    pub exit: bool,

    /// What still needs saving or repainting.
    pub dirty: Dirty,

    /// The cursor's position in [`buffer`](Editor::buffer).
    pub cursor: TextPosition,

    /// The edited text.
    pub buffer: TextBuffer,

    /// The one-line message under the status line, if any.
    pub message: Option<String>,

    /// The compile-time configuration.
    pub config: Config,

    /// The keys pressed so far for a binding that is still being typed.
    pub pending_keys: KeySequence,

    /// The active selection, if a marking mode is on.
    pub marker: Option<Marker>,

    /// The cut-or-copied pixels, if the clipboard is open.
    pub clipboard: Option<Clipboard>,
}

impl Editor {
    /// Makes an editor over `path`, configured by `config`.
    ///
    /// The file is not read here; call [`reload()`](Editor::reload) for that.
    pub fn new(path: PathBuf, config: Config) -> Result<Self> {
        let mut buffer = TextBuffer::new();
        buffer.filter.fg_chars = config.keybindings.fg_chars().collect();

        Ok(Self {
            path,
            exit: false,
            dirty: Dirty {
                content: false,
                render: true,
            },
            cursor: TextPosition::default(),
            buffer,
            message: None,
            config,
            pending_keys: KeySequence::default(),
            marker: None,
            clipboard: None,
        })
    }

    /// Shows `message` on the message line and marks the frame dirty.
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
        self.dirty.render = true;
    }

    /// Clears the message line, marking the frame dirty only if it had one.
    pub fn clear_message(&mut self) {
        if self.message.take().is_some() {
            self.dirty.render = true;
        }
    }

    /// Writes `c` at the cursor, or across every marked position when a
    /// selection is active.
    pub fn dot(&mut self, c: char) -> Result<()> {
        if let Some(marker) = self.marker.take() {
            // Handle marker: apply character to all marked positions
            let positions: Vec<_> = marker.marked_positions().collect();
            let updates = positions.into_iter().map(|pos| (pos, c));

            if self.buffer.update_bulk(updates) {
                self.dirty.content = true;
                self.dirty.render = true;
            } else {
                self.set_message("No effect");
            }
        } else {
            // Handle single character update at cursor
            if self.buffer.update(self.cursor, c) {
                self.dirty.content = true;
                self.dirty.render = true;
            } else {
                self.set_message("No effect");
            }
        }
        Ok(())
    }

    /// Writes the buffer back to [`path`](Editor::path).
    ///
    /// A buffer with no unsaved changes is left alone, with a message saying
    /// so.
    pub fn save(&mut self) -> Result<()> {
        if !self.dirty.content {
            self.set_message("No changes to save");
            return Ok(());
        }

        let mut content = self.buffer.lines().collect::<Vec<&str>>().join("\n");
        content.push('\n');
        std::fs::write(&self.path, content)?;

        self.dirty.content = false;
        self.set_message(format!("Saved {}", self.path.display()));

        Ok(())
    }

    /// Reads [`path`](Editor::path) back into the buffer and clears the dirty
    /// flags.
    pub fn reload(&mut self) -> Result<()> {
        let text = std::fs::read_to_string(&self.path)?;
        self.buffer.set_text(text);
        self.set_message(format!("Loaded {}", self.path.display()));

        self.dirty.content = false;
        self.dirty.render = true;

        Ok(())
    }
}

/// What the editor has to do next.
#[derive(Debug)]
pub struct Dirty {
    /// The buffer has changes that are not on disk yet.
    pub content: bool,

    /// The terminal has to be repainted.
    pub render: bool,
}
