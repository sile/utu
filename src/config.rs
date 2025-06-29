use std::collections::BTreeMap;

use crate::keybinding::KeyBindings;

#[derive(Debug)]
pub struct Config {
    pub keybindings: KeyBindings,
    pub preview: PreviewConfig,
    pub palette: Palette,
}

impl<'text> nojson::FromRawJsonValue<'text> for Config {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let ([keybindings, preview, palette], []) =
            value.to_fixed_object(["keybindings", "preview", "palette"], [])?;
        Ok(Config {
            keybindings: keybindings.try_to()?,
            preview: preview.try_to()?, // TODO: optional
            palette: palette.try_to()?,
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
    pub width: usize,
    pub height: usize,
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

#[derive(Debug)]
pub struct Palette {
    pub colors: BTreeMap<char, Color>,
}

impl<'text> nojson::FromRawJsonValue<'text> for Palette {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let colors = value.try_to()?;
        Ok(Palette { colors })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl<'text> nojson::FromRawJsonValue<'text> for Color {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let hex_string = value.to_unquoted_string_str()?;

        // Parse hex color string (e.g., "#FFFFFF" or "#000000" or "#FFFFFFAA")
        let hex = hex_string
            .strip_prefix('#')
            .ok_or_else(|| value.invalid("Color must start with #"))?;

        // Support both #RRGGBB and #RRGGBBAA formats
        let (r, g, b, a) = match hex.len() {
            6 => {
                // #RRGGBB format - default to full opacity
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| value.invalid("Invalid hex color format"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| value.invalid("Invalid hex color format"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| value.invalid("Invalid hex color format"))?;
                (r, g, b, 255)
            }
            8 => {
                // #RRGGBBAA format - includes alpha channel
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| value.invalid("Invalid hex color format"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| value.invalid("Invalid hex color format"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| value.invalid("Invalid hex color format"))?;
                let a = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|_| value.invalid("Invalid hex color format"))?;
                (r, g, b, a)
            }
            _ => {
                return Err(value.invalid("Color must be 6 or 8 hex digits"));
            }
        };

        Ok(Color::rgba(r, g, b, a))
    }
}
