use tuinix::KeyInput;

use crate::{editor_command::EditorCommand, nojson_ext::RawJsonValueExt, tuinix_ext::KeyInputExt};

#[derive(Debug)]
pub struct KeyBinding {
    pub sequence: KeySequence,
    pub command: EditorCommand,
}

#[derive(Debug, Default)]
pub struct KeySequence(pub Vec<KeyInput>);

impl KeySequence {
    fn new(key: KeyInput) -> Self {
        Self(vec![key])
    }

    pub fn push(&mut self, key: KeyInput) {
        self.0.push(key);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl std::fmt::Display for KeySequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, key) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?
            }
            key.display(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyBindings(pub Vec<KeyBinding>);

impl KeyBindings {
    pub fn fg_chars(&self) -> impl '_ + Iterator<Item = char> {
        self.0.iter().filter_map(|b| {
            if let EditorCommand::Dot(c) = b.command {
                Some(c)
            } else {
                None
            }
        })
    }

    pub fn find(&self, keys: &KeySequence) -> Result<Option<EditorCommand>, ()> {
        let mut error = true;
        for binding in &self.0 {
            if binding.sequence.0 == keys.0 {
                return Ok(Some(binding.command));
            }
            if error && binding.sequence.0.starts_with(&keys.0) {
                error = false;
            }
        }
        if error { Err(()) } else { Ok(None) }
    }

    pub fn possibe_commands(
        &self,
        prefix: &KeySequence,
    ) -> impl '_ + Iterator<Item = (KeyInput, Option<EditorCommand>)> {
        self.0
            .iter()
            .filter_map(move |binding| {
                // Check if this binding starts with the given prefix
                if binding.sequence.0.len() > prefix.0.len()
                    && binding.sequence.0.starts_with(&prefix.0)
                {
                    let next_key = binding.sequence.0[prefix.0.len()];

                    // Check if this is a complete binding (prefix + 1 key = full sequence)
                    let command = if binding.sequence.0.len() == prefix.0.len() + 1 {
                        Some(binding.command)
                    } else {
                        None // Has children (more keys needed)
                    };

                    Some((next_key, command))
                } else {
                    None
                }
            })
            // Remove duplicates by collecting unique (KeyInput, Option<EditorCommand>) pairs
            .fold(std::collections::BTreeMap::new(), |mut acc, (key, cmd)| {
                // If we already have this key, prefer the complete command over None
                match (acc.get(&key), &cmd) {
                    (Some(Some(_)), _) => {} // Keep existing complete command
                    (Some(None), Some(_)) => {
                        acc.insert(key, cmd);
                    } // Replace None with complete command
                    (None, _) => {
                        acc.insert(key, cmd);
                    } // Insert new entry
                    _ => {}                  // Keep existing entry
                }
                acc
            })
            .into_iter()
    }

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
