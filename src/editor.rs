use std::path::PathBuf;

#[derive(Debug)]
pub struct Editor {
    pub path: PathBuf,
    pub exit: bool,
    pub dirty: Dirty,
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
        }
    }
}

#[derive(Debug)]
pub struct Dirty {
    pub content: bool, // File content needs to be saved
    pub render: bool,  // Terminal needs to be re-rendered
}
