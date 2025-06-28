use std::collections::BTreeMap;

use crate::{buffer::TextPosition, editor::Editor};

#[derive(Debug)]
pub struct Clipboard {
    pub position: TextPosition,               // left-top position
    pub pixels: BTreeMap<TextPosition, char>, // positions are relative to `position`
}

impl Clipboard {
    pub fn copy_marked_pixels(editor: &mut Editor) -> Option<Self> {
        let marker = editor.marker.take()?;
        let position = marker.marked_positions().min()?;
        let pixels = marker
            .marked_positions()
            .filter_map(|pos| {
                editor.buffer.get_char_at(pos).map(|c| {
                    let p = TextPosition {
                        row: pos.row - position.row,
                        col: pos.col - position.col,
                    };
                    (p, c)
                })
            })
            .collect();
        Some(Self { position, pixels })
    }

    pub fn get(&self, pos: TextPosition) -> Option<char> {
        let (Some(row), Some(col)) = (
            pos.row.checked_sub(self.position.row),
            pos.col.checked_sub(self.position.col),
        ) else {
            return None;
        };
        let rel_pos = TextPosition { row, col };
        self.pixels.get(&rel_pos).copied()
    }
}
