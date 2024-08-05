use iced::futures::SinkExt;
use iced::subscription::{self, Subscription};
use tokio::net::UdpSocket;

use crate::keyway::Keystroke;

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
                        if cfg!(debug_assertions) {
                            println!("[INFO] Starting Receiver")
                        }
                    }
                    ReceiverState::Running(receiver) => match receiver.recv(&mut buf).await {
                        Ok(0) => {
                            if cfg!(debug_assertions) {
                                println!("[INFO] Received(0bytes)");
                            }
                        },
                        Ok(n) => {
                            let b = std::str::from_utf8(&buf[..n]).unwrap();
                            let keystrokes: Vec<Keystroke> = serde_json::from_str(b).unwrap();
                            // if cfg!(debug_assertions) {
                            //     println!("[INFO] Received({}bytes) {:?}", n, &keystrokes);
                            // }
                            output
                                .send(ReceiverEvent::Received(keystrokes))
                                .await
                                .unwrap();
                        }
                        Err(e) => eprintln!("[ERROR] {e}"),
                    },
                }
            }
        },
    )
}
