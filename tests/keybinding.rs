//! Tests for the hard-coded key bindings.
//!
//! The bindings are constants in the crate, so these tests state the look the
//! bindings must have rather than agreeing with whatever the crate happens to
//! define: the keys and commands are spelled here, and a change to the tables
//! that breaks one of them fails a test.

use utu::{Config, Editor, EditorCommand, KeyBindings, KeySequence};

/// A key with no modifiers.
fn ch(c: char) -> tuinix::KeyInput {
    tuinix::KeyInput {
        ctrl: false,
        alt: false,
        code: tuinix::KeyCode::Char(c),
    }
}

/// A control key.
fn ctrl(c: char) -> tuinix::KeyInput {
    tuinix::KeyInput {
        ctrl: true,
        alt: false,
        code: tuinix::KeyCode::Char(c),
    }
}

/// Resolves `keys` against the main table.
fn resolve_main(bindings: &KeyBindings, keys: &[tuinix::KeyInput]) -> Option<EditorCommand> {
    let sequence = KeySequence(keys.to_vec());
    bindings
        .find(&bindings.main, &sequence)
        .expect("a bound key resolves")
        .cloned()
}

#[test]
fn the_default_bindings_have_a_main_and_a_clipboard_table() {
    let bindings = KeyBindings::default();
    assert!(!bindings.main.entries.is_empty());
    assert!(!bindings.clipboard.entries.is_empty());
}

#[test]
fn a_plain_key_runs_its_command() {
    let bindings = KeyBindings::default();
    assert!(matches!(
        resolve_main(&bindings, &[ch('a')]),
        Some(EditorCommand::Dot('.'))
    ));
    assert!(matches!(
        resolve_main(&bindings, &[ch('s')]),
        Some(EditorCommand::Dot('+'))
    ));
}

#[test]
fn a_control_key_runs_its_command() {
    let bindings = KeyBindings::default();
    assert!(matches!(
        resolve_main(&bindings, &[ctrl('s')]),
        Some(EditorCommand::Save)
    ));
    assert!(matches!(
        resolve_main(&bindings, &[ctrl('u')]),
        Some(EditorCommand::Undo)
    ));
}

#[test]
fn the_main_and_clipboard_tables_share_the_arrow_bindings() {
    let bindings = KeyBindings::default();
    for key in [tuinix::KeyCode::Up, tuinix::KeyCode::Left] {
        let input = tuinix::KeyInput {
            ctrl: false,
            alt: false,
            code: key,
        };
        assert!(resolve_main(&bindings, &[input]).is_some());
        let sequence = KeySequence(vec![input]);
        assert!(
            bindings
                .find(&bindings.clipboard, &sequence)
                .expect("a bound key resolves")
                .is_some()
        );
    }
}

#[test]
fn a_scope_binding_enters_its_table() {
    let bindings = KeyBindings::default();
    // `m` alone is a scope, so it resolves to `Ok(None)` rather than a command.
    let sequence = KeySequence(vec![ch('m')]);
    assert!(
        bindings
            .find(&bindings.main, &sequence)
            .expect("a scope key resolves")
            .is_none()
    );
}

#[test]
fn an_unbound_key_fails_to_resolve() {
    let bindings = KeyBindings::default();
    let sequence = KeySequence(vec![ctrl('q')]);
    assert!(bindings.find(&bindings.main, &sequence).is_err());
}

#[test]
fn the_canvas_character_is_the_first_dot_character() {
    let bindings = KeyBindings::default();
    assert_eq!(bindings.canvas_char(), ' ');
}

#[test]
fn the_foreground_characters_are_the_dot_characters() {
    let bindings = KeyBindings::default();
    let fg: Vec<char> = bindings.fg_chars().collect();
    assert_eq!(fg, vec![' ', '.', '+', 'o', '#']);
}

/// The default config wires the binding's characters into the buffer filter, so
/// an editor built from it accepts the canvas characters.
#[test]
fn the_default_config_wires_the_bindings_into_the_editor() {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("keybinding");
    std::fs::create_dir_all(&dir).expect("create scratch directory");
    let path = dir.join("editor.txt");
    std::fs::write(&path, "ab\n").expect("write scratch file");

    let editor = Editor::new(path, Config::default()).expect("make editor");
    assert!(editor.buffer.filter.fg_chars.contains(&'.'));
}
