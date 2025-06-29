use crate::keybinding::KeyBindings;

#[derive(Debug)]
pub struct Config {
    pub keybindings: KeyBindings,
    pub preview: PreviewConfig,
}

impl<'text> nojson::FromRawJsonValue<'text> for Config {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let ([keybindings, preview], []) = value.to_fixed_object(["keybindings", "preview"], [])?;
        Ok(Config {
            keybindings: keybindings.try_to()?,
            preview: preview.try_to()?,
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

#[derive(Debug)]
pub struct PreviewConfig {
    pub width: u32,
    pub height: u32,
}

impl<'text> nojson::FromRawJsonValue<'text> for PreviewConfig {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let ([width, height], []) = value.to_fixed_object(["width", "height"], [])?;
        Ok(PreviewConfig {
            width: width.try_to()?,
            height: height.try_to()?,
        })
    }
}
