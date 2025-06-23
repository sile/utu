use std::{path::PathBuf, time::SystemTime};

use orfail::OrFail;

use crate::{
    buffer::{TextBuffer, TextPosition},
    key_binding::{KeyBindings, KeySequence},
};

#[derive(Debug)]
pub struct Editor {
    pub path: PathBuf,
    pub mtime: Option<SystemTime>,
    pub exit: bool,
    pub dirty: Dirty,
    pub cursor: TextPosition,
    pub buffer: TextBuffer,
    pub message: Option<String>,
    pub key_bindings: KeyBindings,
    pub pending_keys: KeySequence,
}

impl Editor {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            mtime: None,
            exit: false,
            dirty: Dirty {
                content: false,
                render: true,
            },
            cursor: TextPosition::default(),
            buffer: TextBuffer::new(),
            message: None,
            key_bindings: KeyBindings::default(),
            pending_keys: KeySequence::default(),
        }
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

        if self.buffer.update(self.cursor, c).or_fail()? {
            self.dirty.content = true;
            self.dirty.render = true;
        }
        Ok(())
    }

    pub fn try_reload(&mut self) -> orfail::Result<()> {
        let metadata = self.path.metadata().or_fail()?;
        let mtime = metadata.modified().or_fail()?;

        if self.mtime == Some(mtime) {
            return Ok(());
        }

        let text = std::fs::read_to_string(&self.path).or_fail()?;
        self.buffer.set_text(text);

        if self.mtime.is_none() {
            self.set_message(format!("Opened {}", self.path.display()));
        } else {
            self.set_message(format!("Reloaded {}", self.path.display()));
        }

        self.mtime = Some(mtime);
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
