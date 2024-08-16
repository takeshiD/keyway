// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  let app = tauri::Builder::default()
  .build(tauri::generate_context!())
  .expect("Error while building tauri application");

  let config_window = tauri::WindowBuilder::new(
    &app,
    "ConfigWindow",
    tauri::WindowUrl::App("config.html".into())
  ).build().expect("Failed to build Config Window");
  let key_window = tauri::WindowBuilder::new(
    &app,
    "KeyWindow",
    tauri::WindowUrl::App("key.html".into())
  ).build().expect("Failed to build Key Window");
  app.run(|_,_| {});
}
