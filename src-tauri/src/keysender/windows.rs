use crate::keyway::Keystroke;

use log::{debug, warn};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug)]
enum KeyAction {
    KEYUP,
    KEYDOWN,
    OTHER,
}

#[derive(Debug)]
struct Key {
    scancode: u32,
    virtkey: u32,
    keyaction: KeyAction,
}

static RX: OnceLock<Mutex<Receiver<Key>>> = OnceLock::new();
static TX: OnceLock<Mutex<Sender<Key>>> = OnceLock::new();

fn init_channel() {
    let (tx, rx) = mpsc::channel();
    RX.set(Mutex::new(rx)).unwrap();
    TX.set(Mutex::new(tx)).unwrap();
}

unsafe fn extract_rawkey(lparam: &LPARAM, keyaction: KeyAction) -> Key {
    let kbdhook = lparam.0 as *const KBDLLHOOKSTRUCT;
    let vkcode = (*kbdhook).vkCode;
    let scancode = (*kbdhook).scanCode;
    let _flags = (*kbdhook).flags;
    let _time = (*kbdhook).time;
    let _exinfo = (*kbdhook).dwExtraInfo;
    let key = Key {
        scancode,
        virtkey: vkcode,
        keyaction,
    };
    key
}

extern "system" fn keyboard_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if ncode as u32 == HC_ACTION {
            match wparam.0 as u32 {
                WM_KEYDOWN | WM_SYSKEYDOWN => {
                    let tx = TX
                        .get()
                        .expect("Failed to get")
                        .lock()
                        .expect("Failed to lock")
                        .clone();
                    let key = extract_rawkey(&lparam, KeyAction::KEYDOWN);
                    tx.send(key).expect("Failed send");
                }
                WM_KEYUP | WM_SYSKEYUP => {
                    let tx = TX
                        .get()
                        .expect("Failed to get")
                        .lock()
                        .expect("Failed to lock")
                        .clone();
                    let key = extract_rawkey(&lparam, KeyAction::KEYUP);
                    tx.send(key).expect("Failed send");
                }
                _ => {
                    let tx = TX
                        .get()
                        .expect("Failed to get")
                        .lock()
                        .expect("Failed to lock")
                        .clone();
                    let key = extract_rawkey(&lparam, KeyAction::OTHER);
                    warn!("Other {:?}", key);
                    tx.send(key).expect("Failed send");
                }
            }
        }
        CallNextHookEx(HHOOK::default(), ncode, wparam, lparam)
    }
}

fn keyboad_hook() {
    unsafe {
        let k_hook =
            SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_proc), HINSTANCE::default(), 0)
                .expect("Failed systemhok");
        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
            TranslateMessage(&message).expect("Failed TranslateMessageA");
            DispatchMessageA(&message);
        }
        if !k_hook.is_invalid() {
            UnhookWindowsHookEx(k_hook).expect("Failed unhook");
        } else {
            panic!("Hook paniced");
        }
    }
}

