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

impl nojson::DisplayJson for EditorCommand {
    fn fmt(&self, f: &mut nojson::JsonFormatter<'_, '_>) -> std::fmt::Result {
        match self {
            EditorCommand::Quit => f.string("quit"),
            EditorCommand::Cancel => f.string("cancel"),
            EditorCommand::PrevLine => f.string("prev-line"),
            EditorCommand::NextLine => f.string("next-line"),
            EditorCommand::PrevChar => f.string("prev-char"),
            EditorCommand::NextChar => f.string("next-char"),
            EditorCommand::Dot(c) => f.string(format!("dot-{}", c)),
        }
    }
}

impl std::fmt::Display for EditorCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", nojson::Json(self))
    }
}
