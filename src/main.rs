use std::io::Cursor;

use clap::Parser;
use image::io::Reader as ImageReader;

mod args;

fn main() -> anyhow::Result<()> {
    args::Args::parse();

    Ok(())
}
