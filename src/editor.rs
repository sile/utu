use std::{path::PathBuf, time::SystemTime};

use orfail::OrFail;

use crate::buffer::{TextBuffer, TextPosition};

#[derive(Debug)]
pub struct Editor {
    pub path: PathBuf,
    pub mtime: Option<SystemTime>,
    pub exit: bool,
    pub dirty: Dirty,
    pub cursor: TextPosition,
    pub buffer: TextBuffer,
    pub message: Option<String>,
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

    pub fn try_reload(&mut self) -> orfail::Result<()> {
        let metadata = self.path.metadata().or_fail()?;
        let mtime = metadata.modified().or_fail()?;
        if self.mtime.is_none() {
            self.mtime = Some(mtime);
            self.set_message("File opened");
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Dirty {
    pub content: bool, // File content needs to be saved
    pub render: bool,  // Terminal needs to be re-rendered
}
