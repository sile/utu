use tuinix::{KeyCode, TerminalFrame, TerminalPosition, TerminalSize};
use unicode_width::UnicodeWidthChar;

pub trait TerminalFrameExt {
    fn draw_in_region<F, E>(&mut self, region: TerminalRegion, f: F) -> Result<(), E>
    where
        F: FnOnce(&mut TerminalFrame) -> Result<(), E>;
}

impl<W: tuinix::EstimateCharWidth> TerminalFrameExt for TerminalFrame<W> {
    fn draw_in_region<F, E>(&mut self, region: TerminalRegion, f: F) -> Result<(), E>
    where
        F: FnOnce(&mut TerminalFrame) -> Result<(), E>,
    {
        let mut subframe = TerminalFrame::new(region.size);
        f(&mut subframe)?;
        self.draw(region.position, &subframe);
        Ok(())
    }
}

pub trait TerminalSizeExt {
    fn rows_cols(rows: usize, cols: usize) -> TerminalSize;
    fn to_region(self) -> TerminalRegion;
}

impl TerminalSizeExt for TerminalSize {
    fn rows_cols(rows: usize, cols: usize) -> TerminalSize {
        Self { rows, cols }
    }

    fn to_region(self) -> TerminalRegion {
        TerminalRegion {
            position: TerminalPosition::ZERO,
            size: self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalRegion {
    pub position: TerminalPosition,
    pub size: TerminalSize,
}

impl TerminalRegion {
    pub fn top_rows(mut self, rows: usize) -> Self {
        self.size.rows = self.size.rows.min(rows);
        self
    }

    pub fn bottom_rows(mut self, rows: usize) -> Self {
        if let Some(offset) = self.size.rows.checked_sub(rows) {
            self.position.row += offset;
            self.size.rows = rows;
        }
        self
    }

    pub fn right_cols(mut self, cols: usize) -> Self {
        if let Some(offset) = self.size.cols.checked_sub(cols) {
            self.position.col += offset;
            self.size.cols = cols;
        }
        self
    }

    pub fn without_bottom_rows(mut self, rows: usize) -> Self {
        self.size.rows = self.size.rows.saturating_sub(rows);
        self
    }
}

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

#[derive(Debug)]
pub struct UnicodeCharWidthEstimator;

impl tuinix::EstimateCharWidth for UnicodeCharWidthEstimator {
    fn estimate_char_width(&self, c: char) -> usize {
        c.width().unwrap_or(0)
    }
}
