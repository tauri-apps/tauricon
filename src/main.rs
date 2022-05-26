#[macro_use]
extern crate log;

#[macro_use]
mod macros;

use anyhow::Context;
use clap::Parser;
use image::{imageops::FilterType, io::Reader as ImageReader, ImageResult};
use rayon::prelude::*;
use simplelog::*;

mod configs;
mod icons;

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

fn read_image(
    icon_path: &str,
) -> anyhow::Result<image::io::Reader<std::io::BufReader<std::fs::File>>> {
    let cwd = std::env::current_dir().context("Could not get current directory")?;

    ImageReader::open(cwd.join(icon_path))
        .context("Failed to read image")?
        .with_guessed_format()
        .map_err(|e| e.into())
}

fn init_logger(log: bool) -> anyhow::Result<()> {
    CombinedLogger::init(vec![TermLogger::new(
        if log { LEVELFILTER } else { LevelFilter::Error },
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger(args.log)?;

    let config = configs::get_config();

    let final_icons = match config {
        Err(config_error) => {
            let res = match config_error {
            configs::ConfigError::Io(e) => format!("{}", e),
            configs::ConfigError::Json(e) => format!("{}", e),
            configs::ConfigError::FileNotFound(e) => format!("Could not find Tauri config file at {}.\nYou must run this command within a Tauri project!", e.display()),
        };

            error!("{}", res);

            exit!();
        }
        Ok(v) => v.tauri.bundle.icon,
    };

    let parsed_icons = final_icons
        .iter()
        .map(|x| icons::parse_icon_path(x))
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    info!("Opening icon file at {}", args.icon_path);

    {
        let reader = read_image(&args.icon_path)?;

        let format = reader
            .format()
            .context("Invalid image format. Must be PNG")?;

        if format != image::ImageFormat::Png {
            error!("Only PNG images are supported");
            exit!();
        }

        let (length, width) = reader.into_dimensions()?;

        if length != width {
            error!("Only square images are supported");
            exit!();
        }

        if length < 1240 || width < 1240 {
            error!("Image size must be at least 1240x1240");
            exit!();
        }

        info!("Image dimensions: {}x{}", width, length);
    }

    let target_path = std::env::current_dir()?.join(args.target);
    std::fs::create_dir_all(&target_path)?;

    // Must be done again as initial reader is moved
    let reader = read_image(&args.icon_path)?;
    let base_image = reader.decode()?;

    parsed_icons
        .par_iter()
        .map(|icon| {
            let target_ext: String = icon.format.into();
            let scale = if icon.scale == 1 {
                String::from("")
            } else {
                format!("@{}x", icon.scale)
            };
            let name = if icon.height == 1240 || icon.format == icons::IconFormat::Ico {
                "icon".into()
            } else {
                format!("{}x{}", icon.width, icon.height)
            };

            let target_file = target_path.join(format!("{}{}.{}", name, scale, target_ext));

            info!("Writing icon to {}", target_file.display());

            let resized = base_image.resize(icon.width, icon.height, FilterType::Nearest);
            resized.save_with_format(target_file, icon.format.into())
        })
        .collect::<ImageResult<Vec<_>>>()?;

    Ok(())
}
