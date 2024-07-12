use input::{Libinput, LibinputInterface, Event};
use input::event::keyboard::{KeyboardEventTrait, KeyState};
use libc::{O_RDONLY, O_RDWR};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::{Path, PathBuf};
use xkbcommon::xkb;
use clap::Parser;

const KEYCODE_OFFSET: u32 = 8;

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

fn mainloop() -> Result<(), String> {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",                                          // rules
        "pc105",                                     // model
        "jp",                                        // layout
        "",                                    // variant
        Some("terminate:ctrl_alt_bksp".to_string()), // options
        xkb::COMPILE_NO_FLAGS,
    ).unwrap();
    let mut state = xkb::State::new(&keymap);
    loop {
        input.dispatch().unwrap(); // event read, 
        for event in &mut input {
            match event {
                Event::Keyboard(keyboard) => {
                    let keycode = xkb::Keycode::new(keyboard.key() + KEYCODE_OFFSET);
                    let keystate = keyboard.key_state();
                    // if !keymap.key_repeats(keycode) {
                    //     continue;
                    // }
                    let keysym = state.key_get_one_sym(keycode);
                    match keystate {
                        KeyState::Pressed  => state.update_key(keycode, xkb::KeyDirection::Down),
                        KeyState::Released => state.update_key(keycode, xkb::KeyDirection::Up),
                    };
                    if state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE) {
                        print!("Control ");
                    }
                    if state.led_name_is_active(xkb::LED_NAME_NUM) {
                        print!("NumLockLED");
                    }
                },
                _ => (),
            }
        }
    }
}

#[derive(Parser)]
struct ArgumentParser {
    #[arg(short, long)]
    config: Option<PathBuf>
}

fn main() -> Result<(), String> {
    // let args = ArgumentParser::parse();
    mainloop()
}