fn initialize_keymap() -> [Option<&'static str>; 256] {
    let mut keymap = [None; 256];
    keymap[VK_LBUTTON.0 as usize] = Some("LBUTTON");
    keymap[VK_RBUTTON.0 as usize] = Some("RBUTTON");
    keymap[VK_BACK.0 as usize] = Some("Backspace");
    keymap[VK_TAB.0 as usize] = Some("Tab");
    keymap[VK_RETURN.0 as usize] = Some("Enter");
    keymap[VK_SHIFT.0 as usize] = Some("Shift");
    keymap[VK_LSHIFT.0 as usize] = Some("Shift");
    keymap[VK_RSHIFT.0 as usize] = Some("Shift");
    keymap[VK_CONTROL.0 as usize] = Some("Ctrl");
    keymap[VK_LCONTROL.0 as usize] = Some("Ctrl");
    keymap[VK_RCONTROL.0 as usize] = Some("Ctrl");
    keymap[VK_MENU.0 as usize] = Some("Alt");
    keymap[VK_LMENU.0 as usize] = Some("Alt");
    keymap[VK_RMENU.0 as usize] = Some("Alt");
    keymap[VK_ESCAPE.0 as usize] = Some("Esc");
    keymap[VK_SPACE.0 as usize] = Some("Space");
    keymap[VK_PRIOR.0 as usize] = Some("PageUp");
    keymap[VK_NEXT.0 as usize] = Some("PageDown");
    keymap[VK_END.0 as usize] = Some("End");
    keymap[VK_HOME.0 as usize] = Some("Home");
    keymap[VK_LEFT.0 as usize] = Some("Left");
    keymap[VK_RIGHT.0 as usize] = Some("Right");
    keymap[VK_UP.0 as usize] = Some("Up");
    keymap[VK_DOWN.0 as usize] = Some("Down");
    keymap[VK_INSERT.0 as usize] = Some("Insert");
    keymap[VK_DELETE.0 as usize] = Some("Delete");

    // number and letters
    keymap[VK_0.0 as usize] = Some("0");
    keymap[VK_1.0 as usize] = Some("1");
    keymap[VK_2.0 as usize] = Some("2");
    keymap[VK_3.0 as usize] = Some("3");
    keymap[VK_4.0 as usize] = Some("4");
    keymap[VK_5.0 as usize] = Some("5");
    keymap[VK_6.0 as usize] = Some("6");
    keymap[VK_7.0 as usize] = Some("7");
    keymap[VK_8.0 as usize] = Some("8");
    keymap[VK_9.0 as usize] = Some("9");
    keymap[VK_A.0 as usize] = Some("A");
    keymap[VK_B.0 as usize] = Some("B");
    keymap[VK_C.0 as usize] = Some("C");
    keymap[VK_D.0 as usize] = Some("D");
    keymap[VK_E.0 as usize] = Some("E");
    keymap[VK_F.0 as usize] = Some("F");
    keymap[VK_G.0 as usize] = Some("G");
    keymap[VK_H.0 as usize] = Some("H");
    keymap[VK_I.0 as usize] = Some("I");
    keymap[VK_J.0 as usize] = Some("J");
    keymap[VK_K.0 as usize] = Some("K");
    keymap[VK_L.0 as usize] = Some("L");
    keymap[VK_M.0 as usize] = Some("M");
    keymap[VK_N.0 as usize] = Some("N");
    keymap[VK_O.0 as usize] = Some("O");
    keymap[VK_P.0 as usize] = Some("P");
    keymap[VK_Q.0 as usize] = Some("Q");
    keymap[VK_R.0 as usize] = Some("R");
    keymap[VK_S.0 as usize] = Some("S");
    keymap[VK_T.0 as usize] = Some("T");
    keymap[VK_U.0 as usize] = Some("U");
    keymap[VK_V.0 as usize] = Some("V");
    keymap[VK_W.0 as usize] = Some("W");
    keymap[VK_X.0 as usize] = Some("X");
    keymap[VK_Y.0 as usize] = Some("Y");
    keymap[VK_Z.0 as usize] = Some("Z");

    // oem-keys
    keymap[VK_OEM_PLUS.0 as usize] = Some("+");
    keymap[VK_OEM_COMMA.0 as usize] = Some(",");
    keymap[VK_OEM_MINUS.0 as usize] = Some("-");
    keymap[VK_OEM_PERIOD.0 as usize] = Some(".");

    // Functions
    keymap[VK_F1.0 as usize] = Some("F1");
    keymap[VK_F2.0 as usize] = Some("F2");
    keymap[VK_F3.0 as usize] = Some("F3");
    keymap[VK_F4.0 as usize] = Some("F4");
    keymap[VK_F5.0 as usize] = Some("F5");
    keymap[VK_F6.0 as usize] = Some("F6");
    keymap[VK_F7.0 as usize] = Some("F7");
    keymap[VK_F8.0 as usize] = Some("F8");
    keymap[VK_F9.0 as usize] = Some("F9");
    keymap[VK_F10.0 as usize] = Some("F10");
    keymap[VK_F11.0 as usize] = Some("F11");
    keymap[VK_F12.0 as usize] = Some("F12");
    keymap[VK_F13.0 as usize] = Some("F13");
    keymap[VK_F14.0 as usize] = Some("F14");
    keymap[VK_F15.0 as usize] = Some("F15");
    keymap[VK_F16.0 as usize] = Some("F16");
    keymap[VK_F17.0 as usize] = Some("F17");
    keymap[VK_F18.0 as usize] = Some("F18");
    keymap[VK_F19.0 as usize] = Some("F19");
    keymap[VK_F20.0 as usize] = Some("F20");
    keymap[VK_F21.0 as usize] = Some("F21");
    keymap[VK_F22.0 as usize] = Some("F22");
    keymap[VK_F23.0 as usize] = Some("F23");
    keymap[VK_F24.0 as usize] = Some("F24");
    keymap
}

