#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{generate_context, WindowBuilder, WindowUrl};

mod cmd;

fn main() {
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
    .run(ctx)
    .expect("error while running tauri app");
}
