use tuinix::KeyInput;

use crate::{editor_command::EditorCommand, nojson_ext::RawJsonValueExt, tuinix_ext::KeyInputExt};

#[derive(Debug)]
pub struct KeyBinding {
    pub sequence: KeySequence,
    pub command: EditorCommand,
}

#[derive(Debug)]
pub struct KeySequence(pub Vec<KeyInput>);

impl KeySequence {
    fn new(key: KeyInput) -> Self {
        Self(vec![key])
    }
}

#[derive(Debug)]
pub struct KeyBindings(pub Vec<KeyBinding>);

impl KeyBindings {
    fn parse(&mut self, value: nojson::RawJsonValue<'_, '_>) -> Result<(), nojson::JsonParseError> {
        for (keys, command_or_children) in value.to_object()? {
            if let Ok(command) = command_or_children.to_unquoted_string_str() {
                let command: EditorCommand = command
                    .parse()
                    .map_err(|e| command_or_children.invalid(e))?;
                for key in keys.to_unquoted_string_str()?.split(',') {
                    let key = KeyInput::from_str(key).ok_or_else(|| keys.invalid("invalid key"))?;
                    let binding = KeyBinding {
                        sequence: KeySequence::new(key),
                        command,
                    };
                    self.0.push(binding);
                }
            } else if command_or_children.kind().is_object() {
                let mut children = KeyBindings(Vec::new());
                children.parse(command_or_children)?;
                for child in children.0 {
                    for key in keys.to_unquoted_string_str()?.split(',') {
                        let key =
                            KeyInput::from_str(key).ok_or_else(|| keys.invalid("invalid key"))?;
                        let binding = KeyBinding {
                            sequence: KeySequence(
                                std::iter::once(key)
                                    .chain(child.sequence.0.iter().copied())
                                    .collect(),
                            ),
                            command: child.command,
                        };
                        self.0.push(binding);
                    }
                }
            } else {
                return Err(command_or_children.invalid("expected JSON string or object"));
            }
        }
        Ok(())
    }
}

impl<'text> nojson::FromRawJsonValue<'text> for KeyBindings {
    fn from_raw_json_value(
        value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        let mut bindings = KeyBindings(Vec::new());
        bindings.parse(value)?;
        Ok(bindings)
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let json = include_str!("../default.keys.json");
        let nojson::Json(bindings) = json.parse().expect("bug");
        bindings
    }
}
