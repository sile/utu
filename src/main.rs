use std::path::PathBuf;

use orfail::OrFail;
use utu::{app::App, buffer::TextPosition};

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    let config = noargs::opt("config-file")
        .short('c')
        .ty("PATH")
        .doc("Configuration file path")
        .env("PIXED_CONFIG_FILE")
        .take(&mut args)
        .present_and_then(|a| -> Result<_, Box<dyn std::error::Error>> {
            let content = std::fs::read_to_string(a.value())?;
            let config = content.parse().map(|nojson::Json(v)| v)?;
            Ok(config)
        })?
        .unwrap_or_default();
    let position: TextPosition = noargs::opt("position")
        .short('p')
        .ty("ROW:COLUMN")
        .default("1:1")
        .take(&mut args)
        .then(|a| a.value().parse())?;
    let file_path: PathBuf = noargs::arg("FILE_PATH")
        .doc("File path to edit")
        .example("/path/to/file")
        .take(&mut args)
        .then(|a| {
            let path = PathBuf::from(a.value());
            if matches!(a, noargs::Arg::Example { .. }) {
                Ok(path)
            } else if !path.exists() {
                Err("no such file")
            } else if !path.is_file() {
                Err("not a file")
            } else {
                Ok(path)
            }
        })?;
    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let mut app = App::new(file_path, config).or_fail()?;
    app.editor.cursor = position;
    app.run().or_fail()?;

    Ok(())
}
