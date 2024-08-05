mod keyway;
mod keysender;
mod keyreceiver;

use keyway::Keyway;

use clap::Parser;
use iced::Settings;
use iced::window;
use iced::{Size, Length};
use iced::multi_window::Application;

use std::path::PathBuf;

#[derive(Parser)]
struct ArgumentParser {
    #[arg(short, long)]
    config: Option<PathBuf>
}

fn main() -> iced::Result {
    Keyway::run(Settings {
        window: window::Settings{
            size: Size::new(500.0, 500.0),
            ..Default::default()
        },
        ..Default::default()
    })
}
