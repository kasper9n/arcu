use cpc::{eval, units::Unit};
use tauri::command;

#[command]
pub fn query(value: String) -> String {
  println!("X query {:?}", value);
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
