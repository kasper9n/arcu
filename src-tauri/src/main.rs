#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::{generate_context, CustomMenuItem, SystemTrayMenuItem, WindowBuilder, WindowUrl};

mod cmd;

fn main() {
  let show = CustomMenuItem::new("show".to_string(), "Show");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu_items = vec![
    SystemTrayMenuItem::Custom(show),
    SystemTrayMenuItem::Custom(hide),
  ];

  let ctx = generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmd::query])
    .create_window("main".into(), WindowUrl::default(), |win, webview| {
      let win = win
        .title("Arcu")
        .resizable(true)
        .transparent(false)
        .decorations(false)
        .always_on_top(false)
        .inner_size(800.0, 600.0)
        .min_inner_size(300.0, 150.0)
        .fullscreen(false);
      return (win, webview);
    })
    .system_tray(tray_menu_items)
    .on_system_tray_event(|app, event| match event.menu_item_id().as_str() {
      "show" => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      "hide" => {
        let window = app.get_window("main").unwrap();
        window.hide().unwrap();
      }
      e => println!("Unhandled tray event {}", e),
    })
    .run(ctx)
    .expect("error while running tauri app");
}
