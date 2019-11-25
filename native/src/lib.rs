use neon::prelude::*;

mod calculator;

fn main(mut cx: FunctionContext) -> JsResult<JsString> {
  let js_object_handle: Handle<JsObject> = cx.argument(0)?;
  let js_object = js_object_handle
    .downcast::<JsObject>()
    .unwrap_or(JsObject::new(&mut cx));
  let rust_string = js_object
    .get(&mut cx, "query")?
    .downcast::<JsString>()
    .unwrap_or(cx.string(""));
  // query.cast(f: str);
  calculator::main(&rust_string.value());
  Ok(cx.string("d"))
}

register_module!(mut cx, {
  cx.export_function("main", main)
});
