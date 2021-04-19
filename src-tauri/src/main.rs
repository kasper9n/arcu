#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{generate_context, WindowUrl};

mod cmd;

fn main() {
  let mut ctx = generate_context!();
  ctx.config.tauri.windows = Vec::new();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmd::query])
    .create_window("main".to_owned(), WindowUrl::default(), |mut webview| {
      webview.resizable = true;
      webview.title = "Arcu".to_string();
      // webview.transparent = false;
      webview.decorations = false;
      webview.always_on_top = false;
      webview.width = 800.0;
      webview.height = 600.0;
      webview.min_width = Some(300.0);
      webview.min_height = Some(150.0);
      webview.fullscreen = false;
      webview.skip_taskbar = false;
      return webview;
    })
    .run(ctx)
    .expect("error while running tauri app");
}
