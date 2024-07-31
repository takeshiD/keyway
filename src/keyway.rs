use iced::executor;
use iced::event::{self, Event};
use iced::multi_window::Application;
use iced::widget::{
    row, column, container, slider, text, toggler,
    mouse_area,
};
use iced::window::{
    self, Level,
};
use iced::mouse::{
    self 
};
use iced::{Color, Command, Element, Length, Theme, Subscription};
use serde::{Deserialize, Serialize};

use std::fmt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
    Drag,
}

pub struct Keyway {
    keys: Vec<Keystroke>,
    windows: HashMap<window::Id, KeywayWindows>,
    config_window: ConfigWindow,
    key_window: KeyWindow,
    timeout: Arc<Mutex<u16>>,
    keywin_visible: bool,
    is_shutdown: Arc<Mutex<bool>>,
}
enum KeywayWindows {
    Configure,
    Keydisplay,
}

impl Application for Keyway {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let timeout = Arc::new(Mutex::new(500));
        let is_shutdown = Arc::new(Mutex::new(false));
        let (keywin_id, keywin_spawn) = window::spawn::<Message>(window::Settings{
            size: iced::Size::new(300.0, 100.0),
            visible: true,
            resizable: true,
            decorations: false,
            transparent: true,
            level: Level::AlwaysOnTop,
            ..Default::default()
        });
        let windows = HashMap::<window::Id, KeywayWindows>::from([
            (window::Id::MAIN, KeywayWindows::Configure),
            (keywin_id, KeywayWindows::Keydisplay)
        ]);
        let sent = Command::perform(
            run_sender(timeout.clone(),
            is_shutdown.clone()), 
            |_| Message::Sender
            );
        let cmd = Command::batch(vec![keywin_spawn, sent]);
        (
            Self {
                keys: vec![],
                windows,
                config_window: ConfigWindow::new(window::Id::MAIN, "Keyway Cofigure"),
                key_window: KeyWindow::new(keywin_id, "Keystroke"),
                timeout,
                keywin_visible: false,
                is_shutdown,
            },
            cmd
        )
    }

    fn title(&self, winid: window::Id) -> String {
        let winname = self.windows.get(&winid).expect("Failed get windows");
        match winname {
            KeywayWindows::Configure => {
                self.config_window.title.clone()
            }
            KeywayWindows::Keydisplay => {
                self.key_window.title.clone()
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::KeyReceived(event) => match event {
                ReceiverEvent::StartReceiver => {},
                ReceiverEvent::Received(keystrokes) => {
                    self.keys = keystrokes;
                }
            },
            Message::Sender => {
                println!("[INFO] Terminated");
            }
            Message::TimeoutChanged(slider_value) => {
                let mut to = self.timeout.lock().unwrap();
                *to = slider_value;
            }
            Message::KeyWinVisibleChanged(visible) => {
                self.keywin_visible = visible;
            }
            Message::ClosedWindow => {
                match self.is_shutdown.lock().as_deref_mut() {
                    Ok(shut) => *shut = true,
                    Err(e) => println!("Failed lock is_shutdown {e}"),
                }
                return window::close(self.key_window.id);
            }
            Message::Drag => {
                return window::drag(self.key_window.id)
            }
        }
        Command::none()
    }

    fn view(&self, winid: window::Id) -> Element<Self::Message> {
        let winname = self.windows.get(&winid).expect("Failed get windows");
        let content: Element<_> = match winname {
            KeywayWindows::Configure => {
                let timeout = *self.timeout.lock().unwrap();
                self.config_window.view(timeout, self.keywin_visible)
            },
            KeywayWindows::Keydisplay => {
                self.key_window.view(&self.keys)
            }
        };
        content
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let recv = run_receiver().map(Message::KeyReceived);
        let winev = event::listen_with(|e, _| {
                match e {
                    Event::Window(id, window_event) => {
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
                    }
                    Event::Mouse(mouse_event) => {
                        match mouse_event {
                            _ => None,
                        }
                    }
                    _ => None
                }
            }
        );
        Subscription::batch(vec![winev, recv])
    }
}

struct ConfigWindow {
    title: String,
    id: window::Id,
}

impl ConfigWindow {
    fn new(id: window::Id, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            id,
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

struct KeyWindow {
    title: String,
    id: window::Id,
}

impl KeyWindow {
    fn new(id: window::Id, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            id,
        }
    }
    fn view(&self, keys: &Vec<Keystroke>) -> Element<Message> {
        let text_keystrokes: Element<_> = if keys.is_empty() {
            container(text("No Pressed"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        } else {
            column(
                keys
                .iter()
                .cloned()
                .map(|k| text(format!("{k}")))
                .map(Element::from),
            )
            .height(Length::Fill)
            .into()
        };
        mouse_area(text_keystrokes)
            .on_press(Message::Drag)
            .into()
    }
}
