use std::path::PathBuf;

use orfail::OrFail;
use pixed::editor::Editor;

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    let file_path: PathBuf = noargs::arg("FILE_PATH")
        .doc("File path to edit")
        .example("/path/to/file")
        .take(&mut args)
        .then(|a| {
            let path = PathBuf::from(a.value());
            if !matches!(a, noargs::Arg::Example { .. }) && !path.is_file() {
                Err("not a file")
            } else {
                Ok(path)
            }
        })?;
    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let editor = Editor::new(file_path);
    editor.run().or_fail()?;

    Ok(())
}
