use tuinix::KeyCode;

use unicode_width::UnicodeWidthChar;

pub type TerminalFrame = tuinix::TerminalFrame<UnicodeCharWidthEstimator>;

pub trait KeyInputExt {
    fn from_str(s: &str) -> Option<tuinix::KeyInput>;
    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn to_string(&self) -> String;
}

impl KeyInputExt for tuinix::KeyInput {
    fn to_string(&self) -> String {
        struct DisplayWrapper<'a>(&'a tuinix::KeyInput);

        impl<'a> std::fmt::Display for DisplayWrapper<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.display(f)
            }
        }

        format!("{}", DisplayWrapper(self))
    }

    fn from_str(s: &str) -> Option<tuinix::KeyInput> {
        // Control
        let (ctrl, s) = if let Some(s) = s.strip_prefix("C-") {
            (true, s)
        } else {
            (false, s)
        };

        // Alt
        let (alt, s) = if let Some(s) = s.strip_prefix("M-") {
            (true, s)
        } else {
            (false, s)
        };

        let key = |code| tuinix::KeyInput { ctrl, alt, code };

        // Special keys
        match s {
            "↑" => return Some(key(KeyCode::Up)),
            "↓" => return Some(key(KeyCode::Down)),
            "←" => return Some(key(KeyCode::Left)),
            "→" => return Some(key(KeyCode::Right)),
            "↵" => return Some(key(KeyCode::Enter)),
            "⎋" => return Some(key(KeyCode::Escape)),
            "⌫" => return Some(key(KeyCode::Backspace)),
            "⇥" => return Some(key(KeyCode::Tab)),
            "⇤" => return Some(key(KeyCode::BackTab)),
            "⌦" => return Some(key(KeyCode::Delete)),
            "⎀" => return Some(key(KeyCode::Insert)),
            "⇱" => return Some(key(KeyCode::Home)),
            "⇲" => return Some(key(KeyCode::End)),
            "⇞" => return Some(key(KeyCode::PageUp)),
            "⇟" => return Some(key(KeyCode::PageDown)),
            _ => {}
        }

        // Normal keys
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) if !c.is_control() => Some(key(KeyCode::Char(c))),
            _ => None,
        }
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write modifiers
        if self.ctrl {
            write!(f, "C-")?;
        }
        if self.alt {
            write!(f, "M-")?;
        }

        // Write the key code
        match self.code {
            KeyCode::Up => write!(f, "↑"),
            KeyCode::Down => write!(f, "↓"),
            KeyCode::Left => write!(f, "←"),
            KeyCode::Right => write!(f, "→"),
            KeyCode::Enter => write!(f, "↵"),
            KeyCode::Escape => write!(f, "⎋"),
            KeyCode::Backspace => write!(f, "⌫"),
            KeyCode::Tab => write!(f, "⇥"),
            KeyCode::BackTab => write!(f, "⇤"),
            KeyCode::Delete => write!(f, "⌦"),
            KeyCode::Insert => write!(f, "⎀"),
            KeyCode::Home => write!(f, "⇱"),
            KeyCode::End => write!(f, "⇲"),
            KeyCode::PageUp => write!(f, "⇞"),
            KeyCode::PageDown => write!(f, "⇟"),
            KeyCode::Char(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Default)]
pub struct UnicodeCharWidthEstimator;

impl tuinix::EstimateCharWidth for UnicodeCharWidthEstimator {
    fn estimate_char_width(&self, c: char) -> usize {
        c.width().unwrap_or(0)
    }
}
