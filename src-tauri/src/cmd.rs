use cpc::{eval, units::Unit};
use tauri::{command, AppHandle};

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
pub fn hide_app(app: AppHandle) -> bool {
  if cfg!(target_os = "macos") {
    app.hide().is_err()
  } else {
    false
  }
}
