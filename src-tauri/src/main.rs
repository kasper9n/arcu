#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::api::shell;
use tauri::{
  AboutMetadata, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, SystemTray,
  SystemTrayEvent, WindowBuilder, WindowEvent, WindowUrl,
};

mod cmd;

fn main() {
  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::query,
      cmd::hide,
      cmd::show,
      cmd::toggle
    ])
    .setup(|app| {
      // hide from dock (also hides menu bar)
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);
      Ok(())
    })
    .setup(|app| {
      let win = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("Arcu")
        // .decorations(false)
        .inner_size(800.0, 600.0)
        .min_inner_size(300.0, 150.0)
        .skip_taskbar(true);

      #[cfg(target_os = "macos")]
      let win = win
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .hidden_title(true);

      let _win = win.build().expect("Unable to create window");

      #[cfg(target_os = "macos")]
      {
        use cocoa::appkit::NSWindow;
        let nsw = _win.ns_window().unwrap() as cocoa::base::id;
        unsafe {
          // set window to always be dark mode
          use cocoa::appkit::NSAppearanceNameVibrantDark;
          use objc::*;
          let appearance: cocoa::base::id = msg_send![
            class!(NSAppearance),
            appearanceNamed: NSAppearanceNameVibrantDark
          ];
          let () = msg_send![nsw, setAppearance: appearance];

          // set window background color
          let bg_color = cocoa::appkit::NSColor::colorWithRed_green_blue_alpha_(
            cocoa::base::nil,
            0.0 / 255.0,
            0.0 / 255.0,
            0.0 / 255.0,
            1.0,
          );
          nsw.setBackgroundColor_(bg_color);
        }
      }

      Ok(())
    })
    .on_window_event(|event| match event.event() {
      WindowEvent::Focused(focused) => {
        if !focused {
          event.window().hide().unwrap();
        }
      }
      _ => {}
    })
    .system_tray(SystemTray::new())
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
        let window = app.get_window("main").unwrap();
        let is_visible = window.is_visible().unwrap();
        if is_visible {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
          window.set_focus().unwrap();
        }
      }
      _ => {}
    })
    .menu(Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuEntry::Submenu(Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([
          MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default()).into(),
          MenuItem::Separator.into(),
          MenuItem::Services.into(),
          MenuItem::Separator.into(),
          MenuItem::Hide.into(),
          MenuItem::HideOthers.into(),
          MenuItem::ShowAll.into(),
          MenuItem::Separator.into(),
          MenuItem::Quit.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
          MenuItem::Undo.into(),
          MenuItem::Redo.into(),
          MenuItem::Separator.into(),
          MenuItem::Cut.into(),
          MenuItem::Copy.into(),
          MenuItem::Paste.into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          MenuItem::SelectAll.into(),
        ]),
      )),
      // You should always have a Help menu on macOS because it will automatically
      // show a menu search field
      MenuEntry::Submenu(Submenu::new(
        "Help",
        Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
      )),
    ]))
    .on_menu_event(|event| {
      let event_name = event.menu_item_id();
      match event_name {
        "Learn More" => {
          let url = "https://kasper.space";
          shell::open(&event.window().shell_scope(), url.to_string(), None).unwrap();
        }
        _ => {}
      }
    })
    .run(ctx)
    .expect("error while running tauri app");
}
