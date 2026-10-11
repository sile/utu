//! The `utu` editor: the buffer model, the widgets that paint it, and the
//! key bindings that drive it.
//!
//! The core here is Sans I/O: it holds the editor state, the text buffer, and
//! the renderers that paint a [`Frame`](tuinix::Frame) from them. Reading and
//! writing the terminal and the edited file happen at the edge, in the
//! binary's `main` and [`app`] module.
//!
//! [`app`] is the one exception to Sans I/O: it owns the terminal driver and
//! the poll loop, and so needs `libc::poll`, which is `unsafe`. That is the
//! single call that cannot avoid it, so the crate relaxes the default
//! `forbid` to `deny` and marks that call with `#[expect]`.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod app;
mod buffer;
mod clipboard;
mod config;
mod editor;
mod editor_command;
mod error;
mod keybinding;
mod marker;
mod tuinix_ext;
mod widget_legend;
mod widget_message;
mod widget_preview;
mod widget_status;
mod widget_text;

pub use app::App;
pub use buffer::{TextBuffer, TextBufferFilter, TextPosition};
pub use clipboard::Clipboard;
pub use config::{Color, Config, FrameSize, Palette};
pub use editor::{Dirty, Editor};
pub use editor_command::EditorCommand;
pub use error::{Error, Result};
pub use keybinding::{KeyBindingEntry, KeyBindings, KeyBindingsGroup, KeySequence, KeySet};
pub use marker::Marker;
pub use widget_legend::Legend;
pub use widget_message::MessageLine;
pub use widget_preview::Preview;
pub use widget_status::StatusLine;
pub use widget_text::TextView;
