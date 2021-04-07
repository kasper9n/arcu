use napi::{CallContext, JsObject, JsString, Result as NResult};
use napi_derive::{js_function, module_exports};

mod calculator;

pub fn arg_to_string(ctx: &CallContext, arg: usize) -> NResult<String> {
  let js_string: JsString = ctx.get(arg)?;
  let js_utf8_string = js_string.into_utf8()?;
  let rust_string = js_utf8_string.as_str()?.to_string();
  return Ok(rust_string);
}

#[js_function(1)]
fn query(ctx: CallContext) -> NResult<JsString> {
  let query_str = arg_to_string(&ctx, 0)?;
  let minitext = calculator::main(&query_str);
  return ctx.env.create_string(&minitext)
}

#[module_exports]
fn init_data_instance(mut exports: JsObject) -> NResult<()> {
  exports.create_named_method("query", query)?;
  Ok(())
}
