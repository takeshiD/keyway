// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("ConfigureWindow").unwrap();
                    match window.hide() {
                        Ok(()) => (),
                        Err(e) => eprintln!("{e}"),
                    }
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick {
                tray_id,
                position,
                size,
                ..
            } => {
                let window = app.get_window("ConfigureWindow").unwrap();
                match window.show() {
                    Ok(()) => (),
                    Err(e) => eprintln!("{e}"),
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("Error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
