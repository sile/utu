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

impl<'text> nojson::FromRawJsonValue<'text> for EditorCommand {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let s = value.to_unquoted_string_str()?;
        match s.as_ref() {
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
                    _ => Err(value.invalid(format!("invalid dot command: {}", s))),
                }
            }
            _ => Err(value.invalid(format!("unknown command: {}", s))),
        }
    }
}

pub trait RawJsonValueExt {
    fn invalid<E>(self, error: E) -> nojson::JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>;
}

impl<'text, 'json> RawJsonValueExt for nojson::RawJsonValue<'text, 'json> {
    fn invalid<E>(self, error: E) -> nojson::JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        nojson::JsonParseError::invalid_value(self, error)
    }
}
