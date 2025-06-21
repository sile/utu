use std::path::PathBuf;

use crate::buffer::{TextBuffer, TextPosition};

#[derive(Debug)]
pub struct Editor {
    pub path: PathBuf,
    pub exit: bool,
    pub dirty: Dirty,
    pub cursor: TextPosition,
    pub buffer: TextBuffer,
    pub message: Option<String>,
}

impl Editor {
    pub fn new(path: PathBuf) -> Self {
        let message = match path.try_exists() {
            Err(e) => Some(e.to_string()),
            Ok(false) => Some("New file".to_owned()),
            Ok(true) => None,
        };
        Self {
            path,
            exit: false,
            dirty: Dirty {
                content: false,
                render: true,
            },
            cursor: TextPosition::default(),
            buffer: TextBuffer::new(),
            message,
        }
    }

    pub fn try_reload(&mut self) {
        // if !self.path.exists() {
        //     // TODO: clear buffer if need
        //     return;
        // }

        // self.dirty.content = false;
        // self.dirty.render = true;
    }
}

#[derive(Debug)]
pub struct Dirty {
    pub content: bool, // File content needs to be saved
    pub render: bool,  // Terminal needs to be re-rendered
}
