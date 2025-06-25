use std::collections::BTreeSet;

use crate::buffer::TextPosition;

#[derive(Debug, Clone)]
pub enum Marker {
    Stroke(StrokeMarker),
    Line(LineMarker),
    Rect(RectMarker),
    Fill(FillMarker),
}

impl Marker {
    pub fn new_stroke() -> Self {
        Self::Stroke(StrokeMarker::new())
    }

    pub fn new_line() -> Self {
        Self::Line(LineMarker::new())
    }

    pub fn new_rect() -> Self {
        Self::Rect(RectMarker::new())
    }

    pub fn new_fill() -> Self {
        Self::Fill(FillMarker::new())
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
            Marker::Line(m) => todo!(),
            Marker::Rect(m) => todo!(),
            Marker::Fill(m) => Box::new(m.filled_positions.iter().copied()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrokeMarker {
    positions: BTreeSet<TextPosition>,
}

impl StrokeMarker {
    fn new() -> Self {
        Self {
            positions: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineMarker {
    start: Option<TextPosition>,
    end: Option<TextPosition>,
}

impl LineMarker {
    fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RectMarker {
    start: Option<TextPosition>,
    end: Option<TextPosition>,
}

impl RectMarker {
    fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FillMarker {
    position: Option<TextPosition>,
    target_char: Option<char>,
    filled_positions: BTreeSet<TextPosition>,
}

impl FillMarker {
    fn new() -> Self {
        Self {
            position: None,
            target_char: None,
            filled_positions: BTreeSet::new(),
        }
    }
}
