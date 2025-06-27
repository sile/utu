use std::collections::BTreeSet;

use crate::{buffer::TextPosition, editor::Editor};

#[derive(Debug, Clone)]
pub enum Marker {
    Stroke(StrokeMarker),
    Line(LineMarker),
    Rect(RectMarker),
    Fill(FillMarker),
}

impl Marker {
    pub fn new_stroke(editor: &Editor) -> Self {
        Self::Stroke(StrokeMarker::new(editor))
    }

    pub fn new_line(editor: &Editor) -> Self {
        Self::Line(LineMarker::new(editor))
    }

    pub fn new_rect(editor: &Editor) -> Self {
        Self::Rect(RectMarker::new(editor))
    }

    pub fn new_fill(editor: &Editor) -> Self {
        Self::Fill(FillMarker::new(editor))
    }

    pub fn name(&self) -> &'static str {
        match self {
            Marker::Stroke(_) => "MARK(STROKE)",
            Marker::Line(_) => "MARK(LINE)",
            Marker::Rect(_) => "MARK(RECT)",
            Marker::Fill(_) => "MARK(FILL)",
        }
    }

    pub fn marked_positions(&self) -> Box<dyn '_ + Iterator<Item = TextPosition>> {
        match self {
            Marker::Stroke(m) => Box::new(m.positions.iter().copied()),
            Marker::Line(m) => Box::new(m.marked_positions()),
            Marker::Rect(m) => Box::new(m.marked_positions()),
            Marker::Fill(m) => Box::new(m.filled_positions.iter().copied()),
        }
    }

    pub fn handle_cursor_move(&mut self, editor: &Editor) {
        match self {
            Marker::Stroke(m) => m.handle_cursor_move(editor),
            Marker::Line(m) => m.handle_cursor_move(editor),
            Marker::Rect(m) => m.handle_cursor_move(editor),
            Marker::Fill(m) => m.handle_cursor_move(editor),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrokeMarker {
    positions: BTreeSet<TextPosition>,
}

impl StrokeMarker {
    fn new(editor: &Editor) -> Self {
        Self {
            positions: [editor.cursor].into_iter().collect(),
        }
    }

    fn handle_cursor_move(&mut self, editor: &Editor) {
        self.positions.insert(editor.cursor);
    }
}

#[derive(Debug, Clone)]
pub struct LineMarker {
    start: TextPosition,
    end: TextPosition,
}

impl LineMarker {
    fn new(editor: &Editor) -> Self {
        Self {
            start: editor.cursor,
            end: editor.cursor,
        }
    }

    fn handle_cursor_move(&mut self, editor: &Editor) {
        self.end = editor.cursor;
    }

    fn marked_positions(&self) -> impl Iterator<Item = TextPosition> + '_ {
        let start = self.start;
        let end = self.end;

        // Calculate the line using simple interpolation
        let dx = end.col as i32 - start.col as i32;
        let dy = end.row as i32 - start.row as i32;

        let steps = std::cmp::max(dx.abs(), dy.abs()) as usize;

        (0..=steps).map(move |i| {
            if steps == 0 {
                return start;
            }

            let t = i as f64 / steps as f64;
            let col = start.col as f64 + t * dx as f64;
            let row = start.row as f64 + t * dy as f64;

            TextPosition {
                row: row.round() as usize,
                col: col.round() as usize,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct RectMarker {
    start: TextPosition,
    end: TextPosition,
}

impl RectMarker {
    fn new(editor: &Editor) -> Self {
        Self {
            start: editor.cursor,
            end: editor.cursor,
        }
    }

    fn handle_cursor_move(&mut self, editor: &Editor) {
        self.end = editor.cursor;
    }

    fn marked_positions(&self) -> impl Iterator<Item = TextPosition> + '_ {
        let start = self.start;
        let end = self.end;

        // Calculate rectangle bounds
        let min_row = std::cmp::min(start.row, end.row);
        let max_row = std::cmp::max(start.row, end.row);
        let min_col = std::cmp::min(start.col, end.col);
        let max_col = std::cmp::max(start.col, end.col);

        // Generate rectangle outline positions
        (min_row..=max_row).flat_map(move |row| {
            (min_col..=max_col).filter_map(move |col| {
                // Only include positions on the rectangle outline
                if row == min_row || row == max_row || col == min_col || col == max_col {
                    Some(TextPosition { row, col })
                } else {
                    None
                }
            })
        })
    }
}

#[derive(Debug, Clone)]
pub struct FillMarker {
    position: TextPosition,
    target_char: Option<char>,
    filled_positions: BTreeSet<TextPosition>,
}

impl FillMarker {
    fn new(editor: &Editor) -> Self {
        let mut marker = Self {
            position: editor.cursor,
            target_char: None, // Initialize as None, will be set on first update
            filled_positions: BTreeSet::new(),
        };
        marker.update_filled_positions(editor);
        marker
    }

    fn handle_cursor_move(&mut self, editor: &Editor) {
        self.position = editor.cursor;
        self.update_filled_positions(editor);
    }

    fn update_filled_positions(&mut self, editor: &Editor) {
        self.filled_positions.clear();

        let start_pos = self.position;

        // Get the character at the current position (None for background/empty positions)
        let target_char = editor.buffer.get_char_at(start_pos);

        // Only update if the character has changed
        if self.target_char != target_char {
            self.target_char = target_char;
        }

        if self.target_char.is_none() {
            self.filled_positions.clear();
            return;
        }

        // Perform flood fill
        self.flood_fill(editor, start_pos, self.target_char);
    }

    fn flood_fill(&mut self, editor: &Editor, start_pos: TextPosition, target_char: Option<char>) {
        let mut stack = vec![start_pos];

        while let Some(current_pos) = stack.pop() {
            // Get character at current position (None for background/empty positions)
            let current_char = editor.buffer.get_char_at(current_pos);

            // If character doesn't match target or position already visited, skip
            if current_char != target_char || self.filled_positions.contains(&current_pos) {
                continue;
            }

            // Mark this position
            self.filled_positions.insert(current_pos);

            // Add adjacent positions to stack (4-directional)
            // Up
            if current_pos.row > 0 {
                stack.push(TextPosition {
                    row: current_pos.row - 1,
                    col: current_pos.col,
                });
            }
            // Down
            stack.push(TextPosition {
                row: current_pos.row + 1,
                col: current_pos.col,
            });
            // Left
            if current_pos.col > 0 {
                stack.push(TextPosition {
                    row: current_pos.row,
                    col: current_pos.col - 1, // TODO: consider unicode width
                });
            }
            // Right
            stack.push(TextPosition {
                row: current_pos.row,
                col: current_pos.col + 1, // TODO: consider unicode width
            });
        }
    }
}
