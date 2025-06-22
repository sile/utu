use tuinix::KeyCode;

use crate::editor_command::EditorCommand;

#[derive(Debug)]
pub struct KeyBindings {}

#[derive(Debug)]
pub struct KeyBinding {
    pub path: Vec<KeyCode>, // KeyPath
    pub command: EditorCommand,
}
