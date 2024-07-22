mod keyserver;
mod ui;

use clap::Parser;
use std::path::PathBuf;
use iced::{Application, Settings};

#[derive(Parser)]
struct ArgumentParser {
    #[arg(short, long)]
    config: Option<PathBuf>
}

fn main() {
    ui::Keyway::run(Settings::default());
}
