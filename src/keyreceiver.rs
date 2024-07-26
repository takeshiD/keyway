use iced::futures::SinkExt;
use iced::subscription::{self, Subscription};
use tokio::net::UdpSocket;

use crate::keyway::{Keystroke, Keyway};

#[derive(Debug, Clone)]
pub enum ReceiverEvent {
    StartReceiver,
    Received(Vec<Keystroke>),
}

#[derive(Debug)]
enum ReceiverState {
    Stop,
    Running(UdpSocket),
}

pub fn run_receiver() -> Subscription<ReceiverEvent> {
    struct UdpReceiver;
    subscription::channel(
        std::any::TypeId::of::<UdpReceiver>(),
        10,
        |mut output| async move {
            let mut state = ReceiverState::Stop;
            let mut buf = [0u8; 4096];
            loop {
                match &mut state {
                    ReceiverState::Stop => {
                        let receiver = UdpSocket::bind("127.0.0.1:53300").await.unwrap();
                        output.send(ReceiverEvent::StartReceiver).await.unwrap();
                        state = ReceiverState::Running(receiver);
                    }
                    ReceiverState::Running(receiver) => match receiver.recv(&mut buf).await {
                        Ok(0) => (),
                        Ok(n) => {
                            let b = std::str::from_utf8(&buf[..n]).unwrap();
                            println!("Received({n}bytes): {}", b);
                            let keystrokes: Vec<Keystroke> = serde_json::from_str(b).unwrap();
                            output
                                .send(ReceiverEvent::Received(keystrokes))
                                .await
                                .unwrap();
                        }
                        Err(e) => println!("Error: {e}"),
                    },
                }
            }
        },
    )
}
