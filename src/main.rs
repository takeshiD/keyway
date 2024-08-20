mod keyreceiver;
mod keysender;
mod keyway;
mod systemfont;

use keyway::Keyway;

use clap::Parser;
use iced::multi_window::Application;
use iced::window::{self, Level};
use iced::Settings;
use iced::{Length, Size};

use std::path::PathBuf;

#[derive(Parser)]
struct ArgumentParser {
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> iced::Result {
    Keyway::run(Settings {
        window: window::Settings {
            size: iced::Size::new(300.0, 100.0),
            position: iced::window::Position::Specific(iced::Point::new(0.0, 0.0)),
            visible: true,
            resizable: false,
            decorations: false,
            transparent: false,
            level: Level::AlwaysOnTop,
            ..Default::default()
        },
        ..Default::default()
    })
}
