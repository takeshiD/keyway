use crate::keyway::Keystroke;

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::Duration;
use tauri::{AppHandle, Manager};

use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

static RX: OnceLock<Mutex<Receiver<u32>>> = OnceLock::new();
static TX: OnceLock<Mutex<Sender<u32>>> = OnceLock::new();

fn init_channel() {
    let (tx, rx) = mpsc::channel();
    RX.set(Mutex::new(rx)).unwrap();
    TX.set(Mutex::new(tx)).unwrap();
}

unsafe fn extract_rawkey(lparam: &LPARAM) -> &u32 {
    &*(lparam.0 as *const u32) as &u32
}

extern "system" fn keyboard_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if ncode as u32 == HC_ACTION {
            match wparam.0 as u32 {
                WM_KEYDOWN => {
                    let tx = TX
                        .get()
                        .expect("Failed to get")
                        .lock()
                        .expect("Failed to lock")
                        .clone();
                    tx.send(*extract_rawkey(&lparam)).expect("Failed send");
                    println!("Send Keydown {}", *extract_rawkey(&lparam));
                }
                WM_KEYUP => {
                    let tx = TX
                        .get()
                        .expect("Failed to get")
                        .lock()
                        .expect("Failed to lock")
                        .clone();
                    tx.send(*extract_rawkey(&lparam)).expect("Falied send");
                    println!("Send Keyup {}", *extract_rawkey(&lparam));
                }
                _ => {
                    let tx = TX
                        .get()
                        .expect("Failed to get")
                        .lock()
                        .expect("Failed to lock")
                        .clone();
                    tx.send(*extract_rawkey(&lparam)).expect("Failed send");
                    println!("Send Other {}", *extract_rawkey(&lparam));
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
        keystrokes.push(Keystroke::new(10u32, "TestSymbol".to_string()));
        '_keysend_loop: loop {
            match RX
                .get()
                .expect("Failed get")
                .lock()
                .expect("Failed read")
                .recv_timeout(Duration::from_millis(*timeout.read().unwrap() as u64))
            {
                Ok(recv) => {
                    if cfg!(debug_assertions) {
                        println!("Keyreceived: {}", recv);
                    }
                    keystrokes.push(Keystroke::new(recv, "TestKey".to_string()));
                    apphandle
                        .emit_to(&label, &event, keystrokes.clone())
                        .unwrap();
                }
                Err(_err) => {
                    if keystrokes.is_empty() {
                        keystrokes.clear();
                    }
                }
            }
        }
    });
    recv.join().expect("Failed join recv");
    hook.join().expect("Failed join hook");
}
