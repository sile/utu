// AppCommand?
#[derive(Debug, Clone)]
pub enum EditorCommand {
    Quit,
    Legend,
    Background(char),
    Cancel,
    Save,
    Undo,
    // reload
    PrevLine,
    NextLine,
    PrevChar,
    NextChar,
    Dot(char),
    MarkStroke,
    MarkLine,
    MarkRect,
    MarkFill,
}

impl std::fmt::Display for EditorCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorCommand::Quit => write!(f, "quit"),
            EditorCommand::Legend => write!(f, "legend"),
            EditorCommand::Background(c) => write!(f, "bg-{}", c),
            EditorCommand::Cancel => write!(f, "cancel"),
            EditorCommand::Save => write!(f, "save"),
            EditorCommand::Undo => write!(f, "undo"),
            EditorCommand::PrevLine => write!(f, "prev-line"),
            EditorCommand::NextLine => write!(f, "next-line"),
            EditorCommand::PrevChar => write!(f, "prev-char"),
            EditorCommand::NextChar => write!(f, "next-char"),
            EditorCommand::Dot(c) => write!(f, "dot-{}", c),
            EditorCommand::MarkStroke => write!(f, "mark-stroke"),
            EditorCommand::MarkLine => write!(f, "mark-line"),
            EditorCommand::MarkRect => write!(f, "mark-rect"),
            EditorCommand::MarkFill => write!(f, "mark-fill"),
        }
    }
}

impl std::str::FromStr for EditorCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quit" => Ok(EditorCommand::Quit),
            "legend" => Ok(EditorCommand::Legend),
            "cancel" => Ok(EditorCommand::Cancel),
            "save" => Ok(EditorCommand::Save),
            "undo" => Ok(EditorCommand::Undo),
            "prev-line" => Ok(EditorCommand::PrevLine),
            "next-line" => Ok(EditorCommand::NextLine),
            "prev-char" => Ok(EditorCommand::PrevChar),
            "next-char" => Ok(EditorCommand::NextChar),
            "mark-stroke" => Ok(EditorCommand::MarkStroke),
            "mark-line" => Ok(EditorCommand::MarkLine),
            "mark-rect" => Ok(EditorCommand::MarkRect),
            "mark-fill" => Ok(EditorCommand::MarkFill),
            s if s.starts_with("dot-") => {
                let mut chars = s[4..].chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) if !c.is_control() => Ok(EditorCommand::Dot(c)),
                    _ => Err(format!("invalid dot command: {}", s)),
                }
            }
            s if s.starts_with("bg-") => {
                let mut chars = s[3..].chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) if !c.is_control() => Ok(EditorCommand::Background(c)),
                    _ => Err(format!("invalid bg command: {}", s)),
                }
            }
            _ => Err(format!("unknown command: {}", s)),
        }
    }
}
