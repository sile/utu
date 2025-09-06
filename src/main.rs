use std::path::PathBuf;

use nopng::PngRgbaImage;
use orfail::OrFail;
use utu::{
    app::App,
    buffer::TextPosition,
    config::{Config, FrameSize},
};

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    let mut config: Config = noargs::opt("config-file")
        .short('c')
        .ty("PATH")
        .doc("Configuration file path")
        .env("UTU_CONFIG_FILE")
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
    let frame_size: Option<FrameSize> = noargs::opt("frame-size")
        .short('s')
        .ty("WIDTHxHEIGHT")
        .take(&mut args)
        .present_and_then(|a| a.value().parse())?;
    let export: Option<PathBuf> = noargs::opt("export")
        .ty("PATH")
        .take(&mut args)
        .present_and_then(|a| a.value().parse())?;
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

    if let Some(size) = frame_size {
        config.preview = size;
    }

    if let Some(output_path) = export {
        let mut editor = utu::editor::Editor::new(file_path, config).or_fail()?;
        editor.cursor = position;
        editor.reload().or_fail()?;

        let png = generate_png_from_buffer(&editor.buffer, &editor.config, position).or_fail()?;
        let mut file = std::fs::File::create(&output_path).or_fail()?;
        png.write_to(&mut file).or_fail()?;

        println!("PNG exported to: {}", output_path.display());
    } else {
        let mut app = App::new(file_path, config).or_fail()?;
        app.editor.cursor = position;
        app.run().or_fail()?;
    }

    Ok(())
}

fn generate_png_from_buffer(
    buffer: &utu::buffer::TextBuffer,
    config: &utu::config::Config,
    offset: TextPosition,
) -> orfail::Result<PngRgbaImage> {
    let height = config.preview.height;
    let width = config.preview.width;

    let mut pixels = Vec::with_capacity(width * height * 4);

    for row in 0..height {
        // TODO: consider unicode width
        for col in 0..width {
            let pos = utu::buffer::TextPosition {
                row: row + offset.row,
                col: col + offset.col,
            };
            let pixel_color = if let Some(ch) = buffer.get_char_at(pos) {
                // Convert char to color using the palette
                let color = config
                    .palette
                    .colors
                    .get(&ch)
                    .expect("Character should be validated in palette");
                [color.r, color.g, color.b, 255] // RGB + full alpha
            } else {
                [0, 0, 0, 255] // Black background
            };
            pixels.extend_from_slice(&pixel_color);
        }
    }

    let png = PngRgbaImage::new(width as u32, height as u32, pixels).or_fail()?;
    Ok(png)
}
