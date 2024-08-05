use iced::executor;
use iced::theme;
use iced::application;
use iced::event::{self, Event};
use iced::multi_window::Application;
use iced::widget::{
    row, column, container, slider, text, toggler,
    mouse_area, button
};
use iced::window::{
    self, Level,
};
use iced::{Color, Command, Element, Length, Theme, Subscription, Alignment};
use serde::{Deserialize, Serialize};

use std::fmt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::keyreceiver::{run_receiver, ReceiverEvent};
use crate::keysender::run_sender;

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
    Drag(window::Id),
    Minimize,
}

pub struct Keyway {
    keys: Vec<Keystroke>,
    windows: HashMap<window::Id, KeywayWindows>,
    config_window: ConfigWindow,
    key_window: KeyWindow,
    timeout: Arc<Mutex<u16>>,
    keywin_visible: bool,
    is_shutdown: Arc<Mutex<bool>>,
    theme: Theme
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
        let (cfgwin_id, cfgwin_spawn) = window::spawn::<Message>(
            window::Settings{
                size: iced::Size::new(500.0, 500.0),
                decorations: false,
                ..Default::default()
            }
        );
        let windows = HashMap::<window::Id, KeywayWindows>::from([
            (window::Id::MAIN, KeywayWindows::Keydisplay),
            (cfgwin_id, KeywayWindows::Configure)
        ]);
        let sent = Command::perform(
            run_sender(timeout.clone(),
            is_shutdown.clone()), 
            |_| Message::Sender
            );
        let cmd = Command::batch(vec![cfgwin_spawn, sent]);
        let mytheme = Theme::Custom(Arc::new(theme::Custom::new(String::from("MyTheme"), theme::Palette {
            background: Color::from_rgba8(0,0,255,0.5),
            text: Color::from_rgb8(0, 255, 0),
            primary: Color::from_rgb8(0x5e, 0x7c, 0xe2),
            success: Color::from_rgb8(0x12, 0x66, 0x4f),
            danger: Color::from_rgb8(0xc3, 0x42, 0x3f),
            })));
        (
            Self {
                keys: vec![],
                windows,
                config_window: ConfigWindow::new(cfgwin_id, "Keyway Cofigure"),
                key_window: KeyWindow::new(window::Id::MAIN, "Keystroke"),
                timeout,
                keywin_visible: false,
                is_shutdown,
                theme: mytheme,
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
                return Command::batch(vec![
                    window::close(self.key_window.id),
                    window::close(self.config_window.id),
                ]);
            }
            Message::Drag(id) => {
                return window::drag(id);
            }
            Message::Minimize => {
                return window::change_mode(self.config_window.id, window::Mode::Hidden);
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
        let winev = event::listen_with(|e, _| match e {
            Event::Window(id, window_event) => {
                match window_event {
                    window::Event::Closed => {
                        match id {
                            window::Id::MAIN => {
                                println!("Closed MAIN");
                                None
                            }
                            _ => {
                                println!("Closed Other");
                                Some(Message::ClosedWindow)
                            }
                        }
                    },
                    _ => {
                        println!("WindowEvent: {:?}", window_event);
                        None
                    }
                }
            },
            _ => {
                None
            }
        });
        Subscription::batch(vec![winev, recv])
    }
    fn theme(&self, id: window::Id) -> Self::Theme {
        if id == self.key_window.id {
            self.theme.clone()
        } else {
            Theme::Light
        }
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
        let header = row![
            button("Min").on_press(Message::Minimize),
            button("Close").on_press(Message::ClosedWindow),
        ].align_items(Alignment::End);
        let slider_timeout = row![
            text("Timeout"),
            slider(100..=2000, timeout, Message::TimeoutChanged).step(50u16),
            text(format!("{timeout}ms")),
        ];
        let keywin_visible = row![
            text("Keystroke Visible"),
            toggler("".to_owned(), is_visible, Message::KeyWinVisibleChanged),
        ];
        let body = column![
            slider_timeout,
            keywin_visible,
        ]
        .spacing(20)
        .padding(20);
        let content = column![
            header,
            body,
        ];
        mouse_area(content)
            .on_press(Message::Drag(self.id))
            .into()
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
        let text_keystrokes: Element<_> = container(
            row(
                keys
                .iter()
                .cloned()
                .map(|k| text(format!("{}", k.symbol)))
                .map(Element::from),
            )
            .height(Length::Fill)
            .width(Length::Fill)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into();
        let content = column![
            text_keystrokes
        ].padding(20);
        mouse_area(content)
            .on_press(Message::Drag(self.id))
            .into()
    }
}
