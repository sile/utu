use std::fmt::Write;

use orfail::OrFail;
use tuinix::{TerminalFrame, TerminalSize};

use crate::{
    editor::Editor,
    tuinix_ext::{TerminalRegion, TerminalSizeExt, UnicodeCharWidthEstimator},
};

#[derive(Debug, Default)]
pub struct Preview {
    pub hide: bool,
}

impl Preview {
    pub fn toggle_hide(&mut self, editor: &mut Editor) {
        self.hide = !self.hide;
        if self.hide {
            editor.set_message("Hide Preview");
        } else {
            editor.set_message("Show Preview");
        }
        editor.dirty.render = true;
    }

    pub fn render(
        &self,
        editor: &Editor,
        frame: &mut TerminalFrame<UnicodeCharWidthEstimator>,
    ) -> orfail::Result<()> {
        if frame.size().cols != self.size(editor).cols {
            return Ok(());
        }

        if self.hide {
            writeln!(frame, "┌───").or_fail()?;
            return Ok(());
        }

        // TODO: Implement preview rendering logic
        // This will depend on what kind of preview functionality is needed
        // For now, just render a placeholder
        writeln!(frame, "┌───").or_fail()?;

        Ok(())
    }

    pub fn size(&self, editor: &Editor) -> TerminalSize {
        if self.hide {
            TerminalSize::rows_cols(1, 4)
        } else {
            let config = &editor.config.preview;
            TerminalSize::rows_cols(config.height / 2 + 1, config.width + 1)
        }
    }

    pub fn region(&self, editor: &Editor, size: TerminalSize) -> TerminalRegion {
        let preview_size = self.size(editor);
        size.to_region()
            .without_bottom_rows(2)
            .bottom_rows(preview_size.rows)
            .right_cols(preview_size.cols)
    }
}
