use tuinix::{TerminalFrame, TerminalPosition, TerminalSize};

pub trait TerminalFrameExt {
    fn draw_in_region<F, E>(&mut self, region: TerminalRegion, f: F) -> Result<(), E>
    where
        F: FnOnce(&mut TerminalFrame) -> Result<(), E>;
}

impl TerminalFrameExt for TerminalFrame {
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
