#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::{
  api, generate_context, CustomMenuItem, Menu, MenuItem, SystemTrayMenuItem, WindowBuilder,
  WindowUrl,
};

mod cmd;

fn main() {
  let show = CustomMenuItem::new("show".to_string(), "Show");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu_items = vec![
    SystemTrayMenuItem::Custom(show),
    SystemTrayMenuItem::Custom(hide),
  ];

  let menu = vec![
    // on macOS first menu is always app name
    Menu::new(
      "Arcu",
      vec![
        MenuItem::About("Arcu".to_string()),
        MenuItem::Separator,
        MenuItem::Services,
        MenuItem::Separator,
        MenuItem::Hide,
        MenuItem::HideOthers,
        MenuItem::ShowAll,
        MenuItem::Separator,
        MenuItem::Quit,
      ],
    ),
    Menu::new(
      "Edit",
      vec![
        MenuItem::Undo,
        MenuItem::Redo,
        MenuItem::Separator,
        MenuItem::Cut,
        MenuItem::Copy,
        MenuItem::Paste,
        MenuItem::Separator,
        MenuItem::SelectAll,
      ],
    ),
    Menu::new(
      "Help",
      vec![MenuItem::Custom(CustomMenuItem::new(
        "learn-more".into(),
        "Learn More",
      ))],
    ),
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
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id().as_str() {
      "learn-more" => {
        api::shell::open("https://github.com/probablykasper/arcu".to_string(), None).unwrap();
      }
      _ => {}
    })
    .run(ctx)
    .expect("error while running tauri app");
}
