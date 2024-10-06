use crate::keyway::Keystroke;

use log::{debug, warn};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;
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


struct KeyboardState {
    last_scancode: u32,
    last_virtkey: u32,
    last_state: [u8; 256],
}

impl KeyboardState {
    fn new() -> Self {
        KeyboardState {
            last_scancode: 0,
            last_virtkey: 0,
            last_state: [0u8; 256],
        }
    }
    fn update(&mut self, virtkey: u16, keyaction: KeyAction) {
        match keyaction {
            KeyAction::KEYDOWN => match VIRTUAL_KEY(virtkey) {
                VK_SHIFT => {
                    self.last_state[VK_SHIFT.0 as usize]  |= 0x80;
                    self.last_state[VK_LSHIFT.0 as usize] |= 0x80;
                    self.last_state[VK_RSHIFT.0 as usize] |= 0x80;
                }
                VK_CONTROL => {
                    self.last_state[VK_CONTROL.0 as usize]  |= 0x80;
                    self.last_state[VK_LCONTROL.0 as usize] |= 0x80;
                    self.last_state[VK_RCONTROL.0 as usize] |= 0x80;
                }
                VK_CAPITAL => {
                    self.last_state[VK_CAPITAL.0 as usize] ^= 0x01;
                }
                _ => {
                    self.last_virtkey = virtkey as u32;
                }
            },
            KeyAction::KEYUP => match VIRTUAL_KEY(virtkey) {
                VK_SHIFT => {
                    self.last_state[VK_SHIFT.0 as usize]  &= !0x80;
                    self.last_state[VK_LSHIFT.0 as usize] &= !0x80;
                }
                VK_CONTROL => {
                    self.last_state[VK_CONTROL.0 as usize]  &= !0x80;
                    self.last_state[VK_LCONTROL.0 as usize] &= !0x80;
                    self.last_state[VK_RCONTROL.0 as usize] &= !0x80;
                }
                _ => {}
            },
            KeyAction::OTHER => {}
        }
    }
    fn pressed_shift(&self) -> bool {
        self.last_state[VK_SHIFT.0 as usize] & 0x80 != 0
    }
    fn pressed_ctrl(&self) -> bool {
        self.last_state[VK_CONTROL.0 as usize] & 0x80 != 0
    }
    fn latched_capital(&self) -> bool {
        self.last_state[VK_CAPITAL.0 as usize] & 0x01 != 0
    }
    fn get_one_sym(&self, _virtkey: u32) -> String {
        "TEST".to_string()
    }
    fn get_keystroke(&self, virtkey: u32) -> Keystroke {
        let keysym = self.get_one_sym(virtkey);
        Keystroke::new(virtkey, keysym)
    }
}

pub fn run_sender(timeout: Arc<RwLock<u32>>, apphandle: AppHandle, label: String, event: String) {
    init_channel();
    let hook = std::thread::spawn(|| {
        keyboad_hook();
    });
    let recv = std::thread::spawn(move || {
        let mut keystrokes = Vec::<Keystroke>::new();
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
                        let keystroke = keyboard.get_keystroke(recv.virtkey);
                        keystrokes.push(keystroke);
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
