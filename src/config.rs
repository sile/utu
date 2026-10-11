//! The editor's compile-time configuration: the preview size and the palette.
//!
//! The key bindings live in [`crate::keybinding`], and nothing here is read
//! from a file: every value is a constant in this module.

use std::collections::BTreeMap;

use crate::keybinding::KeyBindings;

/// The default preview width, in pixels.
pub const PREVIEW_WIDTH: usize = 40;

/// The default preview height, in pixels.
pub const PREVIEW_HEIGHT: usize = 40;

/// The character-to-color palette, as `(character, rrggbb)` pairs.
///
/// Each color is a `#RRGGBB` hex string; the alpha channel is always opaque.
const RAW_PALETTE: &[(char, &str)] = &[
    (' ', "C8C8A8"),
    ('.', "B8B8A8"),
    ('+', "A0A088"),
    ('o', "686850"),
    ('#', "282818"),
];

/// Everything the editor is configured with.
#[derive(Debug)]
pub struct Config {
    /// The key bindings, hard-coded in [`crate::keybinding`].
    pub keybindings: KeyBindings,

    /// The preview frame size, in pixels.
    pub preview: FrameSize,

    /// The character-to-color palette used by the preview.
    pub palette: Palette,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            keybindings: KeyBindings::default(),
            preview: FrameSize {
                width: PREVIEW_WIDTH,
                height: PREVIEW_HEIGHT,
            },
            palette: Palette::default(),
        }
    }
}

/// The size of the preview frame, in pixels.
#[derive(Debug)]
pub struct FrameSize {
    /// The width in pixels.
    pub width: usize,

    /// The height in pixels.
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

/// The character-to-color palette the preview draws with.
#[derive(Debug)]
pub struct Palette {
    /// The color of each character, keyed by the character itself.
    pub colors: BTreeMap<char, Color>,
}

impl Default for Palette {
    fn default() -> Self {
        let colors = RAW_PALETTE
            .iter()
            .map(|&(ch, hex)| (ch, Color::from_hex(hex).expect("bug: bad palette entry")))
            .collect();
        Palette { colors }
    }
}

/// A 32-bit RGBA color.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// The red channel.
    pub r: u8,

    /// The green channel.
    pub g: u8,

    /// The blue channel.
    pub b: u8,

    /// The alpha channel.
    pub a: u8,
}

impl Color {
    /// Makes an opaque [`Color`] from its red, green, and blue channels.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Makes a [`Color`] from its four channels.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Parses a `RRGGBB` or `RRGGBBAA` hex string, without a leading `#`.
    fn from_hex(hex: &str) -> Result<Self, String> {
        let parse = |range: std::ops::Range<usize>| {
            u8::from_str_radix(&hex[range], 16).map_err(|_| "invalid hex color".to_string())
        };
        match hex.len() {
            6 => Ok(Color::rgba(parse(0..2)?, parse(2..4)?, parse(4..6)?, 255)),
            8 => Ok(Color::rgba(
                parse(0..2)?,
                parse(2..4)?,
                parse(4..6)?,
                parse(6..8)?,
            )),
            _ => Err("color must be 6 or 8 hex digits".to_string()),
        }
    }
}
