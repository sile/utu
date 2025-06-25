use crate::key_binding::KeyBindings;

#[derive(Debug, Default)]
pub struct Config {
    pub keybindings: KeyBindings,
}

impl<'text> nojson::FromRawJsonValue<'text> for Config {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let ([], [keybindings]) = value.to_fixed_object([], ["keybindings"])?;
        Ok(Config {
            keybindings: keybindings
                .map(|v| v.try_to())
                .transpose()?
                .unwrap_or_default(),
        })
    }
}
