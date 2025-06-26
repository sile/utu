use crate::keybinding::KeyBindings;

#[derive(Debug)]
pub struct Config {
    pub keybindings: KeyBindings,
}

impl<'text> nojson::FromRawJsonValue<'text> for Config {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let ([keybindings], []) = value.to_fixed_object(["keybindings"], [])?;
        Ok(Config {
            keybindings: keybindings.try_to()?,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        let json = include_str!("../default.config.json");
        let nojson::Json(config) = json.parse().expect("bug");
        config
    }
}
