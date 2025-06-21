#[derive(Debug)]
pub struct TextBuffer {}

impl TextBuffer {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    pub row: usize,
    pub col: usize,
}