struct KeyboardState {
    last_scancode: u32,
    last_virtkey: u32,
    last_state: [u8; 256],
    keymap: [Option<&'static str>; 256],
}

impl KeyboardState {
    fn new() -> Self {
        KeyboardState {
            last_scancode: 0,
            last_virtkey: 0,
            last_state: [0u8; 256],
            keymap: initialize_keymap(),
        }
    }
    fn update(&mut self, virtkey: u16, keyaction: KeyAction) {
        match keyaction {
            KeyAction::KEYDOWN => match VIRTUAL_KEY(virtkey) {
                VK_SHIFT => {
                    self.last_state[VK_SHIFT.0 as usize] |= 0x80;
                    self.last_state[VK_LSHIFT.0 as usize] |= 0x80;
                    self.last_state[VK_RSHIFT.0 as usize] |= 0x80;
                }
                VK_CONTROL => {
                    self.last_state[VK_CONTROL.0 as usize] |= 0x80;
                    self.last_state[VK_LCONTROL.0 as usize] |= 0x80;
                    self.last_state[VK_RCONTROL.0 as usize] |= 0x80;
                }
                VK_MENU => {
                    self.last_state[VK_MENU.0 as usize] |= 0x80;
                    self.last_state[VK_LMENU.0 as usize] |= 0x80;
                    self.last_state[VK_RMENU.0 as usize] |= 0x80;
                }
                VK_CAPITAL => {
                    self.last_state[VK_CAPITAL.0 as usize] ^= 0x01;
                }
                _ => {
                    self.last_state[virtkey as usize] |= 0x80;
                    self.last_virtkey = virtkey as u32;
                }
            },
            KeyAction::KEYUP => match VIRTUAL_KEY(virtkey) {
                VK_SHIFT => {
                    self.last_state[VK_SHIFT.0 as usize] &= !0x80;
                    self.last_state[VK_LSHIFT.0 as usize] &= !0x80;
                }
                VK_CONTROL => {
                    self.last_state[VK_CONTROL.0 as usize] &= !0x80;
                    self.last_state[VK_LCONTROL.0 as usize] &= !0x80;
                    self.last_state[VK_RCONTROL.0 as usize] &= !0x80;
                }
                VK_MENU => {
                    self.last_state[VK_MENU.0 as usize] &= !0x80;
                    self.last_state[VK_LMENU.0 as usize] &= !0x80;
                    self.last_state[VK_RMENU.0 as usize] &= !0x80;
                }
                _ => {
                    self.last_state[virtkey as usize] &= !0x80;
                }
            },
            KeyAction::OTHER => {}
        }
    }
    fn get_syms(&self) -> Vec<&'static str> {
        let mut syms = Vec::new();
        for (i, state ) in self.last_state.iter().enumerate() {
            if *state & 0x80 != 0 {
                match self.keymap[i] {
                    Some(sym) => {
                        syms.push(sym);
                    }
                    None => {}
                }
            }
        }
        syms
    }
}

pub fn run_sender(timeout: Arc<RwLock<u32>>, apphandle: AppHandle, label: String, event: String) {
    init_channel();
    let hook = std::thread::spawn(|| {
        keyboad_hook();
    });
    let recv = std::thread::spawn(move || {
        let mut keystrokes = Vec::<Vec<&str>>::new();
        let mut timestamp = Instant::now();
        let mut keyboard = KeyboardState::new();
        '_keysend_loop: loop {
            let timeout = Duration::from_millis(*timeout.read().unwrap() as u64);
            match RX
                .get()
                .expect("Failed get")
                .lock()
                .expect("Failed read")
                .recv_timeout(Duration::from_millis(50))
            {
                Ok(recv) => match recv.keyaction {
                    KeyAction::KEYDOWN => {
                        timestamp = Instant::now();
                        keyboard.update(recv.virtkey as u16, recv.keyaction);
                        let keysyms = keyboard.get_syms();
                        debug!("{:?}", keysyms);
                        if !keysyms.is_empty() {
                            keystrokes.push(keysyms);
                        }
                    }
                    KeyAction::KEYUP => {
                        keyboard.update(recv.virtkey as u16, recv.keyaction);
                    }
                    KeyAction::OTHER => {}
                },
                Err(_err) => {}
            }
            let now = Instant::now();
            if !keystrokes.is_empty() && (now - timestamp > timeout) {
                keystrokes.clear();
                debug!("Key Cleared");
            }
            if !keystrokes.is_empty() {
                debug!("Keystrokes: {:#?}", keystrokes);
            }
            apphandle
                .emit_to(&label, &event, keystrokes.clone())
                .unwrap();
            // keystrokes.clear();
        }
    });
    recv.join().expect("Failed join recv");
    hook.join().expect("Failed join hook");
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_keyboardstate() {
        let keyboard = KeyboardState::new();
        for i in 0..255 {
            println!("")
        }
    }
}
