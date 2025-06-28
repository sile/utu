use std::collections::{BTreeMap, BTreeSet};

use tuinix::KeyInput;

use crate::{editor_command::EditorCommand, nojson_ext::RawJsonValueExt, tuinix_ext::KeyInputExt};

#[derive(Debug, Default)]
pub struct KeySequence(pub Vec<KeyInput>);

impl KeySequence {
    pub fn push(&mut self, key: KeyInput) {
        self.0.push(key);
    }

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
            key.display(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyBindings {
    pub main: KeyBindingsGroup,
    pub clipboard: KeyBindingsGroup,
    pub global: Option<KeyBindingsGroup>,
    pub groups: BTreeMap<String, KeyBindingsGroup>,
}

impl KeyBindings {
    pub fn find<'a>(
        &'a self,
        group: &'a KeyBindingsGroup,
        keys: &KeySequence,
    ) -> Result<Option<&'a EditorCommand>, ()> {
        self.find_in_group(group, keys.0.iter().copied())
    }

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
                    if let EditorCommand::Scope(scope_name) = &entry.command {
                        if let Some(scoped_group) = self.groups.get(scope_name) {
                            let empty_prefix = KeySequence(vec![]);
                            self.collect_possible_commands(scoped_group, &empty_prefix, results);
                        }
                    }
                }
            }
        }
    }

    pub fn fg_chars(&self) -> impl Iterator<Item = char> {
        self.all_commands().filter_map(|c| {
            if let EditorCommand::Dot(c) = c {
                Some(*c)
            } else {
                None
            }
        })
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
        mut keys: impl Iterator<Item = KeyInput>,
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

impl<'text> nojson::FromRawJsonValue<'text> for KeyBindings {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let mut group_names = value
            .to_object()?
            .map(|(k, _)| Ok(k.to_unquoted_string_str()?.into_owned()))
            .collect::<Result<BTreeSet<_>, _>>()?;
        group_names.retain(|n| {
            !matches!(
                n.as_str(),
                "__main__" | "__global__" | "__clipboard__" | "__comment__"
            )
        });

        let mut groups = BTreeMap::new();
        for (raw_name, raw_group) in value.to_object()? {
            let name = raw_name.to_unquoted_string_str()?;
            if name == "__comment__" {
                continue;
            }

            if name.starts_with("__")
                && name.ends_with("__")
                && !matches!(name.as_ref(), "__main__" | "__global__" | "__clipboard__")
            {
                return Err(raw_name.invalid("no such built-in group"));
            }

            let group = KeyBindingsGroup::parse(raw_group, &group_names)?;
            groups.insert(name.into_owned(), group);
        }

        let main = groups
            .remove("__main__")
            .ok_or_else(|| value.invalid("missing __main__ group"))?;
        let clipboard = groups
            .remove("__clipboard__")
            .ok_or_else(|| value.invalid("missing __clipboard__ group"))?;
        let global = groups.remove("__global__");
        Ok(KeyBindings {
            main,
            clipboard,
            global,
            groups,
        })
    }
}

#[derive(Debug)]
pub struct KeyBindingsGroup {
    pub entries: Vec<KeyBindingEntry>,
}

impl KeyBindingsGroup {
    fn parse(
        raw_entries: nojson::RawJsonValue<'_, '_>,
        group_names: &BTreeSet<String>,
    ) -> Result<Self, nojson::JsonParseError> {
        let mut entries = Vec::new();
        for (key, value) in raw_entries.to_object()? {
            let entry = KeyBindingEntry::parse(key, value, group_names)?;
            entries.push(entry);
        }
        Ok(Self { entries })
    }
}

#[derive(Debug)]
pub struct KeyBindingEntry {
    pub keys: KeySet,
    pub command: EditorCommand,
}

impl KeyBindingEntry {
    fn parse(
        raw_keys: nojson::RawJsonValue<'_, '_>,
        raw_command: nojson::RawJsonValue<'_, '_>,
        group_names: &BTreeSet<String>,
    ) -> Result<Self, nojson::JsonParseError> {
        let keys = KeySet::parse(raw_keys)?;
        let command = raw_command
            .to_unquoted_string_str()?
            .parse::<EditorCommand>()
            .map_err(|e| raw_command.invalid(e))?;
        if let EditorCommand::Scope(group_name) = &command {
            if !group_names.contains(group_name) {
                return Err(raw_command.invalid("no such group"));
            }
        }
        Ok(Self { keys, command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeySet(pub Vec<KeyInput>);

impl KeySet {
    fn parse(raw_keys: nojson::RawJsonValue<'_, '_>) -> Result<Self, nojson::JsonParseError> {
        let mut keys = Vec::new();
        for key in raw_keys.to_unquoted_string_str()?.split(',') {
            let key = KeyInput::from_str(key).ok_or_else(|| raw_keys.invalid("invalid key"))?;
            keys.push(key);
        }
        Ok(Self(keys))
    }
}

impl std::fmt::Display for KeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, key) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            key.display(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use nojson::FromRawJsonValue;
    use orfail::OrFail;

    use super::*;

    #[test]
    fn parse_key_bindings() -> orfail::Result<()> {
        let json = include_str!("../default.config.json");
        let json = nojson::RawJson::parse(json).or_fail()?;
        let ([keybindings], []) = json
            .value()
            .to_fixed_object(["keybindings"], [])
            .or_fail()?;
        KeyBindings::from_raw_json_value(keybindings).or_fail()?;
        Ok(())
    }
}
