use cpc::{eval, units::Unit};
use tauri::{command, AppHandle, Window};

#[command]
pub fn query(value: String) -> String {
  println!("X query {:?}", value);
  println!("X query len {:?}", value.len());
  println!("X query charlen {:?}", value.chars().count());
  match eval(&value, true, Unit::Celsius, false) {
    Ok(number) => {
      let value = number.value.to_string();
      let unit = match number.unit {
        Unit::NoUnit => "".to_owned(),
        x => format!(" {:?}", x),
      };
      return "= ".to_owned() + &value + &unit;
    }
    Err(e) => {
      println!("X err {:?}", e);
      return "".to_string();
    }
  }
}

#[command]
pub fn hide(app: AppHandle, win: Window) -> tauri::Result<()> {
  if cfg!(target_os = "macos") {
    app.hide()?;
  } else {
    win.hide()?;
  }
  Ok(())
}

#[command]
pub fn show(win: Window) -> tauri::Result<()> {
  win.show()?;
  win.set_focus()?;
  Ok(())
}

#[command]
pub fn toggle(app: AppHandle, win: Window) -> tauri::Result<()> {
  if win.is_visible()? {
    hide(app, win)?;
  } else {
    show(win)?;
  }
  Ok(())
}
