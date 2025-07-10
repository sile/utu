use std::collections::BTreeMap;

use crate::keybinding::KeyBindings;

#[derive(Debug)]
pub struct Config {
    pub keybindings: KeyBindings,
    pub preview: FrameSize,
    // TODO: use a map to be able to switch palettes
    pub palette: Palette,
}

impl<'text, 'raw> TryFrom<nojson::RawJsonValue<'text, 'raw>> for Config {
    type Error = nojson::JsonParseError;

    fn try_from(value: nojson::RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let keybindings = value.to_member("keybindings")?.required()?;
        let preview = value.to_member("preview")?.required()?;
        let palette = value.to_member("palette")?.required()?;

        Ok(Config {
            keybindings: keybindings.try_into()?,
            preview: preview.try_into()?, // TODO: optional
            palette: palette.try_into()?,
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
pub struct FrameSize {
    pub width: usize,
    pub height: usize,
}

impl std::str::FromStr for FrameSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width, height) = s
            .split_once('x')
            .ok_or_else(|| format!("Invalid format: expected 'WIDTHxHEIGHT', got '{}'", s))?;
        let width = width
            .parse()
            .map_err(|e| format!("Invalid width '{}': {}", width, e))?;
        let height = height
            .parse()
            .map_err(|e| format!("Invalid height '{}': {}", height, e))?;
        Ok(FrameSize { width, height })
    }
}

impl<'text, 'raw> TryFrom<nojson::RawJsonValue<'text, 'raw>> for FrameSize {
    type Error = nojson::JsonParseError;

    fn try_from(value: nojson::RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let width = value.to_member("width")?.required()?;
        let height = value.to_member("height")?.required()?;

        Ok(FrameSize {
            width: width.try_into()?,
            height: height.try_into()?,
        })
    }
}

#[derive(Debug)]
pub struct Palette {
    pub colors: BTreeMap<char, Color>,
}

impl<'text, 'raw> TryFrom<nojson::RawJsonValue<'text, 'raw>> for Palette {
    type Error = nojson::JsonParseError;

    fn try_from(value: nojson::RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let colors = value.try_into()?;
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

impl<'text, 'raw> TryFrom<nojson::RawJsonValue<'text, 'raw>> for Color {
    type Error = nojson::JsonParseError;

    fn try_from(value: nojson::RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
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
