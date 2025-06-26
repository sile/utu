use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

use crate::nojson_ext::RawJsonValueExt;

#[derive(Debug)]
pub struct KeyBindings {
    pub groups: BTreeMap<String, GroupedKeyBindings>,
}

impl<'text> nojson::FromRawJsonValue<'text> for KeyBindings {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let group_names = value
            .to_object()?
            .map(|(k, _)| k.to_unquoted_string_str())
            .collect::<Result<BTreeSet<_>, _>>()?;
        if !group_names.contains("__main__") {
            return Err(value.invalid("No '__main__' group"))?;
        }

        let mut parser = KeyBindingsParser {
            group_names,
            include_path: Vec::new(),
        };
        parser.parse(value)?;
        // let group = value.try_to()?;
        // Ok(KeyBindings { groups: group })
        todo!()
    }
}

#[derive(Debug)]
struct KeyBindingsParser<'text> {
    group_names: BTreeSet<Cow<'text, str>>,
    include_path: Vec<Cow<'text, str>>,
}

impl<'text> KeyBindingsParser<'text> {
    fn parse(
        &mut self,
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<(), nojson::JsonParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct GroupedKeyBindings {
    //
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
