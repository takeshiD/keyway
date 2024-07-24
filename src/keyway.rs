use iced::{executor, Application};
use iced::widget::{
    column, text, container
};
use iced::{
    Color, Element, Command, Length, Subscription, Theme
};
use serde::{Serialize, Deserialize};

use crate::keyreceiver::{ReceiverEvent, run_receiver};
use crate::keysender::run_sender;

pub struct Keyway {
    keys: Vec<Keystroke>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    StartSender,
    KeyReceived(ReceiverEvent),
}

impl Application for Keyway {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                keys: vec![]
            },
            Command::perform(run_sender(), |_| Message::StartSender),
        )
    }

    fn title(&self) -> String {
        String::from("Keyway")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::KeyReceived(event) => {
                match event {
                    ReceiverEvent::StartReceiver => (),
                    ReceiverEvent::Received(keystrokes) => {
                        self.keys = keystrokes;
                    }
                }
            }
            Message::StartSender => (),
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let text_keystrokes: Element<_>= if self.keys.is_empty() {
            container(
                text("No Pressed")
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        } else {
            column(
                self.keys.iter().cloned().map(|k| text(format!("{:?}", k))).map(Element::from),
            )
            .height(Length::Fill)
            .into()
        };
        column![text_keystrokes].into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        run_receiver().map(Message::KeyReceived)
    }
}

