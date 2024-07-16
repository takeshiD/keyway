mod ui;
mod hid;

use hid::{KeyboardListener, Listener, Event};
use clap::Parser;
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{glib, gio, Application};

use async_channel::bounded;

#[derive(Parser)]
struct ArgumentParser {
    #[arg(short, long)]
    config: Option<PathBuf>
}

const APP_ID: &str = "org.gtk_rs.keyway";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let (tx, rx) = bounded::<hid::Key>(1);
        ui::build_ui(app, rx);
        gio::spawn_blocking(move || {
            let mut listener = hid::KeyboardListener::new();
            loop {
                listener.dispatch().unwrap();
                for event in mut listener {
                    match event {
                        Event::Keyboard => {

                        }
                        Event::Mouse => {
                            unimplemented!()
                        }
                    }
                }
                match tx.send_blocking(key) {
                    Ok(()) => (),
                    Err(err) => println!("Error: {}", err),
                }
            }
        });
    });
    app.run()
}
