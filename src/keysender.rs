use mio::{Events, Poll, Interest, Token, unix::SourceFd};
use mio::net::UdpSocket;
use std::time::{Duration, Instant};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use evdev::Device;
use serde::{Serialize, Deserialize};
use iced::subscription::{self, Subscription};

use crate::keyway::Keystroke;

fn is_keyboard(dev: &Device) -> bool {
    let has_key = dev.supported_events().contains(evdev::EventType::KEY);
    let has_misc = dev.supported_events().contains(evdev::EventType::MISC);
    let has_rpt = dev.supported_events().contains(evdev::EventType::REPEAT);
    has_key && has_misc && has_rpt
}

fn get_allkeyabords() -> Vec<(PathBuf, Device)> {
    let devices = evdev::enumerate().filter(|x| {
        let dev = &x.1;
        is_keyboard(dev)
    }).collect::<Vec<_>>();
    devices
}


pub async fn run_sender() {
    let mut devices = get_allkeyabords();
    let mut tokens = vec![];
    for i in 0..devices.len() {
        tokens.push(Token(i));
    }
    let mut poll = Poll::new().unwrap();
    for (i, (_, d)) in devices.iter().enumerate() {
        poll.registry().register(&mut SourceFd(&d.as_raw_fd()), tokens[i], Interest::READABLE).unwrap();
    }
    let udp_sender = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let target = "127.0.0.1:53300".parse().unwrap();
    udp_sender.connect(target).unwrap();
    println!("Receive wait: {:?}", target);
    let mut events = Events::with_capacity(32);
    let mut buf = Vec::<Keystroke>::with_capacity(100);
    let timeout = Duration::from_millis(500);
    let mut timestamp = Instant::now();
    loop {
        poll.poll(&mut events, Some(Duration::from_millis(50))).unwrap();
        for event in events.iter() {
            match event.token() {
                Token(i) if (0..devices.len()).contains(&i) => {
                    let (_, ref mut d) = devices.get_mut(i).unwrap();
                    for e in d.fetch_events().unwrap() {
                        match e.kind() {
                            evdev::InputEventKind::Key(keyevent) => {
                                timestamp = Instant::now();
                                let keystroke = Keystroke::new(keyevent.code());
                                buf.push(keystroke);
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
        match udp_sender.send(serde_json::to_string(&buf).unwrap().as_bytes()) {
            Ok(_) => {
                println!("Send {:?}", &buf);
            }
            Err(_) => (),
        }
    }
}
