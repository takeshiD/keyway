// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keysender;
mod keyway;
use keysender::run_sender;

use std::sync::{Arc, RwLock};
use log::debug;
use std::env;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};


#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct BehaviorParameter {
    timeout: u32,
    mousevisible: bool,
    modvisible: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct TypographyParameter {
    fontsize: u32,
    fontfamily: String,
    textcolor: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct WindowAppearanceParameter {
    backgroundcolor: String,
    transparantetoggle: bool,
    backgroundopacity: f32,
}

fn main() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();
    debug!("Starting keyway");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .setup(|app| {
            let config_window = app.get_window("ConfigWindow").unwrap();
            let key_window = app.get_window("KeyWindow").unwrap();
            let timeout = Arc::new(RwLock::new(500u32));
            // ************** Behavior *****************
            {
                let timeout_ = timeout.clone();
                config_window.listen("on-change-behavior", move |event| {
                    debug!(
                        "ConfigWindow onChangeBehavior: {:?}",
                        event.payload()
                    );
                    let behavior_param =
                        serde_json::from_str::<BehaviorParameter>(event.payload().unwrap())
                            .unwrap();
                    *timeout_.write().unwrap() = behavior_param.timeout;
                });
                debug!("Setup on-change-behavior");
            }

            // ************** Typography *****************
            {
                let key_window_ = key_window.clone();
                config_window.listen("on-change-typography", move |event| {
                   debug!(
                        "ConfigWindow onChangeTypography: {:?}",
                        event.payload()
                    );
                    key_window_
                        .emit(
                            "on-change-typography",
                            serde_json::from_str::<TypographyParameter>(event.payload().unwrap())
                                .unwrap(),
                        )
                        .unwrap();
                });
                debug!("Setup on-change-typography");
            }

            // ************** WindowAppearance *****************
            {
                let key_window_ = key_window.clone();
                config_window.listen("on-change-windowappearance", move |event| {
                    debug!(
                        "ConfigWindow onChangeWindowAppearance: {:?}",
                        event.payload()
                    );
                    key_window_
                        .emit(
                            "on-change-windowappearance",
                            serde_json::from_str::<WindowAppearanceParameter>(
                                event.payload().unwrap(),
                            )
                            .unwrap(),
                        )
                        .unwrap();
                });
                debug!("Setup on-change-windowappearance");
            }

            // ************** KeySender *****************
            let apphandle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                run_sender(
                    timeout.clone(),
                    apphandle,
                    "KeyWindow".to_string(),
                    "keyevent".to_string(),
                );
            });
            debug!("Starting keysender");
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "open" => {
                    let window = app.get_window("ConfigWindow").unwrap();
                    match window.show() {
                        Ok(()) => (),
                        Err(e) => eprintln!("{e}"),
                    }
                }
                "hide" => {
                    let window = app.get_window("ConfigWindow").unwrap();
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
            } => {}
            SystemTrayEvent::RightClick {
                tray_id,
                position,
                size,
                ..
            } => {}
            SystemTrayEvent::DoubleClick {
                tray_id,
                position,
                size,
                ..
            } => {}
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
