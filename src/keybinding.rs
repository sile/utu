use std::collections::BTreeMap;

use crate::{
    editor_command::EditorCommand,
    tuinix_ext::{display_key, parse_key},
};

/// The key bindings are hard-coded here rather than loaded from a file: the
/// tables below are the whole set, and a key that is not spelled in one of them
/// is unbound.
///
/// Each table entry is a comma-separated list of the keys that run one command
/// (so `"\u{2191},i"` binds both Up and `i`), paired with the command's name as
/// [`EditorCommand::from_str`](std::str::FromStr::from_str) reads it. The
/// `__main__`, `__clipboard__`, and `__global__` tables are looked up by name;
/// every other name is a scope entered with a `scope(NAME)` binding.
const RAW_KEYBINDINGS: &[(&str, &[(&str, &str)])] = &[
    (
        "__global__",
        &[
            ("C-c", "quit"),
            ("C-h", "legend"),
            ("C-p", "preview"),
            ("g", "cancel"),
        ],
    ),
    (
        "__main__",
        &[
            ("C-s", "save"),
            ("C-u", "undo"),
            ("C-b", "bg(_)"),
            ("\u{2191},i", "prev-line"),
            ("\u{2193},k", "next-line"),
            ("\u{2190},j", "prev-char"),
            ("\u{2192},l", "next-char"),
            (" ", "dot( )"),
            ("a", "dot(.)"),
            ("s", "dot(+)"),
            ("d", "dot(o)"),
            ("f", "dot(#)"),
            ("w", "cut"),
            ("W", "copy"),
            ("m", "scope(marker)"),
        ],
    ),
    (
        "marker",
        &[
            ("s", "mark-stroke"),
            ("l", "mark-line"),
            ("r", "mark-rect"),
            ("R", "mark-filled-rect"),
            ("f", "mark-fill"),
        ],
    ),
    (
        "__clipboard__",
        &[
            ("C-u", "undo"),
            ("\u{2191},i", "prev-line"),
            ("\u{2193},k", "next-line"),
            ("\u{2190},j", "prev-char"),
            ("\u{2192},l", "next-char"),
            ("v", "paste"),
        ],
    ),
];

/// The keys pressed so far for a binding that is still being typed.
#[derive(Debug, Default)]
pub struct KeySequence(pub Vec<tuinix::KeyInput>);

impl KeySequence {
    /// Appends `key` to the sequence.
    pub fn push(&mut self, key: tuinix::KeyInput) {
        self.0.push(key);
    }

    /// Empties the sequence.
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl std::fmt::Display for KeySequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, key) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?
            }
            display_key(key, f)?;
        }
        Ok(())
    }
}

/// The whole set of key bindings, split into the tables the app looks up.
///
/// The tables are compiled into the binary; see [`RAW_KEYBINDINGS`].
#[derive(Debug)]
pub struct KeyBindings {
    /// The table used while no clipboard is open.
    pub main: KeyBindingsGroup,

    /// The table used while the clipboard is open.
    pub clipboard: KeyBindingsGroup,

    /// The table whose bindings are always available, if any.
    pub global: Option<KeyBindingsGroup>,

    /// The other tables, reached with a `scope(NAME)` binding.
    pub groups: BTreeMap<String, KeyBindingsGroup>,
}

