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
    Scope(String),
    PrevLine,
    NextLine,
    PrevChar,
    NextChar,
    Dot(char),
    MarkStroke,
    MarkLine,
    MarkRect,
    MarkFilledRect,
    MarkFill,
    Cut,
    Copy,
}

impl std::fmt::Display for EditorCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorCommand::Quit => write!(f, "quit"),
            EditorCommand::Legend => write!(f, "legend"),
            EditorCommand::Background(c) => write!(f, "bg({})", c),
            EditorCommand::Cancel => write!(f, "cancel"),
            EditorCommand::Save => write!(f, "save"),
            EditorCommand::Undo => write!(f, "undo"),
            EditorCommand::Scope(s) => write!(f, "scope({})", s),
            EditorCommand::PrevLine => write!(f, "prev-line"),
            EditorCommand::NextLine => write!(f, "next-line"),
            EditorCommand::PrevChar => write!(f, "prev-char"),
            EditorCommand::NextChar => write!(f, "next-char"),
            EditorCommand::Dot(c) => write!(f, "dot({})", c),
            EditorCommand::MarkStroke => write!(f, "mark-stroke"),
            EditorCommand::MarkLine => write!(f, "mark-line"),
            EditorCommand::MarkRect => write!(f, "mark-rect"),
            EditorCommand::MarkFilledRect => write!(f, "mark-filled-rect"),
            EditorCommand::MarkFill => write!(f, "mark-fill"),
            EditorCommand::Cut => write!(f, "cut"),
            EditorCommand::Copy => write!(f, "copy"),
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
            "mark-filled-rect" => Ok(EditorCommand::MarkFilledRect),
            "mark-fill" => Ok(EditorCommand::MarkFill),
            "cut" => Ok(EditorCommand::Cut),
            "copy" => Ok(EditorCommand::Copy),
            s if s.starts_with("dot(") && s.ends_with(")") => {
                let arg = &s[4..s.len() - 1];
                let mut chars = arg.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) if !c.is_control() => Ok(EditorCommand::Dot(c)),
                    _ => Err(format!("invalid dot command: {}", s)),
                }
            }
            s if s.starts_with("bg(") && s.ends_with(")") => {
                let arg = &s[3..s.len() - 1];
                let mut chars = arg.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) if !c.is_control() => Ok(EditorCommand::Background(c)),
                    _ => Err(format!("invalid bg command: {}", s)),
                }
            }
            s if s.starts_with("scope(") && s.ends_with(")") => {
                let group_name = &s[6..s.len() - 1];
                if group_name.is_empty() {
                    Err(format!("invalid scope command: {}", s))
                } else {
                    Ok(EditorCommand::Scope(group_name.to_owned()))
                }
            }
            _ => Err(format!("unknown command: {}", s)),
        }
    }
}
