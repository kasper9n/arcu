extern crate neon;
use neon::prelude::*;
extern crate rustbreak;
use rustbreak::{MemoryDatabase, deser::Yaml};
use std::collections::HashMap;
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

mod calculator;

type DB = MemoryDatabase::<HashMap<u32, String>, Yaml>;

pub struct Arcu {
    #[allow(dead_code)]
    db: DB,
    user_data_dir: String,
}

#[derive(Deserialize, Debug)]
struct Query {
  full_value: String,
}

#[derive(Serialize, Debug)]
pub struct Result {
  confidence: i8, // confidence 0-10
  minitext: String,
}

declare_types! {
  pub class JsArcu for Arcu {
    init(mut cx) {
      let db = DB::memory(HashMap::new()).expect("Error creating Rustbreak db");
      db.write(|db| {
        db.insert(0, String::from("world"));
        db.insert(1, String::from("bar"));
      }).expect("Error writing to db");

      let user_data_dir: Handle<JsString> = cx.argument::<JsString>(0)?;
      Ok(Arcu {
        db: db,
        user_data_dir: user_data_dir.value(),
      })
    }
    method query(mut cx) {
      let this = cx.this();
      let _x = {
        let guard = cx.lock();
        let arcu = this.borrow(&guard);
        let mut x: String = "".to_owned();
        arcu.db.read(|db| {
          x = db.get(&0).expect("Error reading").to_owned();
          // x = db.get(&0).expect("3").clone;
          println!("Hello: {:?}", db.get(&0));
        }).expect("Error reading Rustbreak db");
        x
      };

      let arg0 = cx.argument::<JsValue>(0)?;
      let query: Query = neon_serde::from_value(&mut cx, arg0)?;
      println!("query: {:?}", query);
        
      let mut results: Vec<Result> = vec![];
      results.extend(calculator::main(&query.full_value));
      
      // convert results to JsArray
      let js_array = JsArray::new(&mut cx, results.len() as u32);
      for (i, result) in results.iter().enumerate() {
        let js_result = JsObject::new(&mut cx);

        let js_confidence = cx.number(result.confidence as f64);
        let js_minitext = cx.string(&result.minitext);

        js_result.set(&mut cx, "minitext", js_minitext).unwrap();
        js_result.set(&mut cx, "confidence", js_confidence).unwrap();

        js_array.set(&mut cx, i as u32, js_result).unwrap();
      }
      
      Ok(js_array.as_value(&mut cx))
      
      // Ok(cx.string(&_x).upcast())
    }
    method show_user_data_dir(mut cx) {
      let this = cx.this();
      let name = {
        let guard = cx.lock();
        let user = this.borrow(&guard);
        user.user_data_dir.clone()
      };
      println!("{}", &name);
      Ok(cx.undefined().upcast())
    }
    // method get(mut cx) {
    //   let attr: String = cx.argument::<JsString>(0)?.value();
    //   let this = cx.this();
    //   match &attr[..] {
    //     "first_name" => {
    //       let first_name = {
    //         let guard = cx.lock();
    //         let user = this.borrow(&guard);
    //         user.first_name.clone()
    //       };
    //       Ok(cx.string(&first_name).upcast())
    //     },
    //     _ => cx.throw_type_error("property does not exist")
    //   }
    // }
    method panic(_) {
      panic!("User.prototype.panic")
    }
  }
}
register_module!(mut m, {
    m.export_class::<JsArcu>("Arcu")?;
    Ok(())
});







// use neon::prelude::*;

// extern crate neon_serde;
// extern crate neon;
// #[macro_use]
// extern crate serde_derive;
// mod calculator;
// mod currency_exchange_rate;

// #[derive(Deserialize, Debug)]
// struct Query {
//   full_value: String,
// }

// #[derive(Serialize, Debug)]
// pub struct Result {
//   confidence: i8, // confidence 0-10
//   minitext: String,
// }

// fn query(mut cx: FunctionContext) -> JsResult<JsArray> {

//   let arg0 = cx.argument::<JsValue>(0)?;
//   let query: Query = neon_serde::from_value(&mut cx, arg0)?;
//   println!("query: {:?}", query);
  
//   let mut results: Vec<Result> = vec![];
//   results.extend(calculator::main(&query.full_value));
//   results.extend(currency_exchange_rate::main(&query.full_value));
  
//   // convert results to JsArray
//   let js_array = JsArray::new(&mut cx, results.len() as u32);
//   for (i, result) in results.iter().enumerate() {
//     let js_result = JsObject::new(&mut cx);

//     let js_confidence = cx.number(result.confidence as f64);
//     let js_minitext = cx.string(&result.minitext);

//     js_result.set(&mut cx, "minitext", js_minitext).unwrap();
//     js_result.set(&mut cx, "confidence", js_confidence).unwrap();

//     js_array.set(&mut cx, i as u32, js_result).unwrap();
//   }
  
//   Ok(js_array)
// }

// register_module!(mut cx, {
//   cx.export_function("init", init)?;
//   cx.export_function("query", query)?;
//   Ok(())
// });
