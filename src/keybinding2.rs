use std::collections::BTreeMap;

use nojson::{JsonParseError, RawJsonValue};
use tuinix::KeyInput;

use crate::{editor_command::EditorCommand, nojson_ext::RawJsonValueExt};

#[derive(Debug)]
pub struct KeyBindings {
    pub main: KeyBindingsGroup,
    pub groups: BTreeMap<String, KeyBindingsGroup>,
}

impl<'text> nojson::FromRawJsonValue<'text> for KeyBindings {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let mut groups = RawKeyBindings::parse(value)?.process()?;

        let main = groups
            .remove("__main__")
            .ok_or_else(|| value.invalid("missing __main__ group"))?;

        Ok(KeyBindings { main, groups })
    }
}

#[derive(Debug)]
pub struct KeyBindingsGroup {
    pub entries: Vec<KeyBindingEntry>,
}

#[derive(Debug)]
pub struct KeyBindingEntry {
    pub input: KeySet,
    pub command: EditorCommand,
}

#[derive(Debug)]
pub struct KeySet(pub Vec<KeyInput>);

#[derive(Debug)]
pub struct RawKeyBindings<'text, 'a> {
    pub groups: BTreeMap<String, RawKeyBindingsGroup<'text, 'a>>,
}

impl<'text, 'a> RawKeyBindings<'text, 'a> {
    pub fn parse(root: RawJsonValue<'text, 'a>) -> Result<Self, JsonParseError> {
        let groups = root
            .to_object()?
            .map(|(k, v)| {
                let name = k.to_unquoted_string_str()?;
                if name.starts_with("__")
                    && name.ends_with("__")
                    && !matches!(name.as_ref(), "__main__" | "__global__")
                {
                    return Err(k.invalid("no such built-in group"));
                }

                Ok((name.into_owned(), RawKeyBindingsGroup::parse(v)?))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { groups })
    }

    fn process(&self) -> Result<BTreeMap<String, KeyBindingsGroup>, JsonParseError> {
        let mut groups = BTreeMap::new();
        for (name, raw_group) in &self.groups {
            let group = raw_group.process(self)?;
            groups.insert(name.clone(), group);
        }
        Ok(groups)
    }
}

#[derive(Debug)]
pub struct RawKeyBindingsGroup<'text, 'a> {
    entries: Vec<RawKeyBindingEntry<'text, 'a>>,
}

impl<'text, 'a> RawKeyBindingsGroup<'text, 'a> {
    pub fn parse(value: RawJsonValue<'text, 'a>) -> Result<Self, JsonParseError> {
        let entries = value
            .to_object()?
            .map(|(key, value)| RawKeyBindingEntry { key, value })
            .collect();
        Ok(Self { entries })
    }

    fn process(&self, raw: &RawKeyBindings<'text, 'a>) -> Result<KeyBindingsGroup, JsonParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct RawKeyBindingEntry<'text, 'a> {
    key: RawJsonValue<'text, 'a>,
    value: RawJsonValue<'text, 'a>,
}

#[cfg(test)]
mod tests {
    use nojson::FromRawJsonValue;
    use orfail::OrFail;

    use super::*;

    #[test]
    fn parse_raw_key_bindings() -> orfail::Result<()> {
        let json = include_str!("../default.config.json");
        let json = nojson::RawJson::parse(json).or_fail()?;
        let ([keybindings], []) = json
            .value()
            .to_fixed_object(["keybindings"], [])
            .or_fail()?;
        RawKeyBindings::parse(keybindings).or_fail()?;
        Ok(())
    }

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
