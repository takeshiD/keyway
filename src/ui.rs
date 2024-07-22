use iced::{
    event::{self, Status},
    executor,
    widget::text,
    Application, Event, Settings,
};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
enum Message {
    KeyPressed,
    NoPressed,
}

pub struct Keyway {
    pressed_key: String,
}

impl Application for Keyway {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                pressed_key: "".into(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Keyway")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::KeyPressed(s) => self.pressed_key = s,
            MyAppMessage::NoPressed => self.pressed_key = String::from("timeout"),
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        text(self.pressed_key.as_str()).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        println!("Subs: {:?}", SystemTime::now());
        event::listen_with(|event, status| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Enter),
                    ..
                }),
                Status::Ignored,
            ) => Some(MyAppMessage::KeyPressed("Enter".into())),
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(MyAppMessage::KeyPressed("Space".into())),
            _ => Some(MyAppMessage::NoPressed),
        })
    }
}
