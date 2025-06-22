use tuinix::KeyCode;

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

impl std::fmt::Display for EditorCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorCommand::Quit => write!(f, "quit"),
            EditorCommand::Cancel => write!(f, "cancel"),
            EditorCommand::PrevLine => write!(f, "prev-line"),
            EditorCommand::NextLine => write!(f, "next-line"),
            EditorCommand::PrevChar => write!(f, "prev-char"),
            EditorCommand::NextChar => write!(f, "next-char"),
            EditorCommand::Dot(c) => write!(f, "dot-{}", c),
        }
    }
}

impl std::str::FromStr for EditorCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quit" => Ok(EditorCommand::Quit),
            "cancel" => Ok(EditorCommand::Cancel),
            "prev-line" => Ok(EditorCommand::PrevLine),
            "next-line" => Ok(EditorCommand::NextLine),
            "prev-char" => Ok(EditorCommand::PrevChar),
            "next-char" => Ok(EditorCommand::NextChar),
            s if s.starts_with("dot-") => {
                let mut chars = s[4..].chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) if !c.is_control() => Ok(EditorCommand::Dot(c)),
                    _ => Err(format!("invalid dot command: {}", s)),
                }
            }
            _ => Err(format!("unknown command: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct KeyBindings {}

#[derive(Debug)]
pub struct KeyBinding {
    pub path: Vec<KeyCode>, // KeyPath
    pub command: EditorCommand,
}
