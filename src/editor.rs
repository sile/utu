use std::path::PathBuf;

use crate::buffer::{TextBuffer, TextPosition};

#[derive(Debug)]
pub struct Editor {
    pub path: PathBuf,
    pub exit: bool,
    pub dirty: Dirty,
    pub cursor: TextPosition,
    pub buffer: TextBuffer,
    pub notification: Option<String>,
}

impl Editor {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            exit: false,
            dirty: Dirty {
                content: false,
                render: true,
            },
            cursor: TextPosition::default(),
            buffer: TextBuffer::new(),
            notification: None,
        }
    }
}

#[derive(Debug)]
pub struct Dirty {
    pub content: bool, // File content needs to be saved
    pub render: bool,  // Terminal needs to be re-rendered
}
