use std::io::Cursor;

#[macro_use]
extern crate log;

use anyhow::Context;
use clap::Parser;
use image::io::Reader as ImageReader;
use simplelog::*;

#[derive(Debug, Clone, Parser)]
#[clap(about, version, author)]
struct Args {
    /// The path to the source icon
    #[clap(default_value = "app-icon.png")]
    pub icon_path: String,

    /// Target folder
    #[clap(short, long, default_value = "src-tauri/icons")]
    pub target: String,

    /// Enable/Disable logging
    #[clap(short, long)]
    pub log: bool,
}

const LEVELFILTER: LevelFilter = {
    if cfg!(profile = "release") {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    }
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let logging = args.log;

    CombinedLogger::init(vec![TermLogger::new(
        if logging {
            LEVELFILTER
        } else {
            LevelFilter::Off
        },
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    info!("Opening icon file at {}", args.icon_path);

    let reader = ImageReader::open(args.icon_path)?.with_guessed_format()?;

    let format = reader.format().context("Invalid image format")?;

    if format != image::ImageFormat::Png {
        error!("Only PNG images are supported");
        return Ok(());
    }

    let (length, width) = reader.into_dimensions()?;

    if length != width {
        error!("Only square images are supported");
        return Ok(());
    }

    if length < 1240 || width < 1240 {
        error!("Image size must be at least 1240x1240");
        return Ok(());
    }

    info!("Image dimensions: {}x{}", width, length);

    Ok(())
}
