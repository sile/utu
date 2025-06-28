use std::collections::BTreeMap;

use crate::{buffer::TextPosition, editor::Editor};

#[derive(Debug)]
pub struct Clipboard {
    pub original_cursor: TextPosition,
    pub cursor: TextPosition,
    pub pixels: BTreeMap<TextPosition, char>,
}

impl Clipboard {
    pub fn copy_marked_pixels(editor: &mut Editor) -> Option<Self> {
        let marker = editor.marker.take()?;
        let pixels = marker
            .marked_positions()
            .filter_map(|pos| editor.buffer.get_char_at(pos).map(|c| (pos, c)))
            .collect();
        let cursor = editor.cursor;
        Some(Self {
            original_cursor: cursor,
            cursor,
            pixels,
        })
    }

    pub fn get(&self, pos: TextPosition) -> Option<char> {
        let (Some(row), Some(col)) = (
            (pos.row + self.original_cursor.row).checked_sub(self.cursor.row),
            (pos.col + self.original_cursor.col).checked_sub(self.cursor.col),
        ) else {
            return None;
        };
        let rel_pos = TextPosition { row, col };
        self.pixels.get(&rel_pos).copied()
    }
}
