use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Args {
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
