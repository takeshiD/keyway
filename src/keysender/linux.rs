use evdev::Device;
use iced::futures::SinkExt;
use iced::subscription::{self, Subscription};
use mio::net::UdpSocket;
use mio::{unix::SourceFd, Events, Interest, Poll, Token};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::net::SocketAddr;

use xkbcommon::xkb;

use crate::keyway::{Keystroke, Keyway};

fn is_keyboard(dev: &Device) -> bool {
    let has_key = dev.supported_events().contains(evdev::EventType::KEY);
    let has_misc = dev.supported_events().contains(evdev::EventType::MISC);
    let has_rpt = dev.supported_events().contains(evdev::EventType::REPEAT);
    has_key && has_misc && has_rpt
}

fn get_allkeyabords() -> Vec<(PathBuf, Device)> {
    let devices = evdev::enumerate()
        .filter(|x| {
            let dev = &x.1;
            is_keyboard(dev)
        })
        .collect::<Vec<_>>();
    devices
}

const KEY_STATE_RELEASE: i32 = 0;
const KEY_STATE_PREESS: i32 = 1;
const KEY_STATE_REPEAT: i32 = 2;
const KEY_OFFSET: u16 = 8;

struct Keyboard {
    context: xkb::Context,
    keymap: xkb::Keymap,
    state: xkb::State,
    // compose_state: xkb::compose::State,
    path: PathBuf,
}

impl Keyboard {
    fn new(p: &PathBuf) -> Self {
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap =
            xkb::Keymap::new_from_names(&context, "", "", "", "", None, xkb::COMPILE_NO_FLAGS)
                .unwrap();
        let state = xkb::State::new(&keymap);
        let path = p.clone();
        // let compose_state = xkb::compose::State::new();
        Keyboard {
            context,
            keymap,
            state,
            path,
        }
    }
    fn is_repeats(&self, keycode: xkb::Keycode) -> bool {
        self.keymap.key_repeats(keycode)
    }
    fn update(&mut self, keycode: xkb::Keycode, direction: xkb::KeyDirection) {
        self.state.update_key(keycode, direction);
    }
    fn get_string(&self, keycode: xkb::Keycode) -> String {
        self.state.key_get_utf8(keycode)
    }
}

pub async fn run_sender(
    timeout: Arc<Mutex<u16>>,
    is_shutdown: Arc<Mutex<bool>>
) {
    let mut devices = get_allkeyabords();
    let mut tokens = vec![];
    let mut keyboards = Vec::<Keyboard>::new();
    for (p, _d) in devices.iter() {
        keyboards.push(Keyboard::new(p));
    }
    for i in 0..devices.len() {
        tokens.push(Token(i));
    }
    let mut poll = Poll::new().unwrap();
    for (i, (_, d)) in devices.iter().enumerate() {
        poll.registry()
            .register(&mut SourceFd(&d.as_raw_fd()), tokens[i], Interest::READABLE)
            .unwrap();
    }
    let udp_sender = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let target = "127.0.0.1:53300".parse::<SocketAddr>().unwrap();
    let mut events = Events::with_capacity(32);
    let mut buf = Vec::<Keystroke>::new();
    let mut timestamp = Instant::now();
    while !*is_shutdown.lock().unwrap() {
        let timeout = Duration::from_millis(*timeout.lock().unwrap() as u64);
        poll.poll(&mut events, Some(Duration::from_millis(50))).unwrap();
        for event in events.iter() {
            match event.token() {
                Token(i) if (0..devices.len()).contains(&i) => {
                    let (_, ref mut d) = devices.get_mut(i).unwrap();
                    let keyboard = keyboards.get_mut(i).unwrap();
                    for e in d.fetch_events().unwrap() {
                        match e.kind() {
                            evdev::InputEventKind::Key(keycode) => {
                                timestamp = Instant::now();
                                let keycode: xkb::Keycode = (keycode.0 + KEY_OFFSET).into();
                                let keystate = e.value();
                                match keystate {
                                    KEY_STATE_RELEASE => {
                                        keyboard.update(keycode, xkb::KeyDirection::Up);
                                    }
                                    KEY_STATE_PRESS => {
                                        keyboard.update(keycode, xkb::KeyDirection::Down);
                                        let keystroke = Keystroke::new(
                                            keycode.raw(),
                                            // String::from("Alternate"),
                                            keyboard.get_string(keycode),
                                        );
                                        buf.push(keystroke);
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
        if !buf.is_empty() && (Instant::now() - timestamp > timeout) {
            buf.clear();
        }
        match udp_sender.send_to(serde_json::to_string(&buf).unwrap().as_bytes(), target) {
            Ok(n) => {
                println!("Send({}) {:?}", n, &buf);
            }
            Err(e) => {
                println!("SenderError: {e}");
                break;
            }
        }
    }
}
