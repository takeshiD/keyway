use crate::keyway::Keystroke;

use log::debug;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[derive(Debug)]
enum KeyAction {
    KEYUP,
    KEYDOWN,
    OTHER,
}

#[derive(Debug)]
struct Key {
    scancode: u32,
    keycode: u32,
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
        keycode: vkcode,
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
                    // debug!("Keydown {:?}", key);
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
                    // debug!("Keyup {:?}", key);
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
                    // warn!("Other {:?}", key);
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


pub fn run_sender(timeout: Arc<RwLock<u32>>, apphandle: AppHandle, label: String, event: String) {
    init_channel();
    let hook = std::thread::spawn(|| {
        keyboad_hook();
    });
    let recv = std::thread::spawn(move || {
        let mut keystrokes = Vec::<Keystroke>::new();
        let mut timestamp = Instant::now();
        '_keysend_loop: loop {
            let timeout = Duration::from_millis(*timeout.read().unwrap() as u64);
            match RX
                .get()
                .expect("Failed get")
                .lock()
                .expect("Failed read")
                .recv_timeout(Duration::from_millis(50))
            {
                Ok(recv) => {
                    debug!("Stroke: {:?}", recv);
                    match recv.keyaction {
                        KeyAction::KEYDOWN=> {
                            timestamp = Instant::now();
                            let keystroke = Keystroke::new(
                                recv.scancode, 
                                recv.keycode,
                                "Test".to_string());
                            // keystrokes.push(Keystroke::new(recv.scancode, "Test".to_string()));
                        }
                        KeyAction::KEYUP=> {
                            timestamp = Instant::now();
                            // keystrokes.push(Keystroke::new(recv.scancode, "Test".to_string()));
                        }
                        KeyAction::OTHER=> {}
                    }
                }
                Err(_err) => {
                }
            }
            let now = Instant::now();
            if !keystrokes.is_empty() && (now - timestamp > timeout) {
                keystrokes.clear();
            }
            if !keystrokes.is_empty() {
                debug!("Keystrokes: {:?}", keystrokes);
            }
            apphandle
                .emit_to(&label, &event, keystrokes.clone())
                .unwrap();
        }
    });
    recv.join().expect("Failed join recv");
    hook.join().expect("Failed join hook");
}
