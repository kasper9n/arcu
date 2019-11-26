use neon::prelude::*;

mod calculator;

#[derive(Debug)]
pub struct Result {
  confidence: i8,
  minitext: String,
}

fn main(mut cx: FunctionContext) -> JsResult<JsArray> {
  // get the object argument
  let js_object_handle: Handle<JsObject> = cx.argument(0)?;
  let js_object = js_object_handle
    .downcast::<JsObject>()
    .unwrap_or(JsObject::new(&mut cx));
  // get query
  let rust_string = js_object
    .get(&mut cx, "query")?
    .downcast::<JsString>()
    .unwrap_or(cx.string(""));
  let query = rust_string.value();
  
  let mut results = vec![];
  results.extend(calculator::main(&query));
  
  
  // convert results to JsArray
  // let vec: Vec<String> = Vec::with_capacity(100);
  let js_array = JsArray::new(&mut cx, results.len() as u32);

  // println!("{:?}", results);
  // for result in results {
  //   println!("{:?}", result);
  // }
  
  for (i, obj) in results.iter().enumerate() {
    let js_obj = JsObject::new(&mut cx);

    let js_confidence = cx.number(obj.confidence as f64);
    let js_minitext = cx.string(&obj.minitext);

    js_obj.set(&mut cx, "minitext", js_minitext).unwrap();
    js_obj.set(&mut cx, "confidence", js_confidence).unwrap();

    js_array.set(&mut cx, i as u32, js_obj).unwrap();
  }
  
  Ok(js_array)
}

register_module!(mut cx, {
  cx.export_function("main", main)
});
