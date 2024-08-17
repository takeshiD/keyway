// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("Error while building tauri application");
    app.run(|_, _| {});
}
