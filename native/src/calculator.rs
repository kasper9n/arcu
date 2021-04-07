use cpc::{eval, units::Unit};

pub fn main(query: &str) -> String {
  println!("X query {:?}", query);
  match eval(&query, true, Unit::Celsius, false) {
    Ok(number) => {
      let value = number.value.to_string();
      let unit = match number.unit {
        Unit::NoUnit => "".to_owned(),
        x => format!(" {:?}", x),
      };
      return "= ".to_owned()+&value+&unit
    },
    Err(e) => {
      println!("X err {:?}", e);
      return "".to_string()
    }
  }
}
