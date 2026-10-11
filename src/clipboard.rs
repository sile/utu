use std::collections::BTreeMap;

use crate::{buffer::TextPosition, editor::Editor};

/// The pixels cut or copied out of the buffer, held while the clipboard is
/// open so the user can move them before pasting.
///
/// The pixels keep the positions they had when they were taken, relative to
/// [`original_cursor`](Clipboard::original_cursor); the cursor's current offset
/// from that point moves them on paste.
#[derive(Debug)]
pub struct Clipboard {
    /// Where the cursor was when the pixels were taken.
    pub original_cursor: TextPosition,

    /// Where the cursor is now.
    pub cursor: TextPosition,

    /// The taken pixels, keyed by their position when taken.
    pub pixels: BTreeMap<TextPosition, char>,
}

impl Clipboard {
    /// Takes the marked pixels out of `editor` into a new [`Clipboard`],
    /// clearing the marker.
    ///
    /// Returns `None` when nothing is marked, or when every marked position
    /// holds the canvas character, which is what a cut leaves behind.
    pub fn copy_marked_pixels(editor: &mut Editor) -> Option<Self> {
        let canvas_char = editor.config.keybindings.canvas_char();
        let marker = editor.marker.take()?;
        let pixels: BTreeMap<_, _> = marker
            .marked_positions()
            .filter_map(|pos| {
                editor
                    .buffer
                    .get_char_at(pos)
                    .filter(|c| *c != canvas_char)
                    .map(|c| (pos, c))
            })
            .collect();
        if pixels.is_empty() {
            return None;
        }
        let cursor = editor.cursor;
        Some(Self {
            original_cursor: cursor,
            cursor,
            pixels,
        })
    }

    /// Iterates over the pixels at the positions they would paste to: where
    /// they were taken, shifted by how far the cursor has moved.
    pub fn pixels(&self) -> impl '_ + Iterator<Item = (TextPosition, char)> {
        self.pixels.iter().filter_map(|(&pos, &ch)| {
            let (Some(row), Some(col)) = (
                (pos.row + self.cursor.row).checked_sub(self.original_cursor.row),
                (pos.col + self.cursor.col).checked_sub(self.original_cursor.col),
            ) else {
                return None;
            };

            let pos = TextPosition { row, col };
            Some((pos, ch))
        })
    }

    /// Returns the pixel that would paste at `pos`, or `None` when nothing
    /// lands there.
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
