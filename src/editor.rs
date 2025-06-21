use std::path::PathBuf;

#[derive(Debug)]
pub struct Editor {
    pub path: PathBuf,
    pub exit: bool,
    pub dirty: bool,
}

impl Editor {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            exit: false,
            dirty: true,
        }
    }
}
