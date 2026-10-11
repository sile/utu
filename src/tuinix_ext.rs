use tuinix::KeyCode;

use unicode_width::UnicodeWidthChar;

/// Paints `text` into `frame` starting at `at`, using `style` for every
/// character, and returns the position just past the last character.
///
/// Newlines move to the next row and tabs advance to the next tab stop; any
/// other control character is skipped, as is any character of zero width.
pub fn put_text(
    frame: &mut tuinix::Frame,
    at: tuinix::Position,
    text: &str,
    style: tuinix::Style,
) -> tuinix::Position {
    let mut at = at;
    let tab_width = std::num::NonZeroUsize::new(8).expect("8 is not 0");
    for c in text.chars() {
        match c {
            '\n' => at = at.next_line(),
            '\t' => at = at.next_tab_stop(tab_width),
            _ => {
                // A width of `0` covers control characters and zero-width
                // combining marks; `Char::new` rejects those.
                let width = c.width().unwrap_or(0);
                if let Some(ch) = tuinix::Char::new(c, width, style) {
                    at = frame.put_char(at, ch);
                }
            }
        }
    }
    at
}

/// Writes the display form of `key` to `f` (for example `C-a` or `↑`).
pub fn display_key(key: &tuinix::KeyInput, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // Write modifiers
    if key.ctrl {
        write!(f, "C-")?;
    }
    if key.alt {
        write!(f, "M-")?;
    }

    // Write the key code
    match key.code {
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
        KeyCode::F(n) => write!(f, "F{}", n),
        KeyCode::Char(c) => write!(f, "{}", c),
    }
}

/// Parses the display form written by [`display_key`] back into a [`KeyInput`].
pub fn parse_key(s: &str) -> Option<tuinix::KeyInput> {
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
