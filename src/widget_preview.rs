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
        if self.hide {
            return Ok(());
        }

        // TODO: Implement preview rendering logic
        // This will depend on what kind of preview functionality is needed
        // For now, just render a placeholder
        writeln!(frame, "Preview content here").or_fail()?;

        Ok(())
    }

    pub fn size(&self, _editor: &Editor) -> TerminalSize {
        if self.hide {
            TerminalSize::rows_cols(0, 0)
        } else {
            // TODO: Calculate actual preview size based on content
            TerminalSize::rows_cols(10, 40) // Placeholder dimensions
        }
    }

    pub fn region(&self, editor: &Editor, size: TerminalSize) -> TerminalRegion {
        // let preview_size = self.size(editor);
        // size.to_region()
        //     .bottom_rows(preview_size.rows)
        //     .left_cols(preview_size.cols)
        todo!()
    }
}
