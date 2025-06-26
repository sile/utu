use std::collections::BTreeMap;

use nojson::{JsonParseError, RawJsonValue};

use crate::nojson_ext::RawJsonValueExt;

#[derive(Debug)]
pub struct KeyBindings {
    pub main: KeyBindingsGroup,
    pub groups: BTreeMap<String, KeyBindingsGroup>,
}

impl<'text> nojson::FromRawJsonValue<'text> for KeyBindings {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let raw = RawKeyBindings::parse(value)?;
        todo!()
    }
}

#[derive(Debug)]
pub struct KeyBindingsGroup {}

#[derive(Debug)]
pub struct RawKeyBindings<'text, 'a> {
    pub root: RawJsonValue<'text, 'a>,
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
                    && !matches!(name.as_ref(), "__main__")
                {
                    return Err(k.invalid("no such built-in group"));
                }

                Ok((name.into_owned(), RawKeyBindingsGroup::parse(v)?))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { root, groups })
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
