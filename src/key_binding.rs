use tuinix::KeyCode;

use crate::editor_command::EditorCommand;

#[derive(Debug)]
pub struct KeyBinding {
    pub path: KeyPath,
    pub command: EditorCommand,
}

#[derive(Debug)]
pub struct KeyPath(pub Vec<KeyCode>);

#[derive(Debug)]
pub struct KeyBindings(pub Vec<KeyBinding>);

impl<'text> nojson::FromRawJsonValue<'text> for KeyBindings {
    fn from_raw_json_value(
        _value: nojson::RawJsonValue<'text, '_>,
    ) -> Result<Self, nojson::JsonParseError> {
        todo!()
        // let mut bindings = Vec::new();

        // fn parse_bindings<'text>(
        //     value: nojson::RawJsonValue<'text, '_>,
        //     prefix: Vec<KeyCode>,
        //     bindings: &mut Vec<KeyBinding>,
        // ) -> Result<(), nojson::JsonParseError> {
        //     for (key_str, cmd_value) in value.to_object()? {
        //         let key_sequence = parse_key_sequence(key_str.to_unquoted_string_str()?)?;
        //         let mut full_path = prefix.clone();
        //         full_path.extend(key_sequence);

        //         if cmd_value.kind().is_object() {
        //             // Nested object - recurse with the current path as prefix
        //             parse_bindings(cmd_value, full_path, bindings)?;
        //         } else {
        //             // Command string - create a binding
        //             let command_str = cmd_value.to_unquoted_string_str()?;
        //             let command = command_str
        //                 .parse()
        //                 .map_err(|e| nojson::JsonParseError::invalid_value(cmd_value, e))?;

        //             bindings.push(KeyBinding {
        //                 path: KeyPath(full_path),
        //                 command,
        //             });
        //         }
        //     }
        //     Ok(())
        // }

        // parse_bindings(value, Vec::new(), &mut bindings)?;
        // Ok(KeyBindings(bindings))
    }
}

// fn parse_key_sequence(key_str: &str) -> Result<Vec<KeyCode>, String> {
//     let mut keys = Vec::new();

//     for key_part in key_str.split(',') {
//         let key = parse_single_key(key_part.trim())?;
//         keys.push(key);
//     }

//     Ok(keys)
// }
