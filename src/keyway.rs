use iced::executor;
use iced::multi_window::{self, Application};
use iced::widget::{column, container, slider, text};
use iced::window;
use iced::{Color, Command, Element, Length, Subscription, Theme};
use serde::{Deserialize, Serialize};

use std::fmt;

use crate::keyreceiver::{run_receiver, ReceiverEvent};
use crate::keysender::{run_sender, SenderEvent};

struct Window {
    title: String,
}
impl Window {
    fn new(title: String) -> Self {
        Self { title }
    }
}
pub struct Keyway {
    keys: Vec<Keystroke>,
    config_window: (window::Id, Window),
    key_window: (window::Id, Window),
    timeout: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystroke {
    keycode: u32,
    symbol: String,
}
impl Keystroke {
    pub fn new(keycode: u32, symbol: String) -> Self {
        Keystroke { keycode, symbol }
    }
}

impl fmt::Display for Keystroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key[{:#x} '{}']", self.keycode, self.symbol)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    StartSender,
    KeyReceived(ReceiverEvent),
    KeySent(SenderEvent),
    SliderChanged(u16),
}

impl Application for Keyway {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (winid, spawn_win) = window::spawn::<Message>(window::Settings{
            ..Default::default()
        });
        let serve = Command::perform(run_sender(), |_| Message::StartSender);
        let cmd = Command::batch(vec![spawn_win, serve]);
        (
            Self {
                keys: vec![],
                config_window: (window::Id::MAIN, Window::new(String::from("Configure"))),
                key_window: (winid, Window::new(String::from("Keystroke"))),
                timeout: 500,
            },
            cmd
            // Command::perform(run_sender(), |_| Message::StartSender),
        )
    }

    fn title(&self, winid: window::Id) -> String {
        match winid {
            window::Id::MAIN => self.config_window.1.title.clone(),
            _ => self.key_window.1.title.clone(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::KeyReceived(event) => match event {
                ReceiverEvent::StartReceiver => {
                    println!("[INFO] Starting Receiver");
                }
                ReceiverEvent::Received(keystrokes) => {
                    self.keys = keystrokes;
                }
            },
            Message::KeySent(event) => match event {
                SenderEvent::StartSender => (),
                SenderEvent::Sent(_key) => {}
            },
            Message::StartSender => {
                println!("[INFO] Starting Sender");
            }
            Message::SliderChanged(slider_value) => {
                self.timeout = slider_value;
            }
        }
        Command::none()
    }

    fn view(&self, winid: window::Id) -> Element<Self::Message> {
        let content: Element<_> = match winid {
            window::Id::MAIN => {
                column![
                    text("Configure!"),
                    slider(100..=1000, self.timeout, Message::SliderChanged)
                ].into()
            },
            _ => {
                let text_keystrokes: Element<_> = if self.keys.is_empty() {
                    container(text("No Pressed"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                        .into()
                } else {
                    column(
                        self.keys
                            .iter()
                            .cloned()
                            .map(|k| text(format!("{k}")))
                            .map(Element::from),
                    )
                    .height(Length::Fill)
                    .into()
                };
                text_keystrokes
            }
        };
        content
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let recv = run_receiver().map(Message::KeyReceived);
        // let sent = run_sender_ch().map(Message::KeySent);
        // Subscription::batch(vec![recv, sent])
        recv
    }
}
