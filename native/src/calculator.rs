use crate::Result;

pub fn main(query: &str) -> Vec<Result> {
  use rsc::computer::Computer;

  let mut computer = Computer::<f64>::default();

  // assert!(computer.eval(&query).unwrap() == 5.3);
  let eval_result = computer.eval(&query);
  if eval_result.is_err() {
    println!("Nothing");
    let vec: Vec<Result> = vec![
      Result {
        confidence: 10,
        minitext: "".to_owned(),
      }
    ];
    return vec;
  } else {
    let unwrapped_eval_result = eval_result.unwrap();
    println!("{}", &unwrapped_eval_result);

    let minitext = "= ".to_owned()+&unwrapped_eval_result.to_string();

    let vec: Vec<Result> = vec![
      Result {
        confidence: 10,
        minitext: minitext,
      }
    ];

    return vec;
  }
}