impl KeyBindings {
    // The error carries no information: it only means "no binding matches".
    #[expect(
        clippy::result_unit_err,
        reason = "the failure case only means no binding matches, so a named error type would add noise"
    )]
    /// Resolves `keys` against `group`, following a `scope(NAME)` binding into
    /// its nested table.
    ///
    /// Returns `Ok(None)` while `keys` is a prefix of a longer binding, and
    /// `Err(())` when it matches no binding at all.
    pub fn find<'a>(
        &'a self,
        group: &'a KeyBindingsGroup,
        keys: &KeySequence,
    ) -> Result<Option<&'a EditorCommand>, ()> {
        self.find_in_group(group, keys.0.iter().copied())
    }

    /// Iterates over the bindings reachable from `prefix` in `group`, together
    /// with the keys that trigger them.
    pub fn possible_commands(
        &self,
        group: &KeyBindingsGroup,
        prefix: &KeySequence,
    ) -> impl Iterator<Item = (KeySet, EditorCommand)> {
        let mut results = std::collections::BTreeMap::new();

        // Check main group
        self.collect_possible_commands(group, prefix, &mut results);

        // Check global group if it exists
        if let Some(global) = &self.global {
            // Global commands are always available regardless of prefix
            let empty_prefix = KeySequence(vec![]);
            self.collect_possible_commands(global, &empty_prefix, &mut results);
        }

        results.into_iter()
    }

    fn collect_possible_commands(
        &self,
        group: &KeyBindingsGroup,
        prefix: &KeySequence,
        results: &mut std::collections::BTreeMap<KeySet, EditorCommand>,
    ) {
        for entry in &group.entries {
            for &key in &entry.keys.0 {
                if prefix.0.is_empty() {
                    // No prefix, so this key is a possible first key
                    // Always insert the command (whether it's complete or Scope)
                    results.insert(entry.keys.clone(), entry.command.clone()); // TODO} remove clone
                } else if prefix.0.len() == 1 && prefix.0[0] == key {
                    // This key matches our prefix, check what comes next
                    if let EditorCommand::Scope(scope_name) = &entry.command
                        && let Some(scoped_group) = self.groups.get(scope_name)
                    {
                        let empty_prefix = KeySequence(vec![]);
                        self.collect_possible_commands(scoped_group, &empty_prefix, results);
                    }
                }
            }
        }
    }

    /// Iterates over the characters that the `dot(CHAR)` bindings write.
    pub fn fg_chars(&self) -> impl Iterator<Item = char> {
        self.all_commands().filter_map(|c| {
            if let EditorCommand::Dot(c) = c {
                Some(*c)
            } else {
                None
            }
        })
    }

    /// Returns the canvas character: the first character a `dot(CHAR)` binding
    /// writes, or a space when there is none.
    pub fn canvas_char(&self) -> char {
        self.fg_chars().next().unwrap_or(' ')
    }

    fn all_commands(&self) -> impl Iterator<Item = &EditorCommand> {
        self.main
            .entries
            .iter()
            .map(|entry| &entry.command)
            .chain(
                self.global
                    .iter()
                    .flat_map(|group| group.entries.iter())
                    .map(|entry| &entry.command),
            )
            .chain(
                self.groups
                    .values()
                    .flat_map(|group| group.entries.iter())
                    .map(|entry| &entry.command),
            )
    }

    fn find_in_group<'a>(
        &'a self,
        group: &'a KeyBindingsGroup,
        mut keys: impl Iterator<Item = tuinix::KeyInput>,
    ) -> Result<Option<&'a EditorCommand>, ()> {
        let Some(key) = keys.next() else {
            return Ok(None);
        };

        for entry in group
            .entries
            .iter()
            .chain(self.global.iter().flat_map(|x| x.entries.iter()))
        {
            if !entry.keys.0.contains(&key) {
                continue;
            }
            if let EditorCommand::Scope(name) = &entry.command {
                let group = self.groups.get(name).expect("bug");
                return self.find_in_group(group, keys);
            } else {
                return Ok(Some(&entry.command));
            }
        }

        Err(())
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::from_raw(RAW_KEYBINDINGS)
    }
}

impl KeyBindings {
    /// Builds the bindings from the hard-coded [`RAW_KEYBINDINGS`] tables.
    ///
    /// The three tables named `__main__`, `__clipboard__`, and `__global__`
    /// become the fields of the same name; every other table becomes a scope
    /// reached with a `scope(NAME)` binding.
    ///
    /// The tables are compiled into the binary, so a malformed entry is a bug
    /// rather than user input: the key and command parsers are the same ones
    /// the tables are written against, and a failure here panics.
    pub fn from_raw(raw: &[(&str, &[(&str, &str)])]) -> Self {
        let mut groups = BTreeMap::new();
        for &(name, entries) in raw {
            let group = KeyBindingsGroup::from_raw(entries);
            groups.insert(name.to_owned(), group);
        }

        let main = groups.remove("__main__").expect("bug: no __main__ group");
        let clipboard = groups
            .remove("__clipboard__")
            .expect("bug: no __clipboard__ group");
        let global = groups.remove("__global__");
        KeyBindings {
            main,
            clipboard,
            global,
            groups,
        }
    }
}

/// One table of key bindings.
#[derive(Debug)]
pub struct KeyBindingsGroup {
    /// The bindings, in the order the legend lists them.
    pub entries: Vec<KeyBindingEntry>,
}

impl KeyBindingsGroup {
    fn from_raw(raw: &[(&str, &str)]) -> Self {
        let entries = raw
            .iter()
            .map(|&(keys, command)| KeyBindingEntry::from_raw(keys, command))
            .collect();
        Self { entries }
    }
}

/// One binding: the keys that trigger it and the command they run.
#[derive(Debug)]
pub struct KeyBindingEntry {
    /// The keys that trigger the binding; any one of them is enough.
    pub keys: KeySet,

    /// The command the keys run.
    pub command: EditorCommand,
}

impl KeyBindingEntry {
    fn from_raw(raw_keys: &str, raw_command: &str) -> Self {
        let keys = KeySet::from_raw(raw_keys);
        let command = raw_command.parse::<EditorCommand>().expect("bug");
        Self { keys, command }
    }
}

/// The keys that trigger one binding, any one of which is enough.
///
/// The order is the order the legend shows them in.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeySet(pub Vec<tuinix::KeyInput>);

impl KeySet {
    fn from_raw(raw_keys: &str) -> Self {
        let keys = raw_keys
            .split(',')
            .map(|key| parse_key(key).expect("bug"))
            .collect();
        Self(keys)
    }
}

impl std::fmt::Display for KeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, key) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            display_key(key, f)?;
        }
        Ok(())
    }
}
