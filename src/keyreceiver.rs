use crate::keyway::Keystroke;

#[derive(Debug)]
pub enum ReceiverEvent {
    Received(Keystroke),
}
