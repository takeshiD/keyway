mod keyway;
mod keysender;
mod keyreceiver;

use keyway::Keyway;

use clap::Parser;
use std::path::PathBuf;
use iced::{Application, Settings};

#[derive(Parser)]
struct ArgumentParser {
    #[arg(short, long)]
    config: Option<PathBuf>
}

fn main() {
    Keyway::run(Settings::default());
}
