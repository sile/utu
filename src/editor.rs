use std::path::PathBuf;

#[derive(Debug)]
pub struct Editor {
    path: PathBuf,
}

impl Editor {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn run(self) -> orfail::Result<()> {
        Ok(())
    }
}
