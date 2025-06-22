#[derive(Debug)]
pub enum EditorCommand {
    Quit,
    Cancel,
    PrevLine,
    NextLine,
    PrevChar,
    NextChar,
    Dot(char),
}
