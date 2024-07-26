mod keyway;
mod keysender;
mod keyreceiver;

use keyway::Keyway;

use clap::Parser;
use std::path::PathBuf;
use iced::Settings;
use iced::multi_window::Application;

#[derive(Parser)]
struct ArgumentParser {
    #[arg(short, long)]
    config: Option<PathBuf>
}

fn main() -> iced::Result {
    Keyway::run(Settings::default())
}
