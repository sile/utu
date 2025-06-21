use std::path::PathBuf;

use orfail::OrFail;
use tuinix::{Terminal, TerminalEvent, TerminalFrame, TerminalInput};

use crate::{
    editor::Editor,
    tuinix_ext::{TerminalFrameExt, TerminalSizeExt},
    widget_notification::NotificationBarWidget,
    widget_status::StatusBarWidget,
};

#[derive(Debug)]
pub struct App {
    terminal: Terminal,
    editor: Editor,
    status_bar: StatusBarWidget,
    notification_bar: NotificationBarWidget,
}

impl App {
    pub fn new(path: PathBuf) -> orfail::Result<Self> {
        let terminal = Terminal::new().or_fail()?;
        Ok(Self {
            terminal,
            editor: Editor::new(path),
            status_bar: StatusBarWidget,
            notification_bar: NotificationBarWidget,
        })
    }

    pub fn run(mut self) -> orfail::Result<()> {
        while !self.editor.exit {
            self.render().or_fail()?;

            if let Some(event) = self.terminal.poll_event(None).or_fail()? {
                self.handle_event(event).or_fail()?;
            }
        }
        Ok(())
    }

    fn render(&mut self) -> orfail::Result<()> {
        if !self.editor.dirty.render {
            return Ok(());
        }

        let mut frame = TerminalFrame::new(self.terminal.size());

        // Create regions for different UI components
        let full_region = frame.size().to_region();

        // Reserve space for status bar (bottom row) and notification bar (above status bar)
        let _main_region = full_region.without_bottom_rows(2);
        let status_region = full_region.without_bottom_rows(1).bottom_rows(1);
        let notification_region = full_region.bottom_rows(1);

        // Render main editor content (if you have editor rendering logic)
        // frame.draw_in_region(main_region, |frame| {
        //     // Editor content would go here
        //     Ok(())\n        // })?;

        // Render status bar above the notification bar
        frame.draw_in_region(status_region, |frame| {
            self.status_bar.render(&self.editor, frame)
        })?;

        // Render notification bar at the bottom
        frame.draw_in_region(notification_region, |frame| {
            self.notification_bar.render(&self.editor, frame)
        })?;

        self.terminal.draw(frame).or_fail()?;

        self.editor.notification = None;
        self.editor.dirty.render = false;

        Ok(())
    }

    fn handle_event(&mut self, event: TerminalEvent) -> orfail::Result<()> {
        match event {
            TerminalEvent::Input(input) => {
                let TerminalInput::Key(key) = input;
                if key.ctrl && matches!(key.code, tuinix::KeyCode::Char('c')) {
                    self.editor.exit = true;
                }
            }
            TerminalEvent::Resize(_) => {
                self.editor.dirty.render = true;
            }
        }
        Ok(())
    }
}
