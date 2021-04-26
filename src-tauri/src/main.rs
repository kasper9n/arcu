#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{generate_context, Attributes, WindowUrl};

mod cmd;

fn main() {
  let mut ctx = generate_context!();
  ctx.config.tauri.windows = Vec::new();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmd::query])
    .create_window("main".into(), WindowUrl::default(), |webview| {
      webview
        .title("Arcu")
        .resizable(true)
        .transparent(false)
        .decorations(false)
        .always_on_top(false)
        .width(800.0)
        .height(600.0)
        .min_width(300.0)
        .min_height(150.0)
        .fullscreen(false)
    })
    .run(ctx)
    .expect("error while running tauri app");
}
