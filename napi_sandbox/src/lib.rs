use std::time::Duration;

use neon::main;
use neon::prelude::*;

fn foo(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let val = cx.argument::<JsNumber>(0)?;
  // let val = cx.boxed(val);

  cx.execute_async_local(|_cx| async {
    println!("hey");
  });

  println!("hi");
  Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("foo", foo);
  Ok(())
}
