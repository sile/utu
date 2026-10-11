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
        tuinix::KeyCode::Up => write!(f, "↑"),
        tuinix::KeyCode::Down => write!(f, "↓"),
        tuinix::KeyCode::Left => write!(f, "←"),
        tuinix::KeyCode::Right => write!(f, "→"),
        tuinix::KeyCode::Enter => write!(f, "↵"),
        tuinix::KeyCode::Escape => write!(f, "⎋"),
        tuinix::KeyCode::Backspace => write!(f, "⌫"),
        tuinix::KeyCode::Tab => write!(f, "⇥"),
        tuinix::KeyCode::BackTab => write!(f, "⇤"),
        tuinix::KeyCode::Delete => write!(f, "⌦"),
        tuinix::KeyCode::Insert => write!(f, "⎀"),
        tuinix::KeyCode::Home => write!(f, "⇱"),
        tuinix::KeyCode::End => write!(f, "⇲"),
        tuinix::KeyCode::PageUp => write!(f, "⇞"),
        tuinix::KeyCode::PageDown => write!(f, "⇟"),
        tuinix::KeyCode::F(n) => write!(f, "F{}", n),
        tuinix::KeyCode::Char(c) => write!(f, "{}", c),
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
        "↑" => return Some(key(tuinix::KeyCode::Up)),
        "↓" => return Some(key(tuinix::KeyCode::Down)),
        "←" => return Some(key(tuinix::KeyCode::Left)),
        "→" => return Some(key(tuinix::KeyCode::Right)),
        "↵" => return Some(key(tuinix::KeyCode::Enter)),
        "⎋" => return Some(key(tuinix::KeyCode::Escape)),
        "⌫" => return Some(key(tuinix::KeyCode::Backspace)),
        "⇥" => return Some(key(tuinix::KeyCode::Tab)),
        "⇤" => return Some(key(tuinix::KeyCode::BackTab)),
        "⌦" => return Some(key(tuinix::KeyCode::Delete)),
        "⎀" => return Some(key(tuinix::KeyCode::Insert)),
        "⇱" => return Some(key(tuinix::KeyCode::Home)),
        "⇲" => return Some(key(tuinix::KeyCode::End)),
        "⇞" => return Some(key(tuinix::KeyCode::PageUp)),
        "⇟" => return Some(key(tuinix::KeyCode::PageDown)),
        _ => {}
    }

    // Normal keys
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) if !c.is_control() => Some(key(tuinix::KeyCode::Char(c))),
        _ => None,
    }
}
