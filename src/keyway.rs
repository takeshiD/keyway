use iced::{executor, Application};
use iced::widget::{column, text};
use serde::{Serialize, Deserialize};
use crate::keyreceiver::ReceiverEvent;

pub struct Keyway {
    keys: Vec<Keystroke>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keystroke {
    scancode: u16,
}
impl Keystroke {
    pub fn new(scancode: u16) ->  Self {
        Keystroke {
            scancode
        }
    }
}

#[derive(Debug)]
pub enum Message {
    KeyReceived(ReceiverEvent),
}

impl Application for Keyway {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                keys: vec![]
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Keyway")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::KeyReceived(event) => {
                match event {
                    ReceiverEvent::Received(keystroke) => {
                        self.keys.push(keystroke);
                    }
                }
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            text(format!("{:?}", self.keys.get(0).unwrap())),
        ].into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        unimplemented!()
    }
}
