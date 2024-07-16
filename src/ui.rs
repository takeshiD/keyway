use crate::hid;
use glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{Window, Button, Label, Box, Align};
use std::thread;
use std::time::Duration;
use async_channel::Receiver;

pub fn build_ui(app: &Application, rx: Receiver<hid::Key>) {
    let label = Label::new(Some(""));
    let mainwin_box = Box::builder()
        .margin_end(12)
        .margin_top(12)
        .margin_start(12)
        .margin_bottom(12)
        .halign(Align::Center)
        .build();
    let keywin_box = Box::builder()
        .margin_top(12)
        .margin_end(12)
        .margin_start(12)
        .margin_bottom(12)
        .halign(Align::Center)
        .build();
    keywin_box.append(&label);
    glib::spawn_future_local(clone!(
        #[weak]
        label,
        async move {
            while let Ok(key) = rx.recv().await {
                // println!("Async receive key: {} {}", key.value, key.raw);
                label.set_text(format!("{}", key.value).as_str());
            } 
        }
    ));
    let main_win = ApplicationWindow::builder()
        .application(app)
        .title("keyway")
        .child(&mainwin_box)
        .build();
    let keywin = Window::builder()
        .child(&keywin_box)
        .title("display key")
        .transient_for(&main_win)
        .destroy_with_parent(true)
        .build();
    let wakeup_button = Button::builder()
        .label("Start")
        .build();
    wakeup_button.connect_clicked(move |_button| {
        let keywin = keywin.clone();
        if !keywin.activate() {
            keywin.present();
        }
    });
    mainwin_box.append(&wakeup_button);
    main_win.present();
    // keywin.present();
}
