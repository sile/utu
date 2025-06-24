use std::path::PathBuf;

use orfail::OrFail;

use crate::{
    buffer::{TextBuffer, TextPosition},
    key_binding::{KeyBindings, KeySequence},
};

#[derive(Debug)]
pub struct Editor {
    pub path: PathBuf,
    pub exit: bool,
    pub dirty: Dirty,
    pub cursor: TextPosition,
    pub buffer: TextBuffer,
    pub message: Option<String>,
    pub key_bindings: KeyBindings,
    pub pending_keys: KeySequence,
}

impl Editor {
    pub fn new(path: PathBuf) -> orfail::Result<Self> {
        let mut buffer = TextBuffer::new();
        let key_bindings = KeyBindings::default();
        buffer.filter.fg_chars = key_bindings.fg_chars().collect();

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
            key_bindings,
            pending_keys: KeySequence::default(),
        })
    }

    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
        self.dirty.render = true;
    }

    pub fn clear_message(&mut self) {
        if self.message.take().is_some() {
            self.dirty.render = true;
        }
    }

    pub fn dot(&mut self, c: char) -> orfail::Result<()> {
        // todo: consider marker

        if self.buffer.update(self.cursor, c) {
            self.dirty.content = true;
            self.dirty.render = true;
        } else {
            self.set_message("No effect");
        }
        Ok(())
    }

    pub fn save(&mut self) -> orfail::Result<()> {
        if !self.dirty.content {
            self.set_message("No changes to save");
            return Ok(());
        }

        let content = self.buffer.lines().collect::<Vec<&str>>().join("\n");
        std::fs::write(&self.path, content).or_fail()?;

        self.dirty.content = false;
        self.set_message(format!("Saved {}", self.path.display()));

        Ok(())
    }

    pub fn reload(&mut self) -> orfail::Result<()> {
        let text = std::fs::read_to_string(&self.path).or_fail()?;
        self.buffer.set_text(text);
        self.set_message(format!("Loaded {}", self.path.display()));

        self.dirty.content = false;
        self.dirty.render = true;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Dirty {
    pub content: bool, // File content needs to be saved
    pub render: bool,  // Terminal needs to be re-rendered
}
