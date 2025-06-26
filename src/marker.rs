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
            Marker::Line(_m) => todo!(),
            Marker::Rect(_m) => todo!(),
            Marker::Fill(m) => Box::new(m.filled_positions.iter().copied()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrokeMarker {
    positions: BTreeSet<TextPosition>,
}

impl StrokeMarker {
    fn new(_editor: &Editor) -> Self {
        Self {
            positions: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
#[expect(dead_code)]
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
}

#[derive(Debug, Clone)]
#[expect(dead_code)]
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
}

#[derive(Debug, Clone)]
#[expect(dead_code)]
pub struct FillMarker {
    position: Option<TextPosition>,
    target_char: Option<char>,
    filled_positions: BTreeSet<TextPosition>,
}

impl FillMarker {
    fn new(editor: &Editor) -> Self {
        Self {
            position: Some(editor.cursor),
            target_char: None, // Could be initialized from buffer at cursor position
            filled_positions: BTreeSet::new(),
        }
    }
}
