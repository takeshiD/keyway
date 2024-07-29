use iced::executor;
use iced::event;
use iced::multi_window::Application;
use iced::widget::{
    row, column, container, slider, text, horizontal_space, vertical_space, toggler,
};
use iced::window;
use iced::{Color, Command, Element, Length, Theme, Subscription};
use serde::{Deserialize, Serialize};

use std::fmt;
use std::sync::{Arc, Mutex};

use crate::keyreceiver::{run_receiver, ReceiverEvent};
use crate::keysender::run_sender;

struct Window {
    title: String,
}
impl Window {
    fn new(title: String) -> Self {
        Self { title }
    }
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
    Sender,
    KeyReceived(ReceiverEvent),
    TimeoutChanged(u16),
    KeyWinVisibleChanged(bool),
    ClosedWindow,
}

pub struct Keyway {
    keys: Vec<Keystroke>,
    config_window: (window::Id, ConfigWindow),
    key_window: (window::Id, Window),
    timeout: Arc<Mutex<u16>>,
    keywin_visible: bool,
    is_shutdown: Arc<Mutex<bool>>,
}

impl Application for Keyway {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let timeout = Arc::new(Mutex::new(500));
        let is_shutdown = Arc::new(Mutex::new(false));
        let (winid, spawn_win) = window::spawn::<Message>(window::Settings{
            size: iced::Size::new(300.0, 100.0),
            visible: true,
            resizable: false,
            decorations: false,
            ..Default::default()
        });
        let sent = Command::perform(run_sender(timeout.clone(), is_shutdown.clone()), |_| Message::Sender);
        let cmd = Command::batch(vec![spawn_win, sent]);
        (
            Self {
                keys: vec![],
                config_window: (window::Id::MAIN, ConfigWindow::new("Keyway Cofigure")),
                key_window: (winid, Window::new(String::from("Keystroke"))),
                timeout,
                keywin_visible: false,
                is_shutdown,
            },
            cmd
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
            Message::Sender => {
                println!("[INFO] Sender Terminated");
            }
            Message::TimeoutChanged(slider_value) => {
                let mut to = self.timeout.lock().unwrap();
                *to = slider_value;
            }
            Message::KeyWinVisibleChanged(visible) => {
                self.keywin_visible = visible;
            }
            Message::ClosedWindow => {
                println!("ClosedWindow in Update");
                match self.is_shutdown.lock().as_deref_mut() {
                    Ok(shut) => *shut = true,
                    Err(e) => println!("Failed lock is_shutdown {e}"),
                }
                println!("Shudown");
                return window::close(self.key_window.0)
            }
        }
        Command::none()
    }

    fn view(&self, winid: window::Id) -> Element<Self::Message> {
        let content: Element<_> = match winid {
            window::Id::MAIN => {
                let timeout = *self.timeout.lock().unwrap();
                self.config_window.1.view(timeout, self.keywin_visible)
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
        let winev = event::listen_with(|event, _| {
            if let iced::Event::Window(id, window_event) = event {
                match window_event {
                    window::Event::Closed => {
                        println!("Closed {:?}", id);
                        match id {
                            window::Id::MAIN => {
                                Some(Message::ClosedWindow)
                            }
                            _ => {
                                None
                            }
                        }
                    }
                    window::Event::CloseRequested => {
                        println!("CloseRequested {:?}", id);
                        match id {
                            window::Id::MAIN => {
                                Some(Message::ClosedWindow)
                            }
                            _ => {
                                None
                            }
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        });
        Subscription::batch(vec![winev, recv])
    }
}

struct ConfigWindow {
    title: String,
}

impl ConfigWindow {
    fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into()
        }
    }
    fn view(&self, timeout: u16, is_visible: bool) -> Element<Message> {
        let slider_timeout = row![
            text("Timeout"),
            slider(100..=2000, timeout, Message::TimeoutChanged).step(50u16),
            text(format!("{timeout}ms")),
        ];
        let keywin_visible = row![
            text("Keystroke Visible"),
            toggler("".to_owned(), is_visible, Message::KeyWinVisibleChanged),
        ];
        let content = column![
            slider_timeout,
            keywin_visible,
        ].spacing(20);
        content.into()
    }
}
