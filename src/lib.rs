pub mod app;
pub mod buffer;
pub mod clipboard;
pub mod config;
pub mod editor;
pub mod editor_command;
mod error;
pub mod keybinding;
pub mod marker;
pub mod tuinix_ext;
pub mod widget_legend;
pub mod widget_message;
pub mod widget_preview;
pub mod widget_status;
pub mod widget_text;

pub use error::{Error, Result};
