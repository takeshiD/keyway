use input::{Libinput, LibinputInterface, Event};
// use input::event::keyboard::{KeyboardEventTrait, KeyState};
use libc::{O_RDONLY, O_RDWR};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::io;
use std::path::Path;
use xkbcommon::xkb;

enum HidError {
    Disconnected,
    DeviceNotFound,
}

pub trait Listener {
    fn dispatch(&mut self) -> io::Result<()>;
    fn suspend(&self);
    fn resume(&mut self) -> Result<(), ()>;
}

const KEYCODE_OFFSET: u32 = 8;
pub enum KeyEvent {
    Keyboard,
    Mouse
}
pub struct Key{
    pub value: String,
    pub raw: u32,
    pub is_mod: bool
}
pub struct Input {
}
pub struct Context {
}
pub struct KeyMap {
}

pub enum KeyState {
    Pressed,
    Released,
}

pub struct KeyboardListener {
    input: Libinput,
    context: xkb::Context,
    keymap: xkb::Keymap,
    state: xkb::State,
}
impl KeyboardListener {
    pub fn new() -> Self {
        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").expect("Failed try to assign wayland's seat0");
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb::Keymap::new_from_names(&context, "","","","", None, xkb::COMPILE_NO_FLAGS).expect("Failed try to get system default keymap");
        let state = xkb::State::new(&keymap);
        KeyboardListener {
            input,
            context,
            keymap,
            state
        }
    }
}
impl Listener for KeyboardListener {
    fn dispatch(&mut self) -> io::Result<()> {
        self.input.dispatch()
    }
    fn suspend(&self) {
        self.input.suspend();
    }
    fn resume(&mut self) -> Result<(), ()> {
        self.input.resume()
    }
}

impl Iterator for KeyboardListener {
    type Item = KeyEvent;
    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            Event::Keyboard => {

            }
        }
    }
}

struct Interface;
impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}
