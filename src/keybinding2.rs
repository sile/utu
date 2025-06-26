use std::collections::{BTreeMap, BTreeSet};

use tuinix::KeyInput;

use crate::{
    editor_command::EditorCommand, keybinding::KeySequence, nojson_ext::RawJsonValueExt,
    tuinix_ext::KeyInputExt,
};

#[derive(Debug)]
pub struct KeyBindings {
    pub main: KeyBindingsGroup,
    pub global: Option<KeyBindingsGroup>,
    pub groups: BTreeMap<String, KeyBindingsGroup>,
}

impl KeyBindings {
    // TODO: return a reference
    pub fn find(&self, keys: &KeySequence) -> Result<Option<EditorCommand>, ()> {
        self.find_in_group(&self.main, &keys.0)
    }

    fn find_in_scope(
        &self,
        scope: Option<&str>,
        keys: &[KeyInput],
    ) -> Result<Option<EditorCommand>, ()> {
        let group = match scope {
            Some(scope_name) => self.groups.get(scope_name).ok_or(())?,
            None => &self.main,
        };

        self.find_in_group(group, keys)
    }

    fn find_in_group(
        &self,
        group: &KeyBindingsGroup,
        keys: &[KeyInput],
    ) -> Result<Option<EditorCommand>, ()> {
        let mut has_prefix = false;

        for entry in &group.entries {
            if entry.keys.0 == keys {
                return Ok(Some(entry.command.clone()));
            }
            if entry.keys.0.starts_with(keys) {
                has_prefix = true;
            }
        }

        // Check global bindings if no match found in current group
        if let Some(global) = &self.global {
            for entry in &global.entries {
                if entry.keys.0 == keys {
                    return Ok(Some(entry.command.clone()));
                }
                if entry.keys.0.starts_with(keys) {
                    has_prefix = true;
                }
            }
        }

        if has_prefix {
            Ok(None) // Partial match, need more keys
        } else {
            Err(()) // No match at all
        }
    }
}

impl<'text> nojson::FromRawJsonValue<'text> for KeyBindings {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let group_names = value
            .to_object()?
            .map(|(k, _)| Ok(k.to_unquoted_string_str()?.into_owned()))
            .collect::<Result<BTreeSet<_>, _>>()?;

        let mut groups = BTreeMap::new();
        for (raw_name, raw_group) in value.to_object()? {
            let name = raw_name.to_unquoted_string_str()?;
            if name.starts_with("__")
                && name.ends_with("__")
                && !matches!(name.as_ref(), "__main__" | "__global__")
            {
                return Err(raw_name.invalid("no such built-in group"));
            }

            let group = KeyBindingsGroup::parse(raw_group, &group_names)?;
            groups.insert(name.into_owned(), group);
        }

        let main = groups
            .remove("__main__")
            .ok_or_else(|| value.invalid("missing __main__ group"))?;
        let global = groups.remove("__global__");
        Ok(KeyBindings {
            main,
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

#[derive(Debug)]
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
