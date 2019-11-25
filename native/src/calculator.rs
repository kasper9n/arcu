pub fn main(query: &str) {
  use rsc::computer::Computer;

  let mut computer = Computer::<f64>::default();

  // assert!(computer.eval(&query).unwrap() == 5.3);
  let eval_result = computer.eval(&query);
  if eval_result.is_err() {
    println!("Nothing");
  } else {
    println!("{}", eval_result.unwrap());
  }
}
